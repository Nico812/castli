use std::sync::Arc;
use tokio::{
    io::BufReader,
    net::TcpStream,
    sync::{Mutex, mpsc},
};

use crate::connection::Connection;
use crate::shutdown::ShutdownChannel;
use crate::tui::Tui;
use common::{config::config, packets::C2S, stream};

pub struct Client {
    shutdown: ShutdownChannel,
}

impl Client {
    pub fn new() -> Self {
        let shutdown = ShutdownChannel::new();
        Self { shutdown }
    }

    /// Runs the main client application.
    pub async fn run(&mut self) {
        let addr = config().network.address.as_str();
        let stream = match TcpStream::connect(addr).await {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to connect to server: {}", e);
                return;
            }
        };

        let (reader, mut writer) = stream.into_split();

        println!("Connection established. Please log in.");
        let name = Tui::login();
        stream::send_msg_to_server(&mut writer, &C2S::Login(name))
            .await
            .unwrap();

        let lobby = Tui::choose_lobby();
        stream::send_msg_to_server(&mut writer, &C2S::Lobby(lobby))
            .await
            .unwrap();

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

        let (t2c_tx, t2c_rx) = mpsc::unbounded_channel();

        let communication_handle = tokio::spawn(connection.communicate_with_server(
            t2c_rx,
            ShutdownChannel::clone(&self.shutdown),
            Arc::clone(&game_state),
        ));

        let mut tui = Tui::new().await;
        tui.run(t2c_tx, game_state, ShutdownChannel::clone(&self.shutdown))
            .await;

        let _ = communication_handle.await;
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let shutdown_str = if let Some(reason) = self.shutdown.get_reason() {
            format!("{:?}", reason)
        } else {
            "no resaon".to_string()
        };
        println!("Client shutting down. Goodbye!");
        println!("Shutdown reason: {}", shutdown_str);
    }
}
