//! # Client Core Logic
//!
//! This module contains the `Client` struct, which manages the client's
//! connection to the server and orchestrates the different parts of the
//! client application, such as the TUI and server communication.
use std::collections::HashMap;
use tokio::{self, net::TcpStream, sync::mpsc};

use crate::tui;
use common;
use common::r#const;

#[derive(Debug)]
pub enum ClientErr {
    MapNotReceived,
}

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the main client loop.
    ///
    /// This function connects to the server, performs the initial login and map request,
    /// and then starts the TUI and server communication tasks.
    pub async fn run(&mut self) {
        let addr = if r#const::ONLINE {
            //r#const::IP_4_CLIENT
            r#const::IP_LOCAL
        } else {
            r#const::IP_LOCAL
        };
        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to connect to server");
        if cfg!(debug_assertions) {
            println!("Connection established");
        }

        let (tx1, rx1) = mpsc::unbounded_channel();
        let (tx2, rx2) = mpsc::unbounded_channel();

        // Autentication
        let name = tui::Tui::login();
        let _ = common::stream::send_msg_to_server(&mut stream, &common::C2S::Login(name)).await;

        // Actual game starts
        let map = Self::ask_for_map(&mut stream).await.unwrap();
        let _tui = tui::Tui::new(tx2, rx1, map).await;
        let _ = tokio::spawn(async move {
            loop {
                Self::comunicate_with_server(&mut stream, &tx1, &rx2).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        });
    }

    /// Handles the ongoing communication with the server.
    ///
    /// This function is typically run in a loop to periodically request updates
    /// from the server and send player input.
    async fn comunicate_with_server(
        stream: &mut TcpStream,
        tui_tx: &mpsc::UnboundedSender<common::S2C>,
        tui_rx: &mpsc::UnboundedReceiver<tui::T2C>,
    ) {
        let _ = common::stream::send_msg_to_server(
            stream,
            &common::C2S::C2S4L(common::C2S4L::GiveObjs),
        )
        .await;

        match common::stream::get_msg_from_server(stream).await {
            Err(err) => println!("client:33 ERROR: {}", err),
            Ok(msg) => {
                let _ = tui_tx.send(msg);
            }
        }

        let _ = common::stream::send_msg_to_server(
            stream,
            &common::C2S::C2S4L(common::C2S4L::GivePlayerData),
        )
        .await;

        match common::stream::get_msg_from_server(stream).await {
            Err(err) => println!("client:33 ERROR: {}", err),
            Ok(msg) => {
                let _ = tui_tx.send(msg);
            }
        }
    }

    async fn ask_for_map(stream: &mut TcpStream) -> Result<Vec<Vec<common::TileE>>, ClientErr> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        println!("sending map request");
        let _ =
            common::stream::send_msg_to_server(stream, &common::C2S::C2S4L(common::C2S4L::GiveMap))
                .await;

        let _ =
            common::stream::send_msg_to_server(stream, &common::C2S::C2S4L(common::C2S4L::GiveMap))
                .await;

        match common::stream::get_msg_from_server(stream).await {
            Err(_) => Err(ClientErr::MapNotReceived),
            Ok(msg) => match msg {
                common::S2C::L2S4C(common::L2S4C::Map(map)) => Ok(map),
                _ => Err(ClientErr::MapNotReceived),
            },
        }
    }
}
