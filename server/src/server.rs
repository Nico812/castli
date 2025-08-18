//! # Server Core Logic
//!
//! This module contains the `Server` struct, which is the heart of the server application.
//! It is responsible for listening for incoming TCP connections, managing game lobbies,
//! and routing clients to the appropriate lobby.

use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::lobby;
use common::r#const::{IP_LOCAL, MAX_LOBBIES, ONLINE};
use common::{self, stream};

/// Messages sent from the main `Server` to a `Lobby` thread.
pub enum S2L {
    IsFull(mpsc::Sender<bool>),
    NewClient(
        ClientID,
        UnboundedSender<common::L2S4C>,
        UnboundedReceiver<common::C2S4L>,
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

pub struct Server {
    threads: Arc<Mutex<[Option<thread::JoinHandle<()>>; MAX_LOBBIES]>>,
    lobby_txs: Arc<Mutex<[Option<mpsc::UnboundedSender<S2L>>; MAX_LOBBIES]>>,
}

impl Server {
    pub fn new() -> Self {
        let threads = Arc::new(Mutex::new([const { None }; MAX_LOBBIES]));
        let lobby_txs = Arc::new(Mutex::new([const { None }; MAX_LOBBIES]));

        Self { threads, lobby_txs }
    }

    /// Runs the main server loop.
    ///
    /// Listens for incoming TCP connections and spawns a new task to handle each client.
    pub async fn run(&mut self) {
        let listener;
        if ONLINE {
            //listener = TcpListener::bind(IP_4_SERVER).await.unwrap();
            listener = TcpListener::bind(IP_LOCAL).await.unwrap();
        } else {
            listener = TcpListener::bind(IP_LOCAL).await.unwrap();
        };

        let mut client_id_cnt = 0;

        while let Ok((mut stream, socket_addr)) = listener.accept().await {
            println!("Connection established, socket address:, {}", socket_addr);

            let threads_copy = self.threads.clone();
            let lobby_txs_copy = self.lobby_txs.clone();

            tokio::spawn(async move {
                // Autentication
                let authentication = match Self::wait_authentication(&mut stream).await {
                    Ok(name) => {
                        println!("[{}] Player '{}' authenticated.", socket_addr, name);
                        name
                    }
                    Err(_) => {
                        eprintln!(
                            "[{}] Authentication failed. Closing connection.",
                            socket_addr
                        );
                        let _ =
                            stream::send_msg_to_client(&mut stream, &common::S2C::ConnectionFailed)
                                .await;
                        return;
                    }
                };
                let client_id = client_id_cnt;
                client_id_cnt += client_id_cnt;
                // Send the player to the lobby
                match Self::handle_client(threads_copy, lobby_txs_copy, &authentication, client_id)
                    .await
                {
                    Ok((client_id, client_tx, mut client_rx)) => loop {
                        println!("cazzo sta succedendo");
                        match stream::get_msg_from_client(&mut stream).await {
                            Ok(msg) => match msg {
                                common::C2S::C2S4L(msg4l) => {
                                    println!("got a msg");
                                    let _ = client_tx.send(msg4l);
                                }
                                _ => {
                                    println!("ricevuto messaggio inaspettato");
                                }
                            },
                            Err(_) => {
                                eprintln!("CLIENT (ID: {}) DISCONNECTED.", client_id);
                                break;
                            }
                        }

                        if let Some(msg) = client_rx.recv().await {
                            match msg {
                                common::L2S4C::GameObjs(objs) => {
                                    let _ = stream::send_msg_to_client(
                                        &mut stream,
                                        &common::S2C::L2S4C(common::L2S4C::GameObjs(objs)),
                                    )
                                    .await;
                                }
                                common::L2S4C::Map(map) => {
                                    println!("Sending map");
                                    let _ = stream::send_msg_to_client(
                                        &mut stream,
                                        &common::S2C::L2S4C(common::L2S4C::Map(map)),
                                    )
                                    .await;
                                }
                                common::L2S4C::PlayerData(player_data) => {
                                    let _ = stream::send_msg_to_client(
                                        &mut stream,
                                        &common::S2C::L2S4C(common::L2S4C::PlayerData(player_data)),
                                    )
                                    .await;
                                }
                            };
                        }
                    },

                    Err(err) => {
                        match err {
                            ServerErr::MissingLobbyTx | ServerErr::PoisonedMutex => {
                                let _ = stream::send_msg_to_client(
                                    &mut stream,
                                    &common::S2C::ConnectionFailed,
                                )
                                .await;
                            }

                            ServerErr::ServerFull => {
                                let _ = stream::send_msg_to_client(
                                    &mut stream,
                                    &common::S2C::ServerFull,
                                )
                                .await;
                            }
                            _ => {}
                        }
                        eprintln!("\x1b[35mSERVER ERROR: {:?}\x1b[0m", err);
                    }
                };
            });
        }
    }

    /// Handles a single client connection.
    ///
    /// This function finds a suitable lobby for the client, either an existing one
    /// with space or a new one if possible. It establishes communication channels
    /// between the client and the lobby.
    async fn handle_client(
        threads: Arc<Mutex<[Option<thread::JoinHandle<()>>; MAX_LOBBIES]>>,
        lobby_txs: Arc<Mutex<[Option<mpsc::UnboundedSender<S2L>>; MAX_LOBBIES]>>,
        autentication: &String,
        client_id: ClientID,
    ) -> Result<
        (
            ClientID,
            UnboundedSender<common::C2S4L>,
            UnboundedReceiver<common::L2S4C>,
        ),
        ServerErr,
    > {
        let client_tx;
        let client_rx;

        // Checking if there's a lobby with space for the new player
        for iter in 0..MAX_LOBBIES {
            if threads.lock().unwrap()[iter].is_some() {
                let (temp_tx, mut temp_rx) = mpsc::channel(100);

                if let Ok(mut lobby_txs) = lobby_txs.lock() {
                    let lobby_tx = match lobby_txs[iter].as_mut() {
                        None => return Err(ServerErr::MissingLobbyTx),
                        Some(lobby_tx) => lobby_tx,
                    };

                    let _ = lobby_tx.send(S2L::IsFull(temp_tx));
                }

                if temp_rx
                    .recv()
                    .await
                    .is_some_and(|response| response == false)
                {
                    if let Ok(mut lobby_txs) = lobby_txs.lock() {
                        let lobby_tx = match lobby_txs[iter].as_mut() {
                            None => return Err(ServerErr::MissingLobbyTx),
                            Some(lobby_tx) => lobby_tx,
                        };

                        let (tx1, rx1) = mpsc::unbounded_channel();
                        let (tx2, rx2) = mpsc::unbounded_channel();
                        client_rx = rx1;
                        client_tx = tx2;

                        let _ = lobby_tx.send(S2L::NewClient(client_id, tx1, rx2));
                        println!("Found an existing lobby with space for the client");
                        return Ok((client_id, client_tx, client_rx));
                    }
                }
            }
        }
        // Checking if there's a lobby uninitialized
        let mut threads = threads.lock().unwrap();
        for iter in 0..MAX_LOBBIES {
            if threads[iter].is_none() {
                {
                    let (client_tx1, client_rx1) = mpsc::unbounded_channel();
                    let (client_tx2, client_rx2) = mpsc::unbounded_channel();
                    let (lobby_tx, lobby_rx) = mpsc::unbounded_channel();

                    let mut lobby = lobby::Lobby::new();
                    let thread = std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();
                        let _ = rt.block_on(async move { lobby.run(lobby_rx).await });
                    });

                    let _ = lobby_tx.send(S2L::NewClient(client_id, client_tx1, client_rx2));

                    threads[iter] = Some(thread);
                    match lobby_txs.lock() {
                        Err(_) => {
                            return Err(ServerErr::PoisonedMutex);
                        }
                        Ok(mut lobby_txs) => {
                            lobby_txs[iter] = Some(lobby_tx);
                            println!("Created a new lobby with space for the client");
                            return Ok((client_id, client_tx2, client_rx1));
                        }
                    }
                }
            }
        }
        Err(ServerErr::ServerFull)
    }

    async fn wait_authentication(stream: &mut TcpStream) -> Result<String, ServerErr> {
        let mut authentication = Err(ServerErr::AuthFailed);
        tokio::select! {
            msg = stream::get_msg_from_client(stream) => {
                if let Ok(common::C2S::Login(name)) = msg {
                    authentication = Ok(name);
                }
            },
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(10000)) => {
            }
        };
        authentication
    }
}
