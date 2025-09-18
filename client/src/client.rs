//! # Client Core Logic
//!
//! This module contains the `Client` struct, which manages the client's
//! connection to the server and orchestrates the different parts of the
//! client application, such as the TUI and server communication.
use std::collections::HashMap;
use tokio::{
    io::BufReader,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
    time,
};

use crate::tui::{self, Tui};
use common::{
    C2S, C2S4L, GameObjE, L2S4C, PlayerE, S2C, TileE,
    r#const::{IP_LOCAL, ONLINE},
    stream,
};

#[derive(Debug)]
pub enum ClientErr {
    DataNotReceived,
}

// --- Client Connection ---

/// Manages the state and logic for the TCP connection to the server.
struct ClientConnection {
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
}

impl ClientConnection {
    /// Handles the ongoing communication with the server in a loop.
    async fn communicate_with_server(
        &mut self,
        s2c_tx: &mpsc::UnboundedSender<S2C>,
        t2c_rx: &mut mpsc::UnboundedReceiver<tui::T2C>,
    ) {
        let mut request_tick = time::interval(time::Duration::from_millis(1000));

        tokio::select! {
            // Check for messages from the TUI to send to the server
            Some(msg_from_tui) = t2c_rx.recv() => {
                let msg = match msg_from_tui {
                    tui::T2C::NewCastle(pos) => C2S::C2S4L(C2S4L::NewCastle(pos)),
                    tui::T2C::AttackCastle(target_id) => {
                        C2S::C2S4L(C2S4L::AttackCastle(target_id))
                    }
                };
                let _ = stream::send_msg_to_server(&mut self.writer, &msg).await;
            },
            // Check for messages from the server and redirects them to the TUI
            // TODO: the tokio select here can cause data loss, should i address this?
            Ok(msg) = stream::get_msg_from_server(&mut self.reader) =>  {
                let _ = s2c_tx.send(msg);
            }
            // Otherwise, run the periodic update requests
            _ = request_tick.tick() => {
                // Request game objects
                let _ = stream::send_msg_to_server(
                    &mut self.writer,
                    &C2S::C2S4L(C2S4L::GiveObjs),
                ).await;
                if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
                    let _ = s2c_tx.send(msg);
                }

                // Request player data
                let _ = stream::send_msg_to_server(
                    &mut self.writer,
                    &C2S::C2S4L(C2S4L::GivePlayer),
                ).await;
                if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
                    let _ = s2c_tx.send(msg);
                }
            }
        }
    }

    /// Makes the initial request to get the game map from the server.
    async fn ask_for_map(&mut self) -> Result<Vec<Vec<TileE>>, ClientErr> {
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap)).await;

        match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => Ok(map),
            _ => Err(ClientErr::DataNotReceived),
        }
    }

    /// Fetches the initial game objects and player data required to start the TUI.
    async fn fetch_initial_state(
        &mut self,
    ) -> Result<(HashMap<usize, GameObjE>, Option<PlayerE>), ClientErr> {
        // Request game objects
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs)).await;
        let game_objs = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::GameObjs(objs))) => objs,
            _ => return Err(ClientErr::DataNotReceived),
        };

        // Request player data (this is a placeholder until the castle is built)
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GivePlayer)).await;
        let player_data = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Player(data))) => Some(data),
            Ok(S2C::L2S4C(L2S4C::CreateCastle)) => None,
            _ => return Err(ClientErr::DataNotReceived),
        };

        Ok((game_objs, player_data))
    }
}

// --- Client ---

/// The main Client struct acts as the application's orchestrator.
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the main client application.
    pub async fn run(&mut self) {
        // 1. Connect to the Server
        let addr = if ONLINE { IP_LOCAL } else { IP_LOCAL };
        let stream = match TcpStream::connect(addr).await {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to connect to server: {}", e);
                return;
            }
        };
        let (reader, mut writer) = stream.into_split();

        // 2. Authenticate
        println!("Connection established. Please log in.");
        let name = Tui::login();
        let _ = stream::send_msg_to_server(&mut writer, &C2S::Login(name)).await;

        // 3. Create the connection manager
        let mut connection = ClientConnection {
            writer,
            reader: BufReader::new(reader),
        };

        // 4. Fetch all initial state required for the TUI
        println!("Fetching initial game state...");
        let map = connection
            .ask_for_map()
            .await
            .expect("Failed to receive map.");
        let (initial_objs, initial_data) = connection
            .fetch_initial_state()
            .await
            .expect("Failed to receive initial state.");
        println!("Game state received.");

        // 5. Set up communication channels
        let (s2c_tx, s2c_rx) = mpsc::unbounded_channel(); // Server -> TUI
        let (t2c_tx, mut t2c_rx) = mpsc::unbounded_channel(); // TUI -> Server

        // 6. Spawn the dedicated network task
        let communication_handle = tokio::spawn(async move {
            loop {
                connection
                    .communicate_with_server(&s2c_tx, &mut t2c_rx)
                    .await;
            }
        });

        // 7. Create and run the TUI. The main thread will now be dedicated to the UI.
        let mut tui = Tui::new(t2c_tx, s2c_rx, map, initial_objs, initial_data);
        tui.run().await; // This blocks until the user quits the TUI.

        // 8. Cleanup
        communication_handle.abort();
        println!("\nClient shutting down. Goodbye!");
    }
}
