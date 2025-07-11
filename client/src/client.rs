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

        let _name = Self::login();
        let map = Self::ask_for_map(&mut stream).await.unwrap();

        let (tx1, rx1) = mpsc::unbounded_channel();
        let (tx2, rx2) = mpsc::unbounded_channel();

        let _ = tokio::spawn(async move {
            loop {
                Self::comunicate_with_server(&mut stream, &tx1, &rx2).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        });

        let tui = tui::Tui::new();
        tui::Tui::run(tx2, rx1, map).await;
    }

    async fn comunicate_with_server(
        stream: &mut TcpStream,
        tui_tx: &mpsc::UnboundedSender<common::MapObjsE>,
        tui_rx: &mpsc::UnboundedReceiver<tui::PlayerInput>,
    ) {
        let _ = common::stream::send_msg_to_server(
            stream,
            &common::C2S::C2S4L(common::C2S4L::GiveObjs),
        )
        .await;

        match common::stream::get_msg_from_server(stream).await {
            Err(err) => println!("client:33 ERROR: {}", err),
            Ok(msg) => {
                match msg {
                    common::S2C::L2S4C(common::L2S4C::MapObjs(objs)) => {
                        let _ = tui_tx.send(objs);
                    }
                    probably_printable_msg => println!("{:?}", probably_printable_msg),
                };
            }
        }
    }

    fn login() -> String {
        let mut input = String::new();

        println!("Login:");

        std::io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    async fn ask_for_map(stream: &mut TcpStream) -> Result<Vec<Vec<common::TileE>>, ClientErr> {
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
