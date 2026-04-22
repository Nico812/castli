mod ansi;
mod assets;
mod client;
mod r#const;
mod coord;
mod game_state;
mod input_handler;
mod renderer;
mod tui;
mod ui_state;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    client.run().await;
}
