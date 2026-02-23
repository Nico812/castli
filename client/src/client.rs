//! # Client Core Logic

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
    C2S, C2S4L, L2S4C, S2C,
    r#const::{IP_LOCAL, ONLINE},
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE},
    stream,
};

#[derive(Debug)]
pub enum ClientErr {
    DataNotReceived,
}

struct ClientConnection {
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
}

impl ClientConnection {
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
                    tui::T2C::AttackCastle(target_id, unit_group_e) => {
                        C2S::C2S4L(C2S4L::AttackCastle(target_id, unit_group_e))
                    }
                    tui::T2C::SendUnits(target_pos, unit_group_e) => {
                        C2S::C2S4L(C2S4L::SendUnits(target_pos, unit_group_e))
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

    // Makes the initial request to get the game map from the server.
    async fn ask_for_map(&mut self) -> Result<Vec<Vec<TileE>>, ClientErr> {
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap)).await;

        match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => Ok(map),
            _ => Err(ClientErr::DataNotReceived),
        }
    }

    // Fetches the initial game objects and player data required to start the TUI.
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

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the main client application.
    pub async fn run(&mut self) {
        // Connect to the Server
        let addr = if ONLINE { IP_LOCAL } else { IP_LOCAL };
        let stream = match TcpStream::connect(addr).await {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to connect to server: {}", e);
                return;
            }
        };

        let (reader, mut writer) = stream.into_split();

        // Authentication
        println!("Connection established. Please log in.");
        let name = Tui::login();
        let _ = stream::send_msg_to_server(&mut writer, &C2S::Login(name)).await;

        // Fetch initial state required for the TUI
        let mut connection = ClientConnection {
            writer,
            reader: BufReader::new(reader),
        };

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

        // Set up communication channels
        let (s2c_tx, s2c_rx) = mpsc::unbounded_channel(); // Server -> TUI
        let (t2c_tx, mut t2c_rx) = mpsc::unbounded_channel(); // TUI -> Server

        // Spawn the dedicated network task
        let communication_handle = tokio::spawn(async move {
            loop {
                connection
                    .communicate_with_server(&s2c_tx, &mut t2c_rx)
                    .await;
            }
        });

        // Create and run the TUI. The main thread will now be dedicated to the UI.
        let mut tui = Tui::new(t2c_tx, s2c_rx, initial_objs, initial_data);
        tui.run(map).await; // This blocks until the user quits the TUI.

        // Cleanup
        communication_handle.abort();
        println!("\nClient shutting down. Goodbye!");
    }
}
