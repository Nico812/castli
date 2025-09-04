//! # Server Core Logic
//!
//! This module contains the `Server` struct, which is the heart of the server application.
//! It is responsible for listening for incoming TCP connections, managing game lobbies,
//! and routing clients to the appropriate lobby.

use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::{
    io::BufReader,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time,
};

use crate::lobby::Lobby;
use common::{
    r#const::{IP_LOCAL, MAX_LOBBIES},
    stream, C2S, C2S4L, L2S4C, S2C,
};

pub enum S2L {
    IsFull(mpsc::Sender<bool>),
    NewClient(
        ClientID,
        String,
        UnboundedSender<L2S4C>,
        UnboundedReceiver<C2S4L>,
    ),
    Shutdown,
}
#[derive(Debug)]
pub enum ServerErr {
    PoisonedMutex,
    MissingLobbyTx,
    ServerFull,
    AuthFailed,
}
pub type ClientID = usize;

// --- Lobby Manager ---

/// Manages the entire lifecycle of game lobbies.
struct LobbyManager {
    threads: Arc<Mutex<[Option<thread::JoinHandle<()>>; MAX_LOBBIES]>>,
    lobby_txs: Arc<Mutex<[Option<mpsc::UnboundedSender<S2L>>; MAX_LOBBIES]>>,
}

impl LobbyManager {
    fn new() -> Self {
        Self {
            threads: Arc::new(Mutex::new([const { None }; MAX_LOBBIES])),
            lobby_txs: Arc::new(Mutex::new([const { None }; MAX_LOBBIES])),
        }
    }

    /// Finds an available lobby or creates a new one for a client.
    async fn assign_client_to_lobby(
        &self,
        client_id: ClientID,
        player_name: String,
    ) -> Result<
        (
            UnboundedSender<C2S4L>,
            UnboundedReceiver<L2S4C>,
        ),
        ServerErr,
    > {
        // First, check for an existing lobby with space
        for i in 0..MAX_LOBBIES {
            let lobby_txs_guard = self.lobby_txs.lock().unwrap();
            if let Some(tx) = &lobby_txs_guard[i] {
                let (resp_tx, mut resp_rx) = mpsc::channel(1);
                let _ = tx.send(S2L::IsFull(resp_tx));
                if let Some(is_full) = resp_rx.recv().await {
                    if !is_full {
                        let (c2s_tx, c2s_rx) = mpsc::unbounded_channel();
                        let (s2c_tx, s2c_rx) = mpsc::unbounded_channel();
                        let _ = tx.send(S2L::NewClient(client_id, player_name, s2c_tx, c2s_rx));
                        return Ok((c2s_tx, s2c_rx));
                    }
                }
            }
        }

        // If no lobby has space, try to create a new one
        let mut threads_guard = self.threads.lock().unwrap();
        let mut lobby_txs_guard = self.lobby_txs.lock().unwrap();
        for i in 0..MAX_LOBBIES {
            if threads_guard[i].is_none() {
                let (lobby_tx, lobby_rx) = mpsc::unbounded_channel();
                let mut lobby = Lobby::new();

                threads_guard[i] = Some(thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async move { lobby.run(lobby_rx).await });
                }));

                lobby_txs_guard[i] = Some(lobby_tx.clone());

                let (c2s_tx, c2s_rx) = mpsc::unbounded_channel();
                let (s2c_tx, s2c_rx) = mpsc::unbounded_channel();
                let _ = lobby_tx.send(S2L::NewClient(client_id, player_name, s2c_tx, c2s_rx));
                return Ok((c2s_tx, s2c_rx));
            }
        }

        Err(ServerErr::ServerFull)
    }
}

// --- Client Handler ---

/// Manages the connection and communication for a single authenticated client.
struct ClientHandler {
    client_id: ClientID,
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    lobby_tx: UnboundedSender<C2S4L>,
    lobby_rx: UnboundedReceiver<L2S4C>,
}

