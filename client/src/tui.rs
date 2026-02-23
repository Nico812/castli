use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use tokio::io::{self, AsyncReadExt};
use tokio::sync::{Mutex, mpsc};
use tokio::time;

use crate::canvas::{Canvas, RightModuleTab};
use crate::logger;
use crate::terminal;
use common::r#const::{MAP_COLS, MAP_ROWS};
use common::exports::game_object::GameObjE;
use common::exports::player::PlayerE;
use common::exports::tile::TileE;
use common::exports::units::UnitGroupE;
use common::{GameCoord, GameID, L2S4C, S2C};

pub enum T2C {
    NewCastle(GameCoord),
    AttackCastle(GameID, UnitGroupE),
    SendUnits(GameCoord, UnitGroupE),
}

struct SharedState {
    game_objs: HashMap<GameID, GameObjE>,
    player_data: PlayerE,
    map_zoom: Option<GameCoord>,
    map_look: Option<GameCoord>,
    logs: VecDeque<String>,
    right_mod_tab: RightModuleTab,
}

pub struct Tui {
    to_server_tx: mpsc::UnboundedSender<T2C>,
    from_server_rx: mpsc::UnboundedReceiver<S2C>,
    canvas: Canvas,
    state: Arc<Mutex<SharedState>>,
}

impl Tui {
    pub fn new(
        tx: mpsc::UnboundedSender<T2C>,
        rx: mpsc::UnboundedReceiver<S2C>,
        tiles: Vec<Vec<TileE>>,
        initial_game_objs: HashMap<GameID, GameObjE>,
        initial_player_data: Option<PlayerE>,
    ) -> Self {
        let mut canvas = Canvas::new();
        canvas.init(tiles);

        let player_data = initial_player_data.unwrap_or_else(PlayerE::undef);

        Self {
            to_server_tx: tx,
            from_server_rx: rx,
            canvas,
            state: Arc::new(Mutex::new(SharedState {
                game_objs: initial_game_objs,
                player_data,
                map_zoom: None,
                map_look: None,
                logs: VecDeque::new(),
                right_mod_tab: RightModuleTab::Castle,
            })),
        }
    }

    pub async fn run(self) {
        terminal::set_raw_mode();
        terminal::hide_cursor();
        logger::write(format_args!("tui: started"));

        let state = self.state;

        let com_handle = tokio::spawn(Self::listen_for_server_updates(
            self.from_server_rx,
            Arc::clone(&state),
        ));

        let ui_handle = tokio::spawn(Self::render_loop(self.canvas, Arc::clone(&state)));

        Self::handle_player_input(self.to_server_tx, state).await;

        com_handle.abort();
        ui_handle.abort();
        terminal::reset_mode();
        logger::write(format_args!("tui: shutdown"));
    }

    async fn render_loop(mut canvas: Canvas, state: Arc<Mutex<SharedState>>) {
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;
        terminal::clear_screen();

        loop {
            let now = time::Instant::now();
            let dt = now.duration_since(last_frame).as_millis() as u64;
            if dt >= 10 {
                frame_dt = dt;
            }
            last_frame = now;

            {
                let guard = state.lock().await;

                canvas.change_right_tab(guard.right_mod_tab);
                canvas.render(
                    &guard.game_objs,
                    &guard.player_data,
                    guard.map_zoom,
                    guard.map_look,
                    frame_dt,
                    &guard.logs,
                );
            }

            render_tick.tick().await;
        }
    }

    async fn listen_for_server_updates(
        mut rx: mpsc::UnboundedReceiver<S2C>,
        state: Arc<Mutex<SharedState>>,
    ) {
        while let Some(msg) = rx.recv().await {
            let mut state = state.lock().await;
            match msg {
                S2C::L2S4C(L2S4C::GameObjs(objs)) => {
                    logger::write(format_args!("tui: received {} game objects", objs.len()));
                    state.game_objs = objs;
                }
                S2C::L2S4C(L2S4C::Player(data)) => state.player_data = data,
                S2C::L2S4C(L2S4C::Log(msg)) => {
                    logger::write(format_args!("tui: server log: {msg}"));
                    state.logs.push_back(msg);
                }
                _ => {}
            }
        }
    }

