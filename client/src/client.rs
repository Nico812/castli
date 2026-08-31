use std::sync::Arc;
use tokio::{
    io::BufReader,
    net::TcpStream,
    sync::{Mutex, mpsc},
};

use crate::connection::Connection;
use crate::shutdown::ShutdownChannel;
use crate::tui::Tui;
use common::{
    r#const::{IP_LOCAL, ONLINE},
    packets::C2S,
    stream,
};

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
        let addr = if ONLINE { IP_LOCAL } else { IP_LOCAL };
        let stream = match TcpStream::connect(addr).await {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to connect to server: {}", e);
                return;
            }
        };

        let (reader, writer) = stream.into_split();
        let mut connection = Connection {
            writer,
            reader: BufReader::new(reader),
        };

        let (t2c_tx, t2c_rx) = mpsc::unbounded_channel();

        let mut tui = Tui::new(t2c_tx, ShutdownChannel::clone(&self.shutdown)).await;

        let name = tui.login();
        connection
            .send_msg_to_server(&C2S::Login(name))
            .await
            .unwrap();

        let lobbies_info = connection.fetch_lobbies_info().await;
        let lobby = tui.choose_lobby(lobbies_info).await;
        connection
            .send_msg_to_server(&C2S::Lobby(lobby))
            .await
            .unwrap();

        let game_state = Arc::new(Mutex::new(
            connection
                .fetch_initial_state()
                .await
                .expect("Failed to receive initial state."),
        ));

        let communication_handle = tokio::spawn(connection.communicate_with_server(
            t2c_rx,
            ShutdownChannel::clone(&self.shutdown),
            Arc::clone(&game_state),
        ));

        tui.run_game(game_state).await;
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
