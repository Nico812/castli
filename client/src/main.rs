mod canvas;
mod logger;
mod net;
mod terminal;
mod tui;

#[tokio::main]
async fn main() {
    logger::init();
    net::run().await;
}
