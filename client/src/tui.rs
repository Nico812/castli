//! # Terminal User Interface (TUI)
//!
//! This module manages the entire terminal user interface. It handles raw mode,
//! rendering the game state to the terminal using the `canvas`, and processing
//! player input.

use std::{collections::HashMap, io::Write, process::Command, sync::Arc};
use tokio::{
    io::{self, AsyncReadExt},
    sync::{mpsc, Mutex},
};

use crate::canvas;
use common;

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle((usize, usize)),
}

enum TuiState {
    InGame,
    CastleCreation,
}

/// The Tui struct now holds all the state required for the UI to function.
pub struct Tui {
    // Tui state
    state: TuiState,
    // Communication channels
    to_server_tx: mpsc::UnboundedSender<T2C>,
    from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<common::S2C>>>,

    // UI and Game State
    canvas: canvas::Canvas,
    game_objs: Arc<Mutex<HashMap<usize, common::GameObjE>>>,
    player_data: Arc<Mutex<common::PlayerDataE>>,
    map_zoom: Arc<Mutex<Option<(usize, usize)>>>,
    map_look: Arc<Mutex<Option<(usize, usize)>>>,
}

impl Tui {
    /// The constructor is now simple and synchronous. Its only job is to
    /// create the Tui instance with the initial state provided by the client.
    pub fn new(
        tx: mpsc::UnboundedSender<T2C>,
        rx: mpsc::UnboundedReceiver<common::S2C>,
        tiles: Vec<Vec<common::TileE>>,
        initial_game_objs: HashMap<usize, common::GameObjE>,
        initial_player_data: Option<common::PlayerDataE>,
    ) -> Self {
        let mut canvas = canvas::Canvas::new();
        canvas.init(&tiles);
        let tui_state = TuiState::InGame;
        let map_look = None;
        let map_zoom = Some((0, 0));

        let player_data = match initial_player_data {
            Some(player_data) => player_data,
            None => {
                tui_state = TuiState::CastleCreation;
                map_look = Some((0, 0));
                common::L2S4C::PlayerData{id: 0, name: "Undefined".to_string(), pos: (0, 0),}
            }
        }

        Self {
            tui_state,
            to_server_tx: tx,
            from_server_rx: Arc::new(Mutex::new(rx)),
            canvas,
            game_objs: Arc::new(Mutex::new(initial_game_objs)),
            player_data: Arc::new(Mutex::new(initial_player_data)),
            map_zoom: Arc::new(Mutex::new(map_zoom)),
            map_look: Arc::new(Mutex::new(map_look)),
        }
    }

    /// The run method is now the main entry point for the TUI. It takes
    /// ownership of `self` and runs until the user quits.
    pub async fn run(self) {
        Self::set_raw_mode();

        // Spawn a task to listen for updates from the server
        let com_handle = tokio::spawn(Self::listen_for_server_updates(
            Arc::clone(&self.from_server_rx),
            Arc::clone(&self.game_objs),
            Arc::clone(&self.player_data),
        ));

        // Spawn a task to render the UI
        let ui_handle = tokio::spawn(Self::render_loop(
            self.canvas,
            Arc::clone(&self.game_objs),
            Arc::clone(&self.player_data),
            Arc::clone(&self.map_zoom),
            Arc::clone(&self.map_look),
        ));

        // The main TUI task now only handles player input
        Self::handle_player_input(self.to_server_tx, self.map_zoom, self.map_look).await;

        // When input handling ends (e.g., user presses 'q'), abort other tasks and clean up.
        com_handle.abort();
        ui_handle.abort();
        Self::reset_mode();
    }

    /// The UI rendering loop, now in its own focused async function.
    async fn render_loop(
        canvas: canvas::Canvas,
        game_objs_arc: Arc<Mutex<HashMap<usize, common::GameObjE>>>,
        player_data_arc: Arc<Mutex<common::PlayerDataE>>,
        map_zoom_arc: Arc<Mutex<Option<(usize, usize)>>>,
        map_look_arc: Arc<Mutex<Option<(usize, usize)>>>,
    ) {
        loop {
            // Lock resources only for the brief moment they are needed for drawing
            let game_objs = game_objs_arc.lock().await;
            let player_data = player_data_arc.lock().await;
            let map_zoom = map_zoom_arc.lock().await;
            let map_look = map_look_arc.lock().await;

            Self::clear_screen();
            canvas.print(&game_objs, &player_data, *map_zoom);
            canvas.update_and_print_cursor(*map_look);
            let _ = std::io::stdout().flush();
            
            // Drop locks automatically here at the end of the scope

            tokio::time::sleep(tokio::time::Duration::from_millis(1000 / 60)).await;
        }
    }

