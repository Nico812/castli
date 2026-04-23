mod client;
mod r#const;
mod game;
mod lobby;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    println!("Server started");

    server.run().await;
}
