//! # Terminal User Interface (TUI)
//!
//! This module manages the entire terminal user interface. It handles raw mode,
//! rendering the game state to the terminal using the `canvas`, and processing
//! player input.

use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    process::Command,
    sync::Arc,
};
use tokio::{
    io::{self, AsyncReadExt},
    sync::{Mutex, mpsc},
    time,
};

use crate::canvas::{
    canvas::Canvas,
    r#const::{CENTRAL_MODULE_CONTENT_COLS, CENTRAL_MODULE_CONTENT_ROWS},
};
use common::{
    GameCoord, GameID, L2S4C, S2C,
    r#const::{MAP_COLS, MAP_ROWS},
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE, units::UnitGroupE},
};

#[derive(Clone, Copy)]
pub struct TermCoord {
    pub x: usize,
    pub y: usize,
}

impl TermCoord {
    pub fn from_game_coord(game_coord: GameCoord, map_zoom: TermCoord) -> (i32, i32) {
        (
            game_coord.y as i32 / 2 - map_zoom.y as i32,
            game_coord.x as i32 - map_zoom.x as i32,
        )
    }
}

pub trait FromTermCoord {
    fn from_term_coord(term_coord: TermCoord, map_zoom: TermCoord) -> GameCoord;
}

impl FromTermCoord for GameCoord {
    fn from_term_coord(term_coord: TermCoord, map_zoom: TermCoord) -> GameCoord {
        Self {
            x: term_coord.x + map_zoom.x,
            y: (term_coord.y + map_zoom.y) * 2,
        }
    }
}

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle(GameCoord),
    AttackCastle(GameID, UnitGroupE),
}

enum TuiState {
    InGame,
    CastleCreation,
}

pub struct Tui {
    // Tui state
    _state: TuiState,
    // Communication channels
    to_server_tx: mpsc::UnboundedSender<T2C>,
    from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<S2C>>>,

    // UI and Game State
    canvas: Arc<Mutex<Canvas>>,
    game_objs: Arc<Mutex<HashMap<GameID, GameObjE>>>,
    player_data: Arc<Mutex<PlayerE>>,
    map_zoom: Arc<Mutex<Option<TermCoord>>>,
    map_look: Arc<Mutex<Option<TermCoord>>>,
    logs: Arc<Mutex<VecDeque<String>>>,
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
        let mut state = TuiState::InGame;
        let map_look = None;
        let map_zoom = None;
        // TODO: Add a max capacity (what happens when it runs out?)
        let logs: VecDeque<String> = VecDeque::from(vec![
            "log1".to_string(),
            "log2".to_string(),
            "log3".to_string(),
        ]);

        let player_data = match initial_player_data {
            Some(player_data) => player_data,
            None => {
                state = TuiState::CastleCreation;
                PlayerE::undef()
            }
        };

