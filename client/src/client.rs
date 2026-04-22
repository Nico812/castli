use std::sync::Arc;
use tokio::{
    io::BufReader,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        Mutex, mpsc,
        watch::{Receiver, Sender},
    },
    time,
};

use crate::{
    game_state::GameState,
    tui::{self, Tui},
};
use common::{
    C2S, C2S4L, L2S4C, S2C,
    r#const::{IP_LOCAL, ONLINE},
    stream,
};

pub struct ShutdownChannel {
    sender: Sender<bool>,
    receiver: Receiver<bool>,
}

impl ShutdownChannel {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::watch::channel(false);
        Self { sender, receiver }
    }
    pub fn clone(other: &Self) -> Self {
        Self {
            sender: other.sender.clone(),
            receiver: other.receiver.clone(),
        }
    }

    pub fn shutdown(&self) {
        let _ = self.sender.send(true);
    }

    pub fn is_shutdown(&self) -> bool {
        return *self.receiver.borrow();
    }
}

struct ClientConnection {
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
}

impl ClientConnection {
    async fn communicate_with_server(
        mut self,
        mut t2c_rx: mpsc::UnboundedReceiver<tui::T2C>,
        shutdown: ShutdownChannel,
        game_state: Arc<Mutex<GameState>>,
    ) {
        let mut request_tick = time::interval(time::Duration::from_millis(1000));

        loop {
            if shutdown.is_shutdown() {
                return;
            }

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
            let mut game_state = game_state.lock().await;

            match msg {
                S2C::L2S4C(L2S4C::GameObjs(objs)) => {
                    game_state.objs = objs;
                }
                S2C::L2S4C(L2S4C::Player(player)) => {
                    game_state.player = player;
                }
                S2C::L2S4C(L2S4C::Log(msg)) => {
                    game_state.add_log(msg);
                }
                _ => {}
            }
                }
                // Otherwise, run the periodic update requests
                _ = request_tick.tick() => {
                    let _ = stream::send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GiveObjs),
                    ).await;

                    let _ = stream::send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GivePlayer),
                    ).await;
                }
            }
        }
    }

    // Fetches the initial game objects and player data required to start the TUI.
    async fn fetch_initial_state(&mut self) -> Result<GameState, ()> {
        // Request map
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap)).await;
        let map = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => map,
            _ => return Err(()),
        };

        // Request game objects
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs)).await;
        let objs = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::GameObjs(objs))) => objs,
            _ => return Err(()),
        };

        // Request player data (this is a placeholder until the castle is built)
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GivePlayer)).await;
        let player = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Player(player))) => Some(player),
            Ok(S2C::L2S4C(L2S4C::CreateCastle)) => None,
            _ => return Err(()),
        };

        Ok(GameState::new(objs, player, map))
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
        let shutdown = ShutdownChannel::new();
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
        let game_state = Arc::new(Mutex::new(
            connection
                .fetch_initial_state()
                .await
                .expect("Failed to receive initial state."),
        ));

        // Set up communication channels
        let (t2c_tx, t2c_rx) = mpsc::unbounded_channel(); // TUI -> Server

        // Spawn the dedicated network task
        let communication_handle = tokio::spawn(connection.communicate_with_server(
            t2c_rx,
            ShutdownChannel::clone(&shutdown),
            Arc::clone(&game_state),
        ));

        // Create and run the TUI. The main thread will now be dedicated to the UI.
        Tui::run(t2c_tx, game_state, shutdown).await; // This blocks until the user quits the TUI.

        // Cleanup
        let _ = communication_handle.await;
        println!("\nClient shutting down. Goodbye!");
    }
}
