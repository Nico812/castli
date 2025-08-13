//! # Terminal User Interface (TUI)
//!
//! This module manages the entire terminal user interface. It handles raw mode,
//! rendering the game state to the terminal using the `canvas`, and processing
//! player input.

use std::{collections::HashMap, io::Write, process::Command};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};

use crate::canvas;
use common;

pub enum PlayerInput {
    Caccaaa,
}

pub struct Tui {}

impl Tui {
    pub fn new() -> Self {
        Self::set_raw_mode();
        Self {}
    }

    /// Runs the main TUI loop.
    ///
    /// This function spawns tasks for rendering the UI and handling communication
    /// with the main client logic. It also contains the player input loop.
    pub async fn run(
        tx: mpsc::UnboundedSender<PlayerInput>,
        mut rx: mpsc::UnboundedReceiver<common::S2C>,
        tiles: Vec<Vec<common::TileE>>,
    ) {
        let mut completed = false;
        let mut game_objs: Option<HashMap<u64, common::GameObj>> = None;
        let mut player_data: Option<common::PlayerData> = None; 

        let map_zoom: Option<(usize, usize)> = Some((0, 0));
        while !completed {
            match rx.recv().await.unwrap() {
                common::S2C::L2S4C(common::L2S4C::GameObjs(objs)) => game_objs = Some(objs),
                common::S2C::L2S4C(common::L2S4C::PlayerData(data)) => player_data = Some(data),
                _ => {}
            }
            completed = game_objs.is_some() && player_data.is_some();
        }

        let game_objs_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(game_objs.unwrap()));
        let game_objs_arc1 = std::sync::Arc::clone(&game_objs_arc0);

        let player_data_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(player_data.unwrap()));
        let player_data_arc1 = std::sync::Arc::clone(&player_data_arc0);

        let map_zoom_arc0 = std::sync::Arc::new(tokio::sync::Mutex::new(map_zoom));
        let map_zoom_arc1 = std::sync::Arc::clone(&map_zoom_arc0);

        // Actual UI
        let ui_handle = tokio::spawn(async move {
            let mut canvas = canvas::Canvas::new();
            canvas.init(&tiles);

            loop {
                let game_objs = game_objs_arc0.lock().await;
                let player_data = player_data_arc0.lock().await;
                let map_zoom = map_zoom_arc0.lock().await;
                Self::clear_screen();
                canvas.print(&*game_objs, &*player_data, &*map_zoom);

                print!("\r\x1b[0;0H");
                let _ = std::io::stdout().flush();

                tokio::time::sleep(tokio::time::Duration::from_millis(1000 / 60)).await;
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

        Self::handle_player_input(tx, map_zoom_arc1).await;

        let _ = com_handle.abort();
        let _ = ui_handle.abort();

        Self::reset_mode();
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
        tx: mpsc::UnboundedSender<PlayerInput>,
        map_zoom_arc: std::sync::Arc<tokio::sync::Mutex<Option<(usize, usize)>>>,
    ) {
        loop {
            let mut buf = [0u8; 3];
            let n = io::stdin().read(&mut buf).await.unwrap();
            if n == 1 {
                match buf[0] as char {
                    'a' => {
                        println!("CACCAAAAA");
                        let _ = tx.send(PlayerInput::Caccaaa);
                    }
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
                    _ => {}
                }
            }
            if n == 3 {
                match buf {
                    [0x1b, b'[', b'C'] => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        if let Some((row, col)) = *map_zoom {
                            *map_zoom = Some((row, std::cmp::min(col + 1, 7)));
                        }
                    }
                    [0x1b, b'[', b'D'] => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        if let Some((row, col)) = *map_zoom {
                            *map_zoom = Some((row, col.saturating_sub(1)));
                        }
                    }
                    [0x1b, b'[', b'A'] => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        if let Some((row, col)) = *map_zoom {
                            *map_zoom = Some((row.saturating_sub(1), col));
                        }
                    }
                    [0x1b, b'[', b'B'] => {
                        let mut map_zoom = map_zoom_arc.lock().await;
                        if let Some((row, col)) = *map_zoom {
                            *map_zoom = Some((std::cmp::min(row + 1, 7), col));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
