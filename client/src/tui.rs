use crate::{
    camera::{Camera, CameraLocation},
    r#const::CURSOR_SIZE,
    game_state::GameState,
    input_handler::InputHandler,
    renderer::renderer::Renderer,
    shutdown::{ShutdownChannel, ShutdownReason},
    ui_state::UiState,
};
use common::{
    GameCoord, GameId,
    r#const::{MAX_LOBBIES, MAX_LOBBY_PLAYERS},
    courtyard::{Facility, FacilityType},
    game_objs::GameObjE,
    units::UnitGroup,
};
use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    style::Stylize,
    terminal,
};
use std::{
    collections::HashSet,
    io::{self, Stdout, Write},
    ops::DerefMut,
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
    InCourtyard,
    OutCourtyard,
    NewFacility((GameCoord, FacilityType)),
}

pub struct Tui {
    stdout: Stdout,
    tx: mpsc::UnboundedSender<T2C>,
    shutdown: ShutdownChannel,
}

impl Tui {
    pub async fn new(tx: mpsc::UnboundedSender<T2C>, shutdown: ShutdownChannel) -> Self {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode().expect("Failed to set terminal to raw mode");
        let _ = stdout.execute(cursor::Hide);
        let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));

        Self {
            stdout,
            tx,
            shutdown,
        }
    }

    pub fn login(&mut self) -> String {
        let mut username = String::new();

        loop {
            self.clear_screen();

            let _ = self.stdout.execute(cursor::MoveTo(2, 1));
            print!("{}", "Login");

            let _ = self.stdout.execute(cursor::MoveTo(4, 3));
            print!("Username: {}_", username);

            let _ = self.stdout.execute(cursor::MoveTo(4, 5));
            print!("Type your username | enter: confirm");

            let _ = self.stdout.flush();

            match read() {
                Ok(Event::Key(key)) if key.kind != KeyEventKind::Release => match key.code {
                    KeyCode::Char(character) => {
                        username.push(character);
                    }
                    KeyCode::Backspace => {
                        username.pop();
                    }
                    KeyCode::Enter if !username.trim().is_empty() => {
                        break;
                    }
                    _ => {}
                },

                _ => {}
            }
        }

        self.clear_screen();
        let _ = self.stdout.execute(cursor::MoveTo(0, 0));

        username
    }

    pub async fn choose_lobby(&mut self, lobbies_info: Result<[usize; MAX_LOBBIES], ()>) -> usize {
        let mut selected = 0;

        let choice = loop {
            self.clear_screen();
            let _ = self.stdout.execute(cursor::MoveTo(2, 1));
            print!("Choose lobby");

            for i in 0..MAX_LOBBIES {
                let players_str = match lobbies_info {
                    Ok(lobbies_info) => lobbies_info[i].to_string(),
                    Err(()) => "Err".to_string(),
                };

                let _ = self.stdout.execute(cursor::MoveTo(4, 3 + i as u16));
                if i == selected {
                    print!(
                        "{}",
                        format!(
                            "> Lobby {}; players: {}/{}",
                            i, players_str, MAX_LOBBY_PLAYERS
                        )
                        .yellow()
                    );
                } else {
                    print!(
                        "  Lobby {}; players: {}/{}",
                        i, players_str, MAX_LOBBY_PLAYERS
                    );
                }
            }
            let _ = self
                .stdout
                .execute(cursor::MoveTo(2, 4 + MAX_LOBBIES as u16));

            print!("arrows: move | enter: confirm");
            let _ = self.stdout.flush();

            match read() {
                Ok(Event::Key(key)) if key.kind != KeyEventKind::Release => match key.code {
                    KeyCode::Up => selected = selected.saturating_sub(1),
                    KeyCode::Down => selected = (selected + 1).min(MAX_LOBBIES - 1),
                    KeyCode::Enter => break selected,
                    _ => {}
                },
                _ => {}
            }
        };

        self.clear_screen();
        let _ = self.stdout.execute(cursor::MoveTo(0, 0));

        choice
    }

    pub async fn run_game(&mut self, game_state: Arc<Mutex<GameState>>) {
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;

        let mut renderer = match Renderer::new() {
            Ok(renderer) => renderer,
            Err(_) => {
                self.shutdown.shutdown(ShutdownReason::TermSize);
                return;
            }
        };
        let mut ui_state = UiState::new(renderer.fov_size(), renderer.zoom_factor());

        while !self.shutdown.is_shutdown() {
            let mut game_guard = game_state.lock().await;
            let game_state = game_guard.deref_mut();

            while let Ok(true) = poll(Duration::from_millis(0)) {
                if let Ok(event) = read() {
                    InputHandler::handle_input(
                        &event,
                        &self.tx,
                        game_state,
                        &mut ui_state,
                        ShutdownChannel::clone(&self.shutdown),
                    );
                }
            }

            // Cap render fps and skip frames where dt is suspiciously short.
            let now = time::Instant::now();
            let dt = now.duration_since(last_frame).as_millis() as u64;
            if dt >= 10 {
                frame_dt = dt;
            };
            last_frame = now;

            renderer.render(&mut self.stdout, game_state, &mut ui_state, frame_dt);

            render_tick.tick().await;
        }
        self.clear_screen();
    }

    pub fn get_looked_objs<'a>(
        game_state: &'a GameState,
        coord: GameCoord,
        camera: &Camera,
    ) -> Vec<(GameId, &'a GameObjE)> {
        let mut looked_objs_ids: HashSet<GameId> = HashSet::new();
        let mut looked_objs: Vec<(GameId, &GameObjE)> = Vec::new();

        let (cursor_bounds_y, cursor_bounds_x) = if camera.location == CameraLocation::WorldMap {
            (
                0..CURSOR_SIZE.y * camera.zoom_factor,
                0..CURSOR_SIZE.x * camera.zoom_factor,
            )
        } else {
            (0..CURSOR_SIZE.y, 0..CURSOR_SIZE.x)
        };

        for i in cursor_bounds_y {
            for j in cursor_bounds_x.clone() {
                if let Some(id) = game_state.hitbox_map[coord.y + i][coord.x + j] {
                    looked_objs_ids.insert(id);
                }
            }
        }

        for id in looked_objs_ids {
            looked_objs.push((
                id,
                game_state
                    .get_obj(id)
                    .expect("Looking at unexistent object."),
            ));
        }

        looked_objs.sort_by(|a, b| a.0.cmp(&b.0));
        looked_objs.sort_by_key(|a| match a.1 {
            GameObjE::Castle(_) => 0,
            GameObjE::Structure(_) => 1,
            GameObjE::DeployedUnits(_) => 2,
        });
        looked_objs
    }

    pub fn get_looked_facility<'a>(
        game_state: &'a GameState,
        coord: GameCoord,
        camera: &Camera,
    ) -> Option<(usize, &'a Facility)> {
        let mut looked_facility_id: Option<usize> = None;
        let looked_facility: Option<(usize, &Facility)>;

        let camera_pos = camera.get_pos();

        let cursor_bounds_y = 0..CURSOR_SIZE.y;
        let cursor_bounds_x = 0..CURSOR_SIZE.x;

        for i in cursor_bounds_y {
            for j in cursor_bounds_x.clone() {
                if let Some(id) = game_state.courtyard_hitbox_map[coord.y + camera_pos.y + i]
                    [coord.x + camera_pos.x + j]
                {
                    looked_facility_id = Some(id);
                }
            }
        }

        looked_facility = looked_facility_id.map(|id| {
            (
                id,
                game_state
                    .get_facility(id)
                    .expect("Looking at an unexistent faility."),
            )
        });

        looked_facility
    }

    fn clear_screen(&mut self) {
        let _ = execute!(self.stdout, terminal::Clear(terminal::ClearType::All));
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.stdout.execute(cursor::MoveTo(0, 0));
        let _ = self.stdout.execute(cursor::Show);
        terminal::disable_raw_mode().expect("Failed to reset terminal mode");
    }
}
