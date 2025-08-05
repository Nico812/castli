//! # Castli Server
//!
//! This is the main entry point for the Castli game server.
//! It initializes and runs the `Server` instance, which handles incoming
//! client connections and manages game lobbies.

mod r#const;
mod game;
mod lobby;
mod player;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    println!("Server started");

    server.run().await;
}
