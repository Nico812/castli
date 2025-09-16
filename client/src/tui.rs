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

use crate::{
    canvas::canvas::Canvas,
    r#const::{QUADRANT_COLS, QUADRANT_ROWS},
};
use common::{GameID, GameObjE, L2S4C, PlayerE, S2C, TileE};

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle((usize, usize)),
    AttackCastle(GameID),
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
    map_zoom: Arc<Mutex<Option<(usize, usize)>>>,
    map_look: Arc<Mutex<Option<(usize, usize)>>>,
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
                PlayerE {
                    id: 0,
                    name: "Undefined".to_string(),
                    pos: (0, 0),
                }
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
        map_zoom_arc: Arc<Mutex<Option<(usize, usize)>>>,
        map_look_arc: Arc<Mutex<Option<(usize, usize)>>>,
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
        map_zoom_arc: Arc<Mutex<Option<(usize, usize)>>>,
        map_look_arc: Arc<Mutex<Option<(usize, usize)>>>,
        logs_arc: Arc<Mutex<VecDeque<String>>>,
    ) {
        loop {
            let mut buf = [0u8; 3];
            let n = io::stdin().read(&mut buf).await.unwrap();
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
                            None => Some((0, 0)),
                            Some(_) => None,
                        };
                    }
                    // look
                    'l' => {
                        let mut map_look = map_look_arc.lock().await;
                        *map_look = match *map_look {
                            None => Some((0, 0)),
                            Some(_) => None,
                        };
                    }
                    // attack
                    'a' => {
                        if let Some(selected_id) =
                            Self::get_selected_obj_id(&game_objs_arc.lock().await, *map_zoom_arc.lock().await, *map_look_arc.lock().await)
                        {
                            let _ = tx.send(T2C::AttackCastle(selected_id));
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
                        let new_castle_coords = (
                            (map_zoom.0 * QUADRANT_ROWS + map_look.0) * 2,
                            map_zoom.1 * QUADRANT_COLS + map_look.1,
                        );

                        let _ = tx.send(T2C::NewCastle(new_castle_coords));

                        let mut logs = logs_arc.lock().await;
                        logs.push_back(format!(
                            "Castle added | cooords: ({:?}, {:?})",
                            new_castle_coords.0, new_castle_coords.1
                        ));
                    }
                    _ => {}
                }
            }
            // Special characters
            if n == 3 {
                match buf {
                    // Up Arrow Key
                    [0x1b, b'[', b'A'] => {
                        if let Some(ref mut map_look) = *map_look_arc.lock().await {
                            map_look.0 = map_look.0.saturating_sub(1);
                        } else if let Some(ref mut map_zoom) = *map_zoom_arc.lock().await {
                            map_zoom.0 = map_zoom.0.saturating_sub(1);
                        }
                    }
                    // Down Arrow Key
                    [0x1b, b'[', b'B'] => {
                        if let Some(ref mut map_look) = *map_look_arc.lock().await {
                            map_look.0 = std::cmp::min(map_look.0 + 1, QUADRANT_ROWS - 1);
                        } else if let Some(ref mut map_zoom) = *map_zoom_arc.lock().await {
                            map_zoom.0 = std::cmp::min(map_zoom.0 + 1, 7);
                        }
                    }
                    // Left Arrow Key
                    [0x1b, b'[', b'D'] => {
                        if let Some(ref mut map_look) = *map_look_arc.lock().await {
                            map_look.1 = map_look.1.saturating_sub(1);
                        } else if let Some(ref mut map_zoom) = *map_zoom_arc.lock().await {
                            map_zoom.1 = map_zoom.1.saturating_sub(1);
                        }
                    }
                    // Right Arrow Key
                    [0x1b, b'[', b'C'] => {
                        if let Some(ref mut map_look) = *map_look_arc.lock().await {
                            map_look.1 = std::cmp::min(map_look.1 + 1, QUADRANT_COLS - 1);
                        } else if let Some(ref mut map_zoom) = *map_zoom_arc.lock().await {
                            map_zoom.1 = std::cmp::min(map_zoom.1 + 1, 7);
                        }
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
        map_zoom: Option<(usize, usize)>,
        map_look: Option<(usize, usize)>,
    ) -> Option<GameID> {
        if let (Some(zoom), Some(look)) = (map_zoom, map_look) {
            let world_pos = (
                (zoom.0 * QUADRANT_ROWS + look.0) * 2,
                zoom.1 * QUADRANT_COLS + look.1,
            );
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
