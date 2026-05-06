use bincode::{config, serde::decode_from_slice, serde::encode_to_vec};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::packets::{C2S, S2C};

pub enum StreamErr {
    ConnectionEnded,
    SerializationErr,
}

pub const MAX_FRAME_BYTES: u32 = 256 * 1024 * 1024;

pub async fn send_msg_to_server(stream: &mut OwnedWriteHalf, msg: &C2S) -> tokio::io::Result<()> {
    let bytes = encode_to_vec(msg, config::standard()).expect("Serialization failed");
    let len: u32 = bytes
        .len()
        .try_into()
        .expect("Outgoing frame exceeds u32 length");
    stream.write_all(&len.to_le_bytes()).await?;
    stream.write_all(&bytes).await?;
    let _ = stream.flush().await;
    Ok(())
}

pub async fn get_msg_from_server(reader: &mut BufReader<OwnedReadHalf>) -> Result<S2C, StreamErr> {
    let len = match reader.read_u32_le().await {
        Ok(n) => n,
        Err(_) => return Err(StreamErr::ConnectionEnded),
    };
    if len > MAX_FRAME_BYTES {
        return Err(StreamErr::SerializationErr);
    }
    let mut buf = vec![0u8; len as usize];
    if reader.read_exact(&mut buf).await.is_err() {
        return Err(StreamErr::ConnectionEnded);
    }
    decode_from_slice::<S2C, _>(&buf, config::standard())
        .map(|(msg, _)| msg)
        .map_err(|_| StreamErr::SerializationErr)
}
