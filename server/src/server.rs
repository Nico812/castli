use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};

use bincode::{config, serde::decode_from_slice, serde::encode_to_vec};

use crate::{r#const::SERVER_TICK, lobby::Lobby, thread_pool::ThreadPool};
use common::{
    r#const::{IP_LOCAL, MAX_LOBBIES},
    packets::{C2S, C2S4L, L2S4C, S2C},
    stream::{MAX_FRAME_BYTES, StreamErr},
};

pub enum S2L {
    IsFull(Sender<bool>),
    NewClient(Client, Sender<L2S4C>, Receiver<C2S4L>),
    Disconnection(ClientId),
    Shutdown,
}

#[derive(Debug)]
pub enum ServerErr {
    LobbyFull,
    AuthFailed,
}

pub type ClientId = usize;
pub type ConnId = usize;

#[derive(Clone)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub lobby: Option<usize>,
}

impl Client {
    pub fn new(id: ClientId, name: String) -> Self {
        Self {
            id,
            name,
            lobby: None,
        }
    }
}

struct Connection {
    stream: TcpStream,
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
    lobby_link: Option<(Sender<C2S4L>, Receiver<L2S4C>)>,
    client: Option<Client>,
    id: ConnId,
}

impl Connection {
    pub fn try_get_msg(&mut self) -> Result<Option<C2S>, StreamErr> {
        let mut tmp_buf = [0u8; 4096];
        match self.stream.read(&mut tmp_buf) {
            Ok(0) => return Err(StreamErr::ConnectionEnded),
            Ok(n) => self.read_buffer.extend_from_slice(&tmp_buf[..n]),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(_) => return Err(StreamErr::ConnectionEnded),
        }

        if self.read_buffer.len() < 4 {
            return Ok(None);
        }
        let len = u32::from_le_bytes(self.read_buffer[..4].try_into().unwrap());
        if len > MAX_FRAME_BYTES {
            return Err(StreamErr::SerializationErr);
        }
        let frame_end = 4 + len as usize;
        if self.read_buffer.len() < frame_end {
            return Ok(None);
        }
        let result =
            decode_from_slice::<C2S, _>(&self.read_buffer[4..frame_end], config::standard())
                .map(|(msg, _)| Some(msg))
                .map_err(|_| StreamErr::SerializationErr);
        self.read_buffer.drain(..frame_end);
        result
    }

    pub fn queue_msg(&mut self, msg: &S2C) {
        let bytes = encode_to_vec(msg, config::standard()).expect("Serialization failed");
        let len: u32 = bytes
            .len()
            .try_into()
            .expect("Outgoing frame exceeds u32 length");
        self.write_buffer.extend_from_slice(&len.to_le_bytes());
        self.write_buffer.extend_from_slice(&bytes);
    }

