use std::sync::Arc;

use common::{
    map::Tile,
    packets::{C2S, C2S4L, L2S4C, LogE, MapPayload, S2C},
    stream::{StreamErr, get_msg_from_server, send_msg_to_server},
};
use tokio::{
    io::BufReader,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::{Mutex, mpsc::UnboundedReceiver},
};

use crate::{
    client::{ShutdownChannel, ShutdownReason},
    game_state::GameState,
    tui::T2C,
};

pub struct Connection {
    pub writer: OwnedWriteHalf,
    pub reader: BufReader<OwnedReadHalf>,
}

impl Connection {
    pub async fn communicate_with_server(
        mut self,
        mut t2c_rx: UnboundedReceiver<T2C>,
        shutdown: ShutdownChannel,
        game_state: Arc<Mutex<GameState>>,
    ) {
        loop {
            if shutdown.is_shutdown() {
                return;
            }

            tokio::select! {
                // Check for messages from the TUI to send to the server
                Some(msg_from_tui) = t2c_rx.recv() => {
                    let msg = match msg_from_tui {
                        T2C::NewCastle(pos) => C2S::C2S4L(C2S4L::NewCastle(pos)),
                        T2C::AttackCastle(target_id, unit_group_e) => {
                            C2S::C2S4L(C2S4L::AttackCastle(target_id, unit_group_e))
                        }
                        T2C::SendUnits(target_pos, unit_group_e) => {
                            C2S::C2S4L(C2S4L::SendUnits(target_pos, unit_group_e))
                        }
                        T2C::InCourtyard => {
                            C2S::C2S4L(C2S4L::InCourtyard)},
                        T2C::OutCourtyard => {
                            C2S::C2S4L(C2S4L::OutCourtyard)},
                        T2C::NewFacility((pos, fac_type)) => {
                            C2S::C2S4L(C2S4L::NewFacility((pos, fac_type)))
                        }
                    };
                    let _ = send_msg_to_server(&mut self.writer, &msg).await;
                },

                // Check for messages from the server and redirects them to the TUI
                // TODO: the tokio select here can cause data loss, should i address this?
                msg = get_msg_from_server(&mut self.reader) =>  {
                    let mut game_state = game_state.lock().await;

                    let msg = match msg {
                        Ok(msg) => msg,
                        Err(StreamErr::ConnectionEnded) => {
                            shutdown.shutdown(ShutdownReason::Connection);
                            return;
                        }
                        Err(StreamErr::SerializationErr) => {
                            game_state.add_log("Some serialization error...");
                            continue;
                        }
                    };

                    match msg {
                        S2C::L2S4C(L2S4C::MainPacket(packet)) => {
                            game_state.castle = packet.castle;
                            game_state.player = packet.player;
                            game_state.objs = packet.objs;
                            game_state.time = packet.time;
                        }
                        S2C::L2S4C(L2S4C::CourtyardPacket(packet)) => {
                            game_state.facilities = packet.facilities;
                            game_state.castle = Some(packet.castle);
                            game_state.player = packet.player;
                            game_state.time = packet.time;
                        },
                        S2C::L2S4C(L2S4C::Map(payload)) => {
                            game_state.map = unflatten_map(payload);
                        }
                        S2C::L2S4C(L2S4C::Log(log)) => {
                            let string = match log {
                                LogE::CastleCreationErr => {"Could not create castle".to_string()},
                                LogE::UnitDeployErr => {"Could not deploy units".to_string()},
                                LogE::AttackDeployErr => {"Could not attack ziocan".to_string()},
                                LogE::FacilityCreationErr => {"Could not create new facility".to_string()}
                            };
                            game_state.add_log(string);
                        }
                        S2C::ServerShutdown => {
                            shutdown.shutdown(ShutdownReason::ServerShutdown);
                        }
                        S2C::LobbyFound => {
                            game_state.add_log("Lobby found");
                        }
                        S2C::LobbyFull => {
                            game_state.add_log("Lobby full");
                        }
                        S2C::ConnectionFailed => {
                            game_state.add_log("Connection failed");
                        }

                    }
                }
            }
        }
    }

    pub async fn fetch_initial_state(&mut self) -> Result<GameState, ()> {
        let map = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(payload))) => unflatten_map(payload),
            _ => {
                println!("Failed to receive map");
                return Err(());
            }
        };
        println!("Received map");

        let (time, objs, client, castle) = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::MainPacket(packet))) => {
                (packet.time, packet.objs, packet.player, packet.castle)
            }
            _ => {
                println!("Failed to receive game objs");
                return Err(());
            }
        };
        println!("Received main packet");

        Ok(GameState::new(time, objs, map, client, castle))
    }
}

fn unflatten_map(payload: MapPayload) -> Vec<Vec<Tile>> {
    let rows = payload.rows as usize;
    let cols = payload.cols as usize;
    let mut out = Vec::with_capacity(rows);
    let mut iter = payload.tiles.into_iter();
    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(iter.next().unwrap_or(Tile::Err));
        }
        out.push(row);
    }
    out
}
