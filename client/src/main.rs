mod ansi;
mod assets;
mod camera;
mod client;
mod connection;
mod r#const;
mod coord;
mod game_state;
mod input_handler;
mod logs;
mod renderer;
mod shutdown;
mod tui;
mod ui_state;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    client.run().await;
}
