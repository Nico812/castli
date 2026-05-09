use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use bincode::{config, serde::decode_from_slice, serde::encode_to_vec};

use common::{
    packets::{C2S, C2S4L, L2S4C, S2C},
    stream::{MAX_FRAME_BYTES, StreamErr},
};

use crate::server::{Client, ConnId};

pub struct Connection {
    pub stream: TcpStream,
    pub read_buffer: Vec<u8>,
    pub write_buffer: Vec<u8>,
    pub lobby_link: Option<(Sender<C2S4L>, Receiver<L2S4C>)>,
    pub client: Option<Client>,
    pub id: ConnId,
}

impl Connection {
    pub fn new(stream: TcpStream, id: ConnId) -> Self {
        Self {
            stream,
            id,
            lobby_link: None,
            client: None,
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
        }
    }

    pub fn try_get_msg(&mut self) -> Result<Option<C2S>, StreamErr> {
        let mut tmp_buf = [0u8; 4096];
        match self.stream.read(&mut tmp_buf) {
            Ok(0) => return Err(StreamErr::ConnectionEnded),
            Ok(n) => self.read_buffer.extend_from_slice(&tmp_buf[..n]),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(_) => return Err(StreamErr::ConnectionEnded),
        }

        if self.read_buffer.len() < 4 {
            return Ok(None);
        }
        let len = u32::from_le_bytes(self.read_buffer[..4].try_into().unwrap());
        if len > MAX_FRAME_BYTES {
            return Err(StreamErr::SerializationErr);
        }
        let frame_end = 4 + len as usize;
        if self.read_buffer.len() < frame_end {
            return Ok(None);
        }
        let result =
            decode_from_slice::<C2S, _>(&self.read_buffer[4..frame_end], config::standard())
                .map(|(msg, _)| Some(msg))
                .map_err(|_| StreamErr::SerializationErr);
        self.read_buffer.drain(..frame_end);
        result
    }

    pub fn queue_msg(&mut self, msg: &S2C) {
        let bytes = encode_to_vec(msg, config::standard()).expect("Serialization failed");
        let len: u32 = bytes
            .len()
            .try_into()
            .expect("Outgoing frame exceeds u32 length");
        self.write_buffer.extend_from_slice(&len.to_le_bytes());
        self.write_buffer.extend_from_slice(&bytes);
    }

    pub fn try_flush(&mut self) -> Result<(), StreamErr> {
        while !self.write_buffer.is_empty() {
            match self.stream.write(&self.write_buffer) {
                Ok(0) => return Err(StreamErr::ConnectionEnded),
                Ok(n) => {
                    self.write_buffer.drain(..n);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                Err(_) => return Err(StreamErr::ConnectionEnded),
            }
        }
        Ok(())
    }
}
