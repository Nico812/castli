mod ansi;
mod canvas;
mod canvas_modules;
mod client;
mod tui;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    client.run().await;
}
