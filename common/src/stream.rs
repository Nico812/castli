use crate::{C2S, S2C};
use serde_json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub async fn send_msg_to_server(stream: &mut TcpStream, msg: &C2S) -> tokio::io::Result<()> {
    let json = serde_json::to_string(msg).expect("Serialization failed");
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    Ok(())
}

pub async fn send_msg_to_client(stream: &mut TcpStream, msg: &S2C) -> tokio::io::Result<()> {
    let json = serde_json::to_string(msg).expect("Serialization failed");
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    Ok(())
}

pub async fn get_msg_from_server(stream: &mut TcpStream) -> Result<S2C, serde_json::Error> {
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    reader.read_line(&mut buf).await.unwrap(); // Gestione errore più robusta consigliata
    serde_json::from_str(&buf)
}

pub async fn get_msg_from_client(stream: &mut TcpStream) -> Result<C2S, serde_json::Error> {
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    reader.read_line(&mut buf).await.unwrap(); // Gestione errore più robusta consigliata
    serde_json::from_str(&buf)
}
