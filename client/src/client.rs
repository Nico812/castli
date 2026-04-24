use std::sync::Arc;
use tokio::{
    io::BufReader,
    net::TcpStream,
    sync::{
        Mutex, mpsc,
        watch::{Receiver, Sender},
    },
};

use crate::connection::Connection;
use crate::tui::Tui;
use common::{
    C2S,
    r#const::{IP_LOCAL, ONLINE},
    stream,
};

#[derive(Copy, Clone)]
pub enum ShutdownReason {
    Key,
    Connection,
    TermSize,
    ServerShutdown,
}

pub struct ShutdownChannel {
    sender: Sender<Option<ShutdownReason>>,
    receiver: Receiver<Option<ShutdownReason>>,
}

impl ShutdownChannel {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::watch::channel(None);
        Self { sender, receiver }
    }

    pub fn clone(other: &Self) -> Self {
        Self {
            sender: other.sender.clone(),
            receiver: other.receiver.clone(),
        }
    }

    pub fn shutdown(&self, reason: ShutdownReason) {
        let _ = self.sender.send(Some(reason));
    }

    pub fn is_shutdown(&self) -> bool {
        return self.receiver.borrow().is_some();
    }

    pub fn get_reason(&self) -> Option<ShutdownReason> {
        return self.receiver.borrow().clone();
    }
}

pub struct Client;

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
        let mut connection = Connection {
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
        // This blocks until the user quits the TUI.
        Tui::run(t2c_tx, game_state, shutdown).await;

        // Cleanup
        let _ = communication_handle.await;
        println!("Client shutting down. Goodbye!");
    }
}