        Self {
            _state: state,
            to_server_tx: tx,
            from_server_rx: Arc::new(Mutex::new(rx)),
            canvas: Arc::new(Mutex::new(canvas)),
            game_objs: Arc::new(Mutex::new(initial_game_objs)),
            player_data: Arc::new(Mutex::new(player_data)),
            map_zoom: Arc::new(Mutex::new(map_zoom)),
            map_look: Arc::new(Mutex::new(map_look)),
            logs: Arc::new(Mutex::new(logs)),
        }
    }

    pub async fn run(&mut self) {
        Self::set_raw_mode();

        // Spawn a task to listen for updates from the server
        let com_handle = tokio::spawn(Self::listen_for_server_updates(
            Arc::clone(&self.from_server_rx),
            Arc::clone(&self.game_objs),
            Arc::clone(&self.player_data),
            Arc::clone(&self.logs),
        ));

        // Spawn a task to render the UI
        let ui_handle = tokio::spawn(Self::render_loop(
            Arc::clone(&self.canvas),
            Arc::clone(&self.game_objs),
            Arc::clone(&self.player_data),
            Arc::clone(&self.map_zoom),
            Arc::clone(&self.map_look),
            Arc::clone(&self.logs),
        ));

        // The main TUI task now only handles player input
        Self::handle_player_input(
            &self.to_server_tx,
            Arc::clone(&self.map_zoom),
            Arc::clone(&self.map_look),
            Arc::clone(&self.game_objs),
            Arc::clone(&self.logs),
        )
        .await;

        // When input handling ends (e.g., user presses 'q'), abort other tasks and clean up.
        com_handle.abort();
        ui_handle.abort();
        Self::reset_mode();
    }

    async fn render_loop(
        canvas_arc: Arc<Mutex<Canvas>>,
        game_objs_arc: Arc<Mutex<HashMap<GameID, GameObjE>>>,
        player_data_arc: Arc<Mutex<PlayerE>>,
        map_zoom_arc: Arc<Mutex<Option<TermCoord>>>,
        map_look_arc: Arc<Mutex<Option<TermCoord>>>,
        logs_arc: Arc<Mutex<VecDeque<String>>>,
    ) {
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;
        Self::clear_screen();

        loop {
            // Rendering fps
            // There's a problem that the frames can go really fast when there is delay
            // so i take only the frames with a reasonable high dt.
            let now = time::Instant::now();
            let dt = now.duration_since(last_frame).as_millis() as u64;
            if dt >= 10 {
                frame_dt = dt;
            };
            last_frame = now;

            // Rendering
            // Self::clear_screen(); // For cool visuals
            {
                let mut canvas = canvas_arc.lock().await;
                let game_objs = game_objs_arc.lock().await;
                let player_data = player_data_arc.lock().await;
                let map_zoom = map_zoom_arc.lock().await;
                let map_look = map_look_arc.lock().await;
                let mut logs = logs_arc.lock().await;

                // Selected object
                let sel_obj_id = Self::get_selected_obj_id(&game_objs, *map_zoom, *map_look);

                canvas.render(
                    &game_objs,
                    &player_data,
                    *map_zoom,
                    frame_dt,
                    &mut *logs,
                    sel_obj_id,
                );
                canvas.update_and_print_cursor(*map_look);
                let _ = std::io::stdout().flush();
            }

            render_tick.tick().await;
        }
    }

    async fn listen_for_server_updates(
        from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<S2C>>>,
        game_objs_arc: Arc<Mutex<HashMap<usize, GameObjE>>>,
        player_data_arc: Arc<Mutex<PlayerE>>,
        logs_arc: Arc<Mutex<VecDeque<String>>>,
    ) {
        while let Some(msg) = from_server_rx.lock().await.recv().await {
            match msg {
                S2C::L2S4C(L2S4C::GameObjs(objs)) => {
                    *game_objs_arc.lock().await = objs;
                }
                S2C::L2S4C(L2S4C::Player(data)) => {
                    *player_data_arc.lock().await = data;
                }
                S2C::L2S4C(L2S4C::Log(log)) => {
                    logs_arc.lock().await.push_back(log);
                }
                _ => {}
            }
        }
    }

    async fn handle_player_input(
        tx: &mpsc::UnboundedSender<T2C>,
        map_zoom_arc: Arc<Mutex<Option<TermCoord>>>,
        map_look_arc: Arc<Mutex<Option<TermCoord>>>,
        game_objs_arc: Arc<Mutex<HashMap<GameID, GameObjE>>>,
        logs_arc: Arc<Mutex<VecDeque<String>>>,
    ) {
        loop {
            let mut buf = [0u8; 8];
            let n = io::stdin().read(&mut buf).await.unwrap();

            // Helper function for arrow keys player input.
            async fn apply_move(
                map_zoom_arc: &Arc<Mutex<Option<TermCoord>>>,
                map_look_arc: &Arc<Mutex<Option<TermCoord>>>,
                dx: isize,
                dy: isize,
            ) {
                if let Some(ref mut map_look) = *map_look_arc.lock().await {
                    map_look.x = (map_look.x as isize + dx)
                        .max(0)
                        .min(CENTRAL_MODULE_CONTENT_COLS as isize - 1)
                        as usize;
                    map_look.y = (map_look.y as isize + dy)
                        .max(0)
                        .min(CENTRAL_MODULE_CONTENT_ROWS as isize - 1)
                        as usize;
                } else if let Some(ref mut map_zoom) = *map_zoom_arc.lock().await {
                    map_zoom.x = (map_zoom.x as isize + dx)
                        .max(0)
                        .min(MAP_COLS as isize - CENTRAL_MODULE_CONTENT_COLS as isize)
                        as usize;
                    map_zoom.y = (map_zoom.y as isize + dy)
                        .max(0)
                        .min((MAP_ROWS / 2) as isize - CENTRAL_MODULE_CONTENT_ROWS as isize)
                        as usize;
                }
            }

            if n == 1 {
                match buf[0] as char {
                    // quit
                    'q' => {
                        break;
                    }
                    // zoom
                    'z' => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        *map_zoom = match *map_zoom {
                            None => Some(TermCoord { x: 0, y: 0 }),
                            Some(_) => None,
                        };
                    }
                    // look
                    'l' => {
                        let mut map_look = map_look_arc.lock().await;
                        *map_look = match *map_look {
                            None => Some(TermCoord { x: 0, y: 0 }),
                            Some(_) => None,
                        };
                    }
                    // attack
                    'a' => {
                        if let Some(selected_id) = Self::get_selected_obj_id(
                            &*game_objs_arc.lock().await,
                            *map_zoom_arc.lock().await,
                            *map_look_arc.lock().await,
                        ) {
                            let _ = tx.send(T2C::AttackCastle(
                                selected_id,
                                UnitGroupE {
                                    quantities: [1, 0, 0],
                                },
                            ));
                            logs_arc
                                .lock()
                                .await
                                .push_back(format!("Requesting to attack object {}!", selected_id));
                        }
                    }
                    // new castle
                    'n' => {
                        let Some(map_look) = *map_look_arc.lock().await else {
                            return;
                        };
                        let Some(map_zoom) = *map_zoom_arc.lock().await else {
                            return;
                        };
                        let new_castle_coords = GameCoord::from_term_coord(map_look, map_zoom);

                        let _ = tx.send(T2C::NewCastle(new_castle_coords));

                        let mut logs = logs_arc.lock().await;
                        logs.push_back(format!(
                            "Castle added | cooords: ({:?}, {:?})",
                            new_castle_coords.y, new_castle_coords.x
                        ));
                    }
                    _ => {}
                }
            }
            // Special characters
            if n >= 3 {
                match &buf[..n] {
                    // Arrow keys
                    [0x1b, b'[', b'A'] => apply_move(&map_zoom_arc, &map_look_arc, 0, -1).await,
                    [0x1b, b'[', b'B'] => apply_move(&map_zoom_arc, &map_look_arc, 0, 1).await,
                    [0x1b, b'[', b'C'] => apply_move(&map_zoom_arc, &map_look_arc, 2, 0).await,
                    [0x1b, b'[', b'D'] => apply_move(&map_zoom_arc, &map_look_arc, -2, 0).await,

                    // Ctrl + arrows (ESC [ 1 ; 5 X])
                    [0x1b, b'[', b'1', b';', b'5', b'A'] => {
                        apply_move(&map_zoom_arc, &map_look_arc, 0, -16).await
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'B'] => {
                        apply_move(&map_zoom_arc, &map_look_arc, 0, 16).await
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'C'] => {
                        apply_move(&map_zoom_arc, &map_look_arc, 32, 0).await
                    }
                    [0x1b, b'[', b'1', b';', b'5', b'D'] => {
                        apply_move(&map_zoom_arc, &map_look_arc, -32, 0).await
                    }
                    _ => {}
                }
            }
        }
    }

    // --- Utility Functions ---

    pub fn login() -> String {
        let mut input = String::new();
        println!("Login:");
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn get_selected_obj_id(
        game_objs: &HashMap<GameID, GameObjE>,
        map_zoom: Option<TermCoord>,
        map_look: Option<TermCoord>,
    ) -> Option<GameID> {
        if let (Some(zoom), Some(look)) = (map_zoom, map_look) {
            let world_pos = GameCoord::from_term_coord(look, zoom);
            game_objs
                .iter()
                .find(|(_, obj)| obj.get_pos() == world_pos)
                .map(|(id, _)| *id)
        } else {
            None
        }
    }

    fn clear_screen() {
        if cfg!(target_os = "windows") {
            let _ = Command::new("cmd").arg("/c").arg("cls").status();
        } else {
            let _ = Command::new("clear").status();
        }
    }

    fn set_raw_mode() {
        Command::new("stty")
            .arg("raw")
            .arg("-echo")
            .status()
            .expect("Failed to set terminal to raw mode");
    }

    fn reset_mode() {
        Command::new("stty")
            .arg("sane")
            .status()
            .expect("Failed to reset terminal mode");
    }
}