    async fn handle_player_input(tx: mpsc::UnboundedSender<T2C>, state: Arc<Mutex<SharedState>>) {
        loop {
            let mut buf = [0u8; 8];
            let n = match io::stdin().read(&mut buf).await {
                Ok(0) => {
                    logger::write(format_args!("tui: stdin EOF"));
                    break;
                }
                Err(e) => {
                    logger::write(format_args!("tui: stdin error: {e}"));
                    break;
                }
                Ok(n) => n,
            };

            if n == 1 {
                match buf[0] as char {
                    'q' => {
                        logger::write(format_args!("tui: quit requested"));
                        break;
                    }
                    'z' => {
                        let mut s = state.lock().await;
                        s.map_zoom = match s.map_zoom {
                            None => Some(GameCoord { x: 0, y: 0 }),
                            Some(_) => None,
                        };
                    }
                    'l' => {
                        let mut s = state.lock().await;
                        if let Some(zoom_coord) = s.map_zoom {
                            s.map_look = match s.map_look {
                                None => Some(zoom_coord),
                                Some(_) => None,
                            };
                        }
                    }
                    'a' => {
                        let mut s = state.lock().await;
                        if let Some((pos, id)) = Self::get_selected_obj(&s.game_objs, s.map_look) {
                            let units = UnitGroupE {
                                quantities: [1, 0, 0],
                            };
                            if let Some(target_id) = id {
                                let _ = tx.send(T2C::AttackCastle(target_id, units));
                                s.logs.push_back(format!("Attacking object {target_id}"));
                            } else {
                                let _ = tx.send(T2C::SendUnits(pos, units));
                                s.logs
                                    .push_back(format!("Sending troops to ({}, {})", pos.y, pos.x));
                            }
                        }
                    }
                    'n' => {
                        let mut s = state.lock().await;
                        let Some(map_look) = s.map_look else {
                            continue;
                        };
                        let _ = tx.send(T2C::NewCastle(map_look));
                        s.logs
                            .push_back(format!("Castle added at ({}, {})", map_look.y, map_look.x));
                    }
                    '1' => state.lock().await.right_mod_tab = RightModuleTab::Castle,
                    '2' => state.lock().await.right_mod_tab = RightModuleTab::Logs,
                    '3' => state.lock().await.right_mod_tab = RightModuleTab::Debug,
                    _ => {}
                }
            }

            if n >= 3 {
                match &buf[..n] {
                    [0x1b, b'[', b'A'] => Self::apply_move(&state, 0, -1).await,
                    [0x1b, b'[', b'B'] => Self::apply_move(&state, 0, 1).await,
                    [0x1b, b'[', b'C'] => Self::apply_move(&state, 1, 0).await,
                    [0x1b, b'[', b'D'] => Self::apply_move(&state, -1, 0).await,
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => Self::apply_move(&state, 0, -16).await,
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => Self::apply_move(&state, 0, 16).await,
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => Self::apply_move(&state, 16, 0).await,
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => Self::apply_move(&state, -16, 0).await,
                    _ => {}
                }
            }
        }
    }

    async fn apply_move(state: &Arc<Mutex<SharedState>>, dx: isize, dy: isize) {
        let mut s = state.lock().await;
        if let Some(ref mut look) = s.map_look {
            look.x = ((look.x as isize + dx).max(0) as usize).min(MAP_COLS);
            look.y = ((look.y as isize + dy).max(0) as usize).min(MAP_ROWS);
        } else if let Some(ref mut zoom) = s.map_zoom {
            zoom.x = ((zoom.x as isize + dx * 2).max(0) as usize).min(MAP_COLS);
            zoom.y = ((zoom.y as isize + dy * 2).max(0) as usize).min(MAP_ROWS);
        }
    }

    fn get_selected_obj(
        game_objs: &HashMap<GameID, GameObjE>,
        map_look: Option<GameCoord>,
    ) -> Option<(GameCoord, Option<GameID>)> {
        let world_pos = map_look?;
        let target_id = game_objs
            .iter()
            .find(|(_, obj)| obj.get_pos() == world_pos)
            .map(|(id, _)| *id);
        Some((world_pos, target_id))
    }
}
