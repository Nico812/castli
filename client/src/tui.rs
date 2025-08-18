//! # Terminal User Interface (TUI)
//!
//! This module manages the entire terminal user interface. It handles raw mode,
//! rendering the game state to the terminal using the `canvas`, and processing
//! player input.

use std::{collections::HashMap, io::Write, process::Command};
use tokio::{
    io::{self, AsyncReadExt},
    sync::mpsc,
};

use crate::canvas;
use common::{self, stream};

pub enum T2C {
    NewCastle((usize, usize)),
}

pub struct Tui {}

impl Tui {
    pub async fn new(
        tx: mpsc::UnboundedSender<T2C>,
        mut rx: mpsc::UnboundedReceiver<common::S2C>,
        tiles: Vec<Vec<common::TileE>>,
    ) -> Self {
        let map_zoom = Some((0, 0));
        let map_look = None;
        let mut game_objs = None;
        let mut player_data = None;

        // Waiting first required data from the server
        while game_objs.is_none() {
            match rx.recv().await.unwrap() {
                common::S2C::L2S4C(common::L2S4C::GameObjs(objs)) => {
                    println!("got objs");
                    game_objs = Some(objs);
                }
                _ => {}
            }
        }

        let _ = tx.send(T2C::NewCastle((1, 3)));

        while player_data.is_none() {
            match rx.recv().await.unwrap() {
                common::S2C::L2S4C(common::L2S4C::PlayerData(data)) => {
                    println!("got player data");
                    player_data = Some(data);
                }
                _ => {}
            }
        }

        Self::set_raw_mode();
        Self::run(
            tx,
            rx,
            tiles,
            map_zoom,
            map_look,
            game_objs.unwrap(),
            player_data.unwrap(),
        )
        .await;
        Self {}
    }

    /// Runs the main TUI loop.
    ///
    /// This function spawns tasks for rendering the UI and handling communication
    /// with the main client logic. It also contains the player input loop.
    async fn run(
        tx: mpsc::UnboundedSender<T2C>,
        mut rx: mpsc::UnboundedReceiver<common::S2C>,
        tiles: Vec<Vec<common::TileE>>,
        map_zoom: Option<(usize, usize)>,
        map_look: Option<(usize, usize)>,
        game_objs: HashMap<usize, common::GameObjE>,
        player_data: common::PlayerDataE,
    ) {
        let map_zoom_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(map_zoom));
        let map_zoom_arc1 = std::sync::Arc::clone(&map_zoom_arc0);
        let map_look_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(map_look));
        let map_look_arc1 = std::sync::Arc::clone(&map_look_arc0);

        let game_objs_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(game_objs));
        let game_objs_arc1 = std::sync::Arc::clone(&game_objs_arc0);
        let player_data_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(player_data));
        let player_data_arc1 = std::sync::Arc::clone(&player_data_arc0);

        // Actual UI
        let ui_handle = tokio::spawn(async move {
            let mut canvas = canvas::Canvas::new();
            canvas.init(&tiles);

            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000 / 60)).await;

                let game_objs = game_objs_arc0.lock().await;
                let player_data = player_data_arc0.lock().await;
                let map_zoom = map_zoom_arc0.lock().await;
                let map_look = map_look_arc0.lock().await;

                Self::clear_screen();
                canvas.print(&*game_objs, &*player_data, *map_zoom);
                canvas.update_and_print_cursor(*map_look);
                let _ = std::io::stdout().flush();
            }
        });

        // Comunication with client uberstruct
        let com_handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    common::S2C::L2S4C(common::L2S4C::GameObjs(objs)) => {
                        *game_objs_arc1.lock().await = objs;
                    }
                    common::S2C::L2S4C(common::L2S4C::PlayerData(data)) => {
                        *player_data_arc1.lock().await = data;
                    }
                    _ => {}
                }
            }
        });

        Self::handle_player_input(tx, map_zoom_arc1, map_look_arc1).await;

        let _ = com_handle.abort();
        let _ = ui_handle.abort();

        Self::reset_mode();
    }

    pub fn login() -> String {
        let mut input = String::new();

        println!("Login:");

        std::io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
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

    async fn handle_player_input(
        tx: mpsc::UnboundedSender<T2C>,
        map_zoom_arc: std::sync::Arc<tokio::sync::Mutex<Option<(usize, usize)>>>,
        map_look_arc: std::sync::Arc<tokio::sync::Mutex<Option<(usize, usize)>>>,
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
}
