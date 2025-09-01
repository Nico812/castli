//! # Castli Client
//!
//! This is the main entry point for the Castli game client.
//! It initializes and runs the `Client` instance, which connects to the
//! server and manages the terminal user interface (TUI).
mod ansi;
mod assets;
mod canvas;
mod client;
mod r#const;
mod tui;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    client.run().await;
}
