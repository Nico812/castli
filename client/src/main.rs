mod ansi;
mod assets;
mod canvas;
mod client;
mod coord;
mod tui;

#[tokio::main]
async fn main() {
    client::run().await;
}
