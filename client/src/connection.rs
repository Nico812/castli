use std::sync::Arc;

use common::{
    packets::{C2S, C2S4L, L2S4C, LogE, S2C},
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
                        S2C::L2S4C(L2S4C::Map(map)) => {
                            game_state.map = map;
                        }
                        S2C::L2S4C(L2S4C::Log(log)) => {
                            let string = match log {
                                LogE::CastleCreationErr => {"Could not create castle".to_string()},
                                LogE::UnitDeployErr => {"Could not deploy units".to_string()},
                                LogE::AttackDeployErr => {"Could not attack ziocan".to_string()},
                            };
                            game_state.add_log(string);
                        }
                        S2C::ServerShutdown => {
                            shutdown.shutdown(ShutdownReason::ServerShutdown);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub async fn fetch_initial_state(&mut self) -> Result<GameState, ()> {
        let map = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => map,
            _ => {
                println!("Failed to receive map");
                return Err(());
            }
        };

        let (time, objs, client, castle) = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::MainPacket(packet))) => {
                (packet.time, packet.objs, packet.player, packet.castle)
            }
            _ => {
                println!("Failed to receive game objs");
                return Err(());
            }
        };

        Ok(GameState::new(time, objs, map, client, castle))
    }
}
