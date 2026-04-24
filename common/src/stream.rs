use crate::{C2S, S2C};
use serde_json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub enum StreamErr {
    ConnectionEnded,
    SerializationErr,
}

pub async fn send_msg_to_server(stream: &mut OwnedWriteHalf, msg: &C2S) -> tokio::io::Result<()> {
    let json = serde_json::to_string(msg).expect("Serialization failed");
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    Ok(())
}

pub async fn send_msg_to_client(stream: &mut OwnedWriteHalf, msg: &S2C) -> tokio::io::Result<()> {
    let json = serde_json::to_string(msg).expect("Serialization failed");
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    Ok(())
}

pub async fn get_msg_from_server(reader: &mut BufReader<OwnedReadHalf>) -> Result<S2C, StreamErr> {
    let mut buf = String::new();
    if let Ok(0) = reader.read_line(&mut buf).await {
        Err(StreamErr::ConnectionEnded)
    } else if let Ok(msg) = serde_json::from_str(&buf) {
        Ok(msg)
    } else {
        Err(StreamErr::SerializationErr)
    }
}

pub async fn get_msg_from_client(reader: &mut BufReader<OwnedReadHalf>) -> Result<C2S, StreamErr> {
    let mut buf = String::new();
    if let Ok(0) = reader.read_line(&mut buf).await {
        Err(StreamErr::ConnectionEnded)
    } else if let Ok(msg) = serde_json::from_str(&buf) {
        Ok(msg)
    } else {
        Err(StreamErr::SerializationErr)
    }
}
