mod ansi;
mod assets;
mod client;
mod r#const;
mod coord;
mod game_renderer;
mod input_handler;
mod tui;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    client.run().await;
}