    async fn listen_for_server_updates(
        from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<common::S2C>>>,
        game_objs_arc: Arc<Mutex<HashMap<usize, common::GameObjE>>>,
        player_data_arc: Arc<Mutex<common::PlayerDataE>>,
    ) {
        while let Some(msg) = from_server_rx.lock().await.recv().await {
            match msg {
                common::S2C::L2S4C(common::L2S4C::GameObjs(objs)) => {
                    *game_objs_arc.lock().await = objs;
                }
                common::S2C::L2S4C(common::L2S4C::PlayerData(data)) => {
                    *player_data_arc.lock().await = data;
                }
                _ => {}
            }
        }
    }

    async fn handle_player_input(
        tx: mpsc::UnboundedSender<T2C>,
        map_zoom_arc: Arc<Mutex<Option<(usize, usize)>>>,
        map_look_arc: Arc<Mutex<Option<(usize, usize)>>>,
    ) {
        loop {
            let mut buf = [0u8; 3];
            let n = io::stdin().read(&mut buf).await.unwrap();
            if n == 1 {
                match buf[0] as char {
                    'q' => {
                        break;
                    }
                    'z' => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        *map_zoom = match *map_zoom {
                            None => Some((0, 0)),
                            Some(_) => None,
                        };
                    }
                    'l' => {
                        let mut map_look = map_look_arc.lock().await;
                        *map_look = match *map_look {
                            None => Some((0, 0)),
                            Some(_) => None,
                        };
                    }
                    'a' => {
                        println!("CACCAAAAA");
                        let _ = tx.send(T2C::NewCastle((4, 4)));
                    }
                    _ => {}
                }
            }
            // Special characters
            if n == 3 {
                match buf {
                    // Arrow keys
                    [0x1b, b'[', b'C'] => {
                        let mut map_look = map_look_arc.lock().await;
                        if let Some((row, col)) = *map_look {
                            *map_look = Some((
                                row,
                                std::cmp::min(col + 1, crate::r#const::CENTRAL_MODULE_COLS - 1),
                            ));
                        } else {
                            let mut map_zoom = map_zoom_arc.lock().await;
                            if let Some((row, col)) = *map_zoom {
                                *map_zoom = Some((row, std::cmp::min(col + 1, 7)));
                            }
                        }
                    }
                    [0x1b, b'[', b'D'] => {
                        let mut map_look = map_look_arc.lock().await;
                        if let Some((row, col)) = *map_look {
                            *map_look = Some((row, col.saturating_sub(1)));
                        } else {
                            let mut map_zoom = map_zoom_arc.lock().await;
                            if let Some((row, col)) = *map_zoom {
                                *map_zoom = Some((row, col.saturating_sub(1)));
                            }
                        }
                    }
                    [0x1b, b'[', b'A'] => {
                        let mut map_look = map_look_arc.lock().await;
                        if let Some((row, col)) = *map_look {
                            *map_look = Some((row.saturating_sub(1), col));
                        } else {
                            let mut map_zoom = map_zoom_arc.lock().await;
                            if let Some((row, col)) = *map_zoom {
                                *map_zoom = Some((row.saturating_sub(1), col));
                            }
                        }
                    }
                    [0x1b, b'[', b'B'] => {
                        let mut map_look = map_look_arc.lock().await;
                        if let Some((row, col)) = *map_look {
                            *map_look = Some((
                                std::cmp::min(row + 1, crate::r#const::CENTRAL_MODULE_ROWS - 1),
                                col,
                            ));
                        } else {
                            let mut map_zoom = map_zoom_arc.lock().await;
                            if let Some((row, col)) = *map_zoom {
                                *map_zoom = Some((std::cmp::min(row + 1, 7), col));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    // --- Utility Functions ---

    pub fn login() -> String {
        // ... (remains the same)
        let mut input = String::new();
        println!("Login:");
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn clear_screen() {
        // ... (remains the same)
        if cfg!(target_os = "windows") {
            let _ = Command::new("cmd").arg("/c").arg("cls").status();
        } else {
            let _ = Command::new("clear").status();
        }
    }

    fn set_raw_mode() {
        // ... (remains the same)
        Command::new("stty")
            .arg("raw")
            .arg("-echo")
            .status()
            .expect("Failed to set terminal to raw mode");
    }

    fn reset_mode() {
        // ... (remains the same)
        Command::new("stty")
            .arg("sane")
            .status()
            .expect("Failed to reset terminal mode");
    }
}