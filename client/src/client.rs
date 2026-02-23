use std::collections::HashMap;
use tokio::{
    io::BufReader,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
    time,
};

use crate::tui::{self, Tui};
use common::{
    C2S, C2S4L, L2S4C, S2C,
    r#const::IP_LOCAL,
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE},
    stream,
};

struct Connection {
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
}

impl Connection {
    async fn run(
        &mut self,
        s2c_tx: &mpsc::UnboundedSender<S2C>,
        t2c_rx: &mut mpsc::UnboundedReceiver<tui::T2C>,
    ) {
        let mut request_tick = time::interval(time::Duration::from_millis(1000));

        loop {
            tokio::select! {
                Some(msg) = t2c_rx.recv() => {
                    let msg = match msg {
                        tui::T2C::NewCastle(pos) => C2S::C2S4L(C2S4L::NewCastle(pos)),
                        tui::T2C::AttackCastle(id, units) => C2S::C2S4L(C2S4L::AttackCastle(id, units)),
                        tui::T2C::SendUnits(pos, units) => C2S::C2S4L(C2S4L::SendUnits(pos, units)),
                    };
                    let _ = stream::send_msg_to_server(&mut self.writer, &msg).await;
                },
                Ok(msg) = stream::get_msg_from_server(&mut self.reader) => {
                    let _ = s2c_tx.send(msg);
                },
                _ = request_tick.tick() => {
                    let _ = stream::send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GiveObjs),
                    ).await;
                    if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
                        let _ = s2c_tx.send(msg);
                    }

                    let _ = stream::send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GivePlayer),
                    ).await;
                    if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
                        let _ = s2c_tx.send(msg);
                    }
                }
            }
        }
    }

    async fn fetch_map(&mut self) -> Vec<Vec<TileE>> {
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap)).await;
        match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => map,
            _ => panic!("Failed to receive map"),
        }
    }

    async fn fetch_initial_state(&mut self) -> (HashMap<usize, GameObjE>, Option<PlayerE>) {
        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs)).await;
        let game_objs = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::GameObjs(objs))) => objs,
            _ => panic!("Failed to receive game objects"),
        };

        let _ = stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GivePlayer)).await;
        let player_data = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Player(data))) => Some(data),
            Ok(S2C::L2S4C(L2S4C::CreateCastle)) => None,
            _ => panic!("Failed to receive player data"),
        };

        (game_objs, player_data)
    }
}

pub async fn run() {
    let stream = match TcpStream::connect(IP_LOCAL).await {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to connect to server: {e}");
            return;
        }
    };
    let (reader, mut writer) = stream.into_split();

    println!("Connection established. Please log in.");
    let name = Tui::login();
    let _ = stream::send_msg_to_server(&mut writer, &C2S::Login(name)).await;

    let mut conn = Connection {
        writer,
        reader: BufReader::new(reader),
    };

    println!("Fetching initial game state...");
    let map = conn.fetch_map().await;
    let (initial_objs, initial_player) = conn.fetch_initial_state().await;
    println!("Game state received.");

    let (s2c_tx, s2c_rx) = mpsc::unbounded_channel();
    let (t2c_tx, mut t2c_rx) = mpsc::unbounded_channel();

    let net_handle = tokio::spawn(async move {
        conn.run(&s2c_tx, &mut t2c_rx).await;
    });

    let tui = Tui::new(t2c_tx, s2c_rx, map, initial_objs, initial_player);
    tui.run().await;

    net_handle.abort();
    println!("\nGoodbye!");
}
