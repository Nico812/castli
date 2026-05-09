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
    game_state::GameState,
    shutdown::{ShutdownChannel, ShutdownReason},
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
                Some(msg_from_tui) = t2c_rx.recv() => {
                    let msg = C2S::C2S4L(t2c_to_c2s4l(msg_from_tui));
                    let _ = send_msg_to_server(&mut self.writer, &msg).await;
                },

                // TODO: the tokio select here can cause data loss, should i address this?
                msg = get_msg_from_server(&mut self.reader) => {
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

                    handle_server_msg(msg, &mut game_state, &shutdown);
                }
            }
        }
    }

    pub async fn fetch_initial_state(&mut self) -> Result<GameState, ()> {
        let map = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(payload))) => payload.unflatten(),
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

fn t2c_to_c2s4l(msg: T2C) -> C2S4L {
    match msg {
        T2C::NewCastle(pos) => C2S4L::NewCastle(pos),
        T2C::AttackCastle(target_id, units) => C2S4L::AttackCastle(target_id, units),
        T2C::SendUnits(target_pos, units) => C2S4L::SendUnits(target_pos, units),
        T2C::InCourtyard => C2S4L::InCourtyard,
        T2C::OutCourtyard => C2S4L::OutCourtyard,
        T2C::NewFacility(payload) => C2S4L::NewFacility(payload),
    }
}

fn handle_server_msg(msg: S2C, game_state: &mut GameState, shutdown: &ShutdownChannel) {
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
        }
        S2C::L2S4C(L2S4C::Map(payload)) => {
            game_state.map = payload.unflatten();
        }
        S2C::L2S4C(L2S4C::Log(log)) => {
            let string = match log {
                LogE::CastleCreationErr => "Could not create castle".to_string(),
                LogE::UnitDeployErr => "Could not deploy units".to_string(),
                LogE::AttackDeployErr => "Could not attack ziocan".to_string(),
                LogE::FacilityCreationErr => "Could not create new facility".to_string(),
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
