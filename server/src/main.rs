mod client;
mod r#const;
mod game;
mod lobby;
mod server;
mod thread_pool;

use server::Server;

fn main() {
    let mut server = Server::new();
    println!("Server started");

    server.run();
}