    pub fn try_flush(&mut self) -> Result<(), StreamErr> {
        while !self.write_buffer.is_empty() {
            match self.stream.write(&self.write_buffer) {
                Ok(0) => return Err(StreamErr::ConnectionEnded),
                Ok(n) => {
                    self.write_buffer.drain(..n);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                Err(_) => return Err(StreamErr::ConnectionEnded),
            }
        }
        Ok(())
    }
}

pub struct Server {
    _pool: ThreadPool,
    txs: [Sender<S2L>; MAX_LOBBIES],
    conns: Vec<Connection>,
    conn_id_cnt: ConnId,
    client_id_cnt: ClientId,
}

impl Server {
    pub fn new() -> Self {
        let pool = ThreadPool::new(MAX_LOBBIES);
        let mut txs = Vec::with_capacity(MAX_LOBBIES);

        for lobby_id in 0..MAX_LOBBIES {
            let (tx, rx) = mpsc::channel();
            txs.push(tx);

            pool.execute(move || {
                let lobby = Lobby::new(lobby_id);
                lobby.run(rx);
            });
        }

        Self {
            _pool: pool,
            txs: txs.try_into().unwrap(),
            conns: Vec::new(),
            conn_id_cnt: 0,
            client_id_cnt: 0,
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind(IP_LOCAL).unwrap();
        listener.set_nonblocking(true).unwrap();
        println!("[server] Server started and listening on {}", IP_LOCAL);

        // Performance tracking
        let tick_duration = Duration::from_millis(SERVER_TICK);
        let mut loop_count = 0;
        let mut total_loop_time = Duration::new(0, 0);

        loop {
            let loop_start = Instant::now();
            let mut ended_conns = Vec::new();

            // Checks if there are new connections and handles them
            if let Ok((stream, socket_addr)) = listener.accept() {
                stream.set_nonblocking(true).unwrap();
                self.handle_connection(stream);
                println!("A weirdo connceted with socket_addr: {}", socket_addr);
            }

            // Iters trough the connections and listens to them
            for (i, ref mut conn) in self.conns.iter_mut().enumerate() {
                match conn.try_get_msg() {
                    Ok(Some(C2S::C2S4L(msg))) => {
                        let Some(ref mut lobby_link) = conn.lobby_link else {
                            continue;
                        };
                        if lobby_link.0.send(msg).is_err() {
                            println!("[server] Failed...");
                        }
                    }
                    Ok(Some(C2S::Login(user_name))) => {
                        conn.client = Some(Client::new(conn.id, user_name));
                        println!("User authenticated");
                    }
                    Ok(Some(C2S::Lobby(lobby_id))) => {
                        let Some(ref mut client) = conn.client else {
                            continue;
                        };
                        if lobby_id >= MAX_LOBBIES {
                            continue;
                        }
                        match Self::assign_client_to_lobby(lobby_id, &self.txs[lobby_id], client) {
                            Ok(link) => {
                                conn.lobby_link = Some(link);
                                println!("Client successfully assigned to lobby");
                            }
                            Err(ServerErr::LobbyFull) => {
                                conn.queue_msg(&S2C::LobbyFull);
                            }
                            _ => {}
                        }
                    }
                    Err(StreamErr::ConnectionEnded) => {
                        eprintln!("[server] CLIENT (ID: {}) DISCONNECTED.", conn.id);
                        if let Some(ref client) = conn.client
                            && let Some(lobby) = client.lobby
                        {
                            let _ = self.txs[lobby].send(S2L::Disconnection(client.id));
                        }
                        ended_conns.push(i);
                        continue;
                    }
                    Err(StreamErr::SerializationErr) => {
                        eprintln!("[server] CLIENT (ID: {}) SERIALIZATION ERR.", conn.id);
                    }
                    Ok(None) => {}
                }

                // Listen to associated lobby for updates to send to client
                let pending = if let Some(ref lobby_link) = conn.lobby_link {
                    lobby_link.1.try_recv().ok()
                } else {
                    None
                };
                if let Some(msg) = pending {
                    conn.queue_msg(&S2C::L2S4C(msg));
                }

                if let Err(StreamErr::ConnectionEnded) = conn.try_flush() {
                    eprintln!("[server] CLIENT (ID: {}) DISCONNECTED ON WRITE.", conn.id);
                    if let Some(ref client) = conn.client
                        && let Some(lobby) = client.lobby
                    {
                        let _ = self.txs[lobby].send(S2L::Disconnection(client.id));
                    }
                    if !ended_conns.contains(&i) {
                        ended_conns.push(i);
                    }
                }
            }

            // Removing disconnected clients from the active connections
            for i in ended_conns {
                self.conns.remove(i);
            }

            // Performace tracking
            let loop_time = loop_start.elapsed();
            loop_count += 1;
            total_loop_time += loop_time;

            if loop_count % 100 == 0 {
                let avg_time = total_loop_time / 100;
                println!(
                    "[server] Performance Stats - Avg loop time over last 100: {} ms",
                    avg_time.as_millis()
                );
                total_loop_time = Duration::new(0, 0);
            }

            // Small tick to prevent CPU overuse
            let elapsed = loop_start.elapsed();
            if elapsed < tick_duration {
                std::thread::sleep(tick_duration - elapsed);
            }
        }
    }

    fn handle_connection(&mut self, stream: TcpStream) {
        let conn_id = self.conn_id_cnt;
        self.conn_id_cnt += 1;

        let conn = Connection {
            stream,
            lobby_link: None,
            client: None,
            id: conn_id,
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
        };
        self.conns.push(conn);
    }

    fn assign_client_to_lobby(
        lobby_id: usize,
        lobby_tx: &Sender<S2L>,
        client: &mut Client,
    ) -> Result<(Sender<C2S4L>, Receiver<L2S4C>), ServerErr> {
        let (resp_tx, resp_rx) = mpsc::channel();
        let _ = lobby_tx.send(S2L::IsFull(resp_tx));

        if let Ok(false) = resp_rx.recv() {
            let (c2s_tx, c2s_rx) = mpsc::channel();
            let (s2c_tx, s2c_rx) = mpsc::channel();
            let _ = lobby_tx.send(S2L::NewClient(client.clone(), s2c_tx, c2s_rx));
            client.lobby = Some(lobby_id);
            return Ok((c2s_tx, s2c_rx));
        }
        Err(ServerErr::LobbyFull)
    }
}
