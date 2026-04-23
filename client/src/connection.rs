use std::{sync::Arc, time::Duration};

use common::{
    C2S, C2S4L, L2S4C, S2C,
    stream::{get_msg_from_server, send_msg_to_server},
};
use tokio::{
    io::BufReader,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::{Mutex, mpsc::UnboundedReceiver},
    time::interval,
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
        let mut request_tick = interval(Duration::from_millis(1000));

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
                Ok(msg) = get_msg_from_server(&mut self.reader) =>  {
                    let mut game_state = game_state.lock().await;
                    match msg {
                        S2C::L2S4C(L2S4C::GameObjs(objs)) => {
                            game_state.objs = objs;
                        }
                        S2C::L2S4C(L2S4C::Client(client)) => {
                            game_state.client = client;
                        }
                        S2C::L2S4C(L2S4C::OwnedCastle(castle)) => {
                            game_state.castle = Some(castle);
                        }
                        S2C::L2S4C(L2S4C::Log(msg)) => {
                            game_state.add_log(msg);
                        }
                        S2C::L2S4C(L2S4C::CreateCastle) => {
                            game_state.castle = None;
                        }
                        S2C::ServerShutdown => {
                            shutdown.shutdown(ShutdownReason::ServerShutdown);
                        }
                        _ => {}
                    }
                }
                // Otherwise, run the periodic update requests
                _ = request_tick.tick() => {
                    let _ = send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GiveObjs),
                    ).await;

                    let _ = send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GiveClient),
                    ).await;

                    let _ = send_msg_to_server(
                        &mut self.writer,
                        &C2S::C2S4L(C2S4L::GiveOwnedCastle),
                    ).await;
                }
            }
        }
    }

    // Fetches the initial game objects and player data required to start the TUI.
    pub async fn fetch_initial_state(&mut self) -> Result<GameState, ()> {
        // Request map
        let _ = send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveMap)).await;
        let map = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Map(map))) => map,
            _ => {
                println!("Failed to receive map");
                return Err(());
            }
        };

        // Request game objects
        let _ = send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveObjs)).await;
        let objs = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::GameObjs(objs))) => objs,
            _ => {
                println!("Failed to receive game objs");
                return Err(());
            }
        };

        let _ = send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveClient)).await;
        let client = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::Client(client))) => client,
            _ => {
                println!("Failed to receive client");
                return Err(());
            }
        };

        let _ = send_msg_to_server(&mut self.writer, &C2S::C2S4L(C2S4L::GiveOwnedCastle)).await;
        let castle = match get_msg_from_server(&mut self.reader).await {
            Ok(S2C::L2S4C(L2S4C::OwnedCastle(castle))) => Some(castle),
            Ok(S2C::L2S4C(L2S4C::CreateCastle)) => None,
            _ => {
                println!("Failed to receive castle");
                return Err(());
            }
        };

        Ok(GameState::new(objs, map, client, castle))
    }
}
