use crate::{
    client::{GameState, ShutdownChannel},
    input_handler::InputHandler,
    renderer::renderer::Renderer,
    shared_state::UiState,
};
use common::{
    GameCoord, GameID, L2S4C, S2C,
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE, units::UnitGroupE},
};
use std::{collections::HashMap, io::Write, net::Shutdown, process::Command, sync::Arc};
use tokio::{
    net::unix::pipe::Sender,
    sync::{Mutex, mpsc},
    time,
};

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle(GameCoord),
    AttackCastle(GameID, UnitGroupE),
    SendUnits(GameCoord, UnitGroupE),
}

pub struct Tui {}

impl Tui {
    pub async fn run(
        tx: mpsc::UnboundedSender<T2C>,
        game_state: Arc<Mutex<GameState>>,
        shutdown: ShutdownChannel,
    ) {
        Self::set_raw_mode();
        Self::hide_cursor();

        let ui_state = Arc::new(Mutex::new(UiState::new()));

        // Spawn a task to listen to user inputs
        let input_handle = tokio::spawn(InputHandler::run(
            tx,
            Arc::clone(&game_state),
            Arc::clone(&ui_state),
            ShutdownChannel::clone(&shutdown),
        ));

        // Render loop
        Self::render_loop(game_state, ui_state, shutdown).await;
        // When input handling ends, abort other tasks and clean up.
        input_handle.abort();
        Self::clear_screen();
        Self::show_cursor();
        Self::reset_mode();
    }

    async fn render_loop(
        game_state: Arc<Mutex<GameState>>,
        ui_state: Arc<Mutex<UiState>>,
        shutdown: ShutdownChannel,
    ) {
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;

        let map = game_state.lock().await.map.clone();
        let mut renderer = match Renderer::new(map) {
            Ok(renderer) => renderer,
            Err(_) => {
                shutdown.shutdown();
                return;
            }
        };

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
                let game_state = game_state.lock().await;
                let mut ui_state = ui_state.lock().await;
                renderer.render(&game_state, &mut ui_state, frame_dt);
                let _ = std::io::stdout().flush();
            }

            render_tick.tick().await;
        }
    }

    pub fn get_looked_objs<'a>(
        coord: GameCoord,
        zoom: &Option<GameCoord>,
        game_objs: &'a HashMap<GameID, GameObjE>,
    ) -> Vec<(GameID, &'a GameObjE)> {
        let mut looked_objs: Vec<(GameID, &GameObjE)> = game_objs
            .iter()
            .filter_map(|(game_id, game_obj)| {
                if (zoom.is_some() && game_obj.get_pos() == coord)
                    || (zoom.is_none()
                        && game_obj.get_pos().y >= coord.y
                        && game_obj.get_pos().x >= coord.x
                        && game_obj.get_pos().y < coord.y + Renderer::ZOOM_FACTOR
                        && game_obj.get_pos().x < coord.x + Renderer::ZOOM_FACTOR)
                {
                    Some((*game_id, game_obj))
                } else {
                    None
                }
            })
            .collect();

        looked_objs.sort_by(|a, b| a.0.cmp(&b.0));
        looked_objs.sort_by_key(|a| match a.1 {
            GameObjE::Castle(_) => 0,
            GameObjE::Structure(_) => 1,
            GameObjE::DeployedUnits(_) => 2,
        });
        looked_objs
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

    fn hide_cursor() {
        print!("\x1b[?25l");
    }

    fn show_cursor() {
        print!("\x1b[?25h");
    }
}
