use std::collections::HashMap;

use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio::time;

use crate::logger;
use crate::terminal;
use crate::tui::{self, Tui};
use common::r#const::IP_LOCAL;
use common::exports::game_object::GameObjE;
use common::exports::player::PlayerE;
use common::exports::tile::TileE;
use common::{C2S, C2S4L, L2S4C, S2C, stream};

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
        logger::write(format_args!("net: connection loop started"));

        loop {
            tokio::select! {
                Some(msg) = t2c_rx.recv() => {
                    let msg = match msg {
                        tui::T2C::NewCastle(pos) => C2S::C2S4L(C2S4L::NewCastle(pos)),
                        tui::T2C::AttackCastle(id, units) => C2S::C2S4L(C2S4L::AttackCastle(id, units)),
                        tui::T2C::SendUnits(pos, units) => C2S::C2S4L(C2S4L::SendUnits(pos, units)),
                    };
                    if stream::send_msg_to_server(&mut self.writer, &msg).await.is_err() {
                        logger::write(format_args!("net: connection lost while sending command"));
                        Self::notify(s2c_tx, "Connection lost while sending command");
                        break;
                    }
                },
                result = stream::get_msg_from_server(&mut self.reader) => {
                    match result {
                        Ok(msg) => { let _ = s2c_tx.send(msg); },
                        Err(e) => {
                            logger::write(format_args!("net: server disconnected: {e}"));
                            Self::notify(s2c_tx, "Server disconnected");
                            break;
                        }
                    }
                },
                _ = request_tick.tick() => {
                    if self.poll_server_state(s2c_tx).await.is_err() {
                        logger::write(format_args!("net: connection lost during state poll"));
                        Self::notify(s2c_tx, "Connection lost during state update");
                        break;
                    }
                }
            }
        }
        logger::write(format_args!("net: connection loop ended"));
    }

    async fn poll_server_state(&mut self, s2c_tx: &mpsc::UnboundedSender<S2C>) -> Result<(), ()> {
        stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs))
            .await
            .map_err(|_| ())?;
        if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
            let _ = s2c_tx.send(msg);
        }

        stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GivePlayer))
            .await
            .map_err(|_| ())?;
        if let Ok(msg) = stream::get_msg_from_server(&mut self.reader).await {
            let _ = s2c_tx.send(msg);
        }

        Ok(())
    }

    fn notify(s2c_tx: &mpsc::UnboundedSender<S2C>, msg: &str) {
        let _ = s2c_tx.send(S2C::L2S4C(L2S4C::Log(msg.to_string())));
    }

    async fn fetch_map(&mut self) -> Result<Vec<Vec<TileE>>, String> {
        stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap))
            .await
            .map_err(|e| format!("Failed to request map: {e}"))?;
        match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => Ok(map),
            Ok(_) => Err("Unexpected response when fetching map".into()),
            Err(e) => Err(format!("Failed to parse map response: {e}")),
        }
    }

    async fn fetch_initial_state(
        &mut self,
    ) -> Result<(HashMap<usize, GameObjE>, Option<PlayerE>), String> {
        stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs))
            .await
            .map_err(|e| format!("Failed to request game objects: {e}"))?;
        let game_objs = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::GameObjs(objs))) => objs,
            Ok(_) => return Err("Unexpected response when fetching objects".into()),
            Err(e) => return Err(format!("Failed to parse objects response: {e}")),
        };

        stream::send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GivePlayer))
            .await
            .map_err(|e| format!("Failed to request player data: {e}"))?;
        let player_data = match stream::get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Player(data))) => Some(data),
            Ok(S2C::L2S4C(L2S4C::CreateCastle)) => None,
            Ok(_) => return Err("Unexpected response when fetching player".into()),
            Err(e) => return Err(format!("Failed to parse player response: {e}")),
        };

        Ok((game_objs, player_data))
    }
}

pub async fn run() {
    logger::write(format_args!("net: connecting to {IP_LOCAL}"));
    let stream = match TcpStream::connect(IP_LOCAL).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to server: {e}");
            return;
        }
    };
    let (reader, mut writer) = stream.into_split();
    logger::write(format_args!("net: connected"));

    println!("Connection established. Please log in.");
    let name = terminal::login();
    logger::write(format_args!("net: logging in as \"{name}\""));
    if let Err(e) = stream::send_msg_to_server(&mut writer, &C2S::Login(name)).await {
        eprintln!("Failed to send login: {e}");
        return;
    }

    let mut conn = Connection {
        writer,
        reader: BufReader::new(reader),
    };

    println!("Fetching initial game state...");
    let map = match conn.fetch_map().await {
        Ok(m) => {
            logger::write(format_args!(
                "net: map received ({}x{})",
                m.len(),
                m.first().map_or(0, |r| r.len())
            ));
            m
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let (initial_objs, initial_player) = match conn.fetch_initial_state().await {
        Ok(state) => {
            logger::write(format_args!(
                "net: initial state received ({} objs, player={})",
                state.0.len(),
                state.1.is_some()
            ));
            state
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
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
