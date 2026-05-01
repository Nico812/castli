use crate::{
    client::{ShutdownChannel, ShutdownReason},
    game_state::GameState,
    input_handler::InputHandler,
    renderer::renderer::Renderer,
    ui_state::UiState,
};
use common::{GameCoord, GameId, r#const::MAX_LOBBIES, game_objs::GameObjE, units::UnitGroup};
use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, poll, read},
    terminal,
};
use std::{
    collections::HashMap,
    io::{self, Stdout},
    ops::DerefMut,
    process::Command,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{Mutex, mpsc},
    time,
};

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle(GameCoord),
    AttackCastle(GameId, UnitGroup),
    SendUnits(GameCoord, UnitGroup),
}

pub struct Tui;

impl Tui {
    pub async fn run(
        tx: mpsc::UnboundedSender<T2C>,
        game_state: Arc<Mutex<GameState>>,
        shutdown: ShutdownChannel,
    ) {
        let mut stdout = io::stdout();
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;
        let map = game_state.lock().await.map.clone();
        let mut ui_state = UiState::new();

        let mut renderer = match Renderer::new(map) {
            Ok(renderer) => renderer,
            Err(_) => {
                shutdown.shutdown(ShutdownReason::TermSize);
                return;
            }
        };

        Self::set_raw_mode();
        Self::hide_cursor(&mut stdout);
        Self::clear_screen();

        while !shutdown.is_shutdown() {
            // Convert the MutexGuard into &mut.
            // This helps the borrow checker perform field-level borrowing more precisely
            // (borrow splitting works better on &mut T than on MutexGuard<T> in complex flows).
            let mut game_guard = game_state.lock().await;
            let game_state = game_guard.deref_mut();

            while let Ok(true) = poll(Duration::from_millis(0)) {
                if let Ok(Event::Key(key)) = read() {
                    InputHandler::handle_key(
                        &key,
                        &tx,
                        game_state,
                        &mut ui_state,
                        ShutdownChannel::clone(&shutdown),
                    );
                }
            }

            // Rendering fps
            // There's a problem that the frames can go really fast when there is delay
            // so i take only the frames with a reasonable high dt.
            let now = time::Instant::now();
            let dt = now.duration_since(last_frame).as_millis() as u64;
            if dt >= 10 {
                frame_dt = dt;
            };
            last_frame = now;

            renderer.render(&mut stdout, &game_state, &mut ui_state, frame_dt);

            render_tick.tick().await;
        }

        Self::clear_screen();
        Self::show_cursor(&mut stdout);
        Self::reset_mode();
    }

    pub fn get_looked_objs<'a>(
        coord: GameCoord,
        game_objs: &'a HashMap<GameId, GameObjE>,
        in_world_map: bool,
    ) -> Vec<(GameId, &'a GameObjE)> {
        let mut looked_objs: Vec<(GameId, &GameObjE)> = game_objs
            .iter()
            .filter_map(|(game_id, game_obj)| {
                if (!in_world_map && game_obj.get_pos() == coord)
                    || (in_world_map
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
        println!("Login:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn choose_lobby() -> usize {
        println!("Choose lobby (0..{}):", MAX_LOBBIES - 1);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().expect("Invalid lobby number")
    }

    fn clear_screen() {
        if cfg!(target_os = "windows") {
            let _ = Command::new("cmd").arg("/c").arg("cls").status();
        } else {
            let _ = Command::new("clear").status();
        }
    }

    fn set_raw_mode() {
        terminal::enable_raw_mode().expect("Failed to set terminal to raw mode");
    }

    fn reset_mode() {
        terminal::disable_raw_mode().expect("Failed to reset terminal mode");
    }

    fn hide_cursor(stdout: &mut Stdout) {
        let _ = stdout.execute(cursor::Hide);
    }

    fn show_cursor(stdout: &mut Stdout) {
        let _ = stdout.execute(cursor::Show);
    }
}
