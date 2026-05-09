mod connection;
mod game;
mod lobby;
mod player;
mod server;
mod thread_pool;

use server::Server;

fn main() {
    let mut server = Server::new();
    println!("Server started");

    server.run();
}