impl ClientHandler {
    /// Runs the main communication loop for the client.
    async fn run(mut self) {
        loop {
            tokio::select! {
                result = stream::get_msg_from_client(&mut self.reader) => {
                    match result {
                        Ok(C2S::C2S4L(msg)) => {
                            println!("Sending msg {:?} from client {:?} to lobby", msg, self.client_id);
                            if self.lobby_tx.send(msg).is_err() {
                                println!("Failed...");
                                break;
                            }
                        },
                        Ok(_) => {},
                        Err(_) => {
                            eprintln!("CLIENT (ID: {}) DISCONNECTED.", self.client_id);
                            break;
                        }
                    }
                },
                Some(msg) = self.lobby_rx.recv() => {
                    let s2c_msg = S2C::L2S4C(msg);
                    println!("Received a msg {:?} from lobby for client {:?}", s2c_msg, self.client_id);
                    if stream::send_msg_to_client(&mut self.writer, &s2c_msg).await.is_err() {
                        println!("Failed to send msg {:?} to client {:?}", s2c_msg, self.client_id);
                        break;
                    }
                }
            }
        }
    }
}

// --- Server ---

pub struct Server {
    lobby_manager: Arc<LobbyManager>,
    next_client_id: Arc<Mutex<ClientID>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            lobby_manager: Arc::new(LobbyManager::new()),
            next_client_id: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn run(&mut self) {
        let listener = TcpListener::bind(IP_LOCAL).await.unwrap();
        println!("Server started and listening on {}", IP_LOCAL);

        while let Ok((stream, socket_addr)) = listener.accept().await {
            println!("Connection established from: {}", socket_addr);

            let lobby_manager_clone = Arc::clone(&self.lobby_manager);
            let next_client_id_clone = Arc::clone(&self.next_client_id);

            tokio::spawn(async move {
                let (reader, mut writer) = stream.into_split();
                let mut buf_reader = BufReader::new(reader);

                // 1. Authenticate
                let auth_result = Self::wait_authentication(&mut buf_reader).await;
                let user_name = match auth_result {
                    Ok(name) => {
                        println!("[{}] Player '{}' authenticated.", socket_addr, name);
                        name
                    }
                    Err(_) => {
                        eprintln!("[{}] Authentication failed.", socket_addr);
                        let _ =
                            stream::send_msg_to_client(&mut writer, &S2C::ConnectionFailed)
                                .await;
                        return;
                    }
                };

                // 2. Assign to Lobby
                let client_id = {
                    let mut id_guard = next_client_id_clone.lock().unwrap();
                    let id = *id_guard;
                    *id_guard += 1;
                    id
                };

                let lobby_channels = lobby_manager_clone
                    .assign_client_to_lobby(client_id, user_name)
                    .await;
                match lobby_channels {
                    Ok((lobby_tx, lobby_rx)) => {
                        // 3. Hand off to Client Handler
                        let handler = ClientHandler {
                            client_id,
                            reader: buf_reader,
                            writer,
                            lobby_tx,
                            lobby_rx,
                        };
                        handler.run().await;
                    }
                    Err(err) => {
                        let msg = if let ServerErr::ServerFull = err {
                            S2C::ServerFull
                        } else {
                            S2C::ConnectionFailed
                        };
                        let _ = stream::send_msg_to_client(&mut writer, &msg).await;
                        eprintln!("[{}] Failed to assign to lobby: {:?}", socket_addr, err);
                    }
                }
            });
        }
    }

    async fn wait_authentication(
        buf_reader: &mut BufReader<OwnedReadHalf>,
    ) -> Result<String, ServerErr> {
        tokio::select! {
            biased;
            msg = stream::get_msg_from_client(buf_reader) => {
                if let Ok(C2S::Login(name)) = msg {
                    Ok(name)
                } else {
                    Err(ServerErr::AuthFailed)
                }
            },
            _ = time::sleep(time::Duration::from_secs(10)) => {
                Err(ServerErr::AuthFailed)
            }
        }
    }
}