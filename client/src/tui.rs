use crate::{
    game_renderer::game_renderer::GameRenderer, input_handler::InputHandler,
    shared_state::SharedState,
};
use common::{
    GameCoord, GameID, L2S4C, S2C,
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE, units::UnitGroupE},
};
use std::{collections::HashMap, io::Write, process::Command, sync::Arc};
use tokio::{
    sync::{Mutex, mpsc},
    time,
};

/// Messages sent from the TUI to the client's network task.
pub enum T2C {
    NewCastle(GameCoord),
    AttackCastle(GameID, UnitGroupE),
    SendUnits(GameCoord, UnitGroupE),
}

pub struct Tui {
    to_server_tx: mpsc::UnboundedSender<T2C>,
    from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<S2C>>>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl Tui {
    pub fn new(
        tx: mpsc::UnboundedSender<T2C>,
        rx: mpsc::UnboundedReceiver<S2C>,
        initial_game_objs: HashMap<GameID, GameObjE>,
        initial_player_data: Option<PlayerE>,
    ) -> Self {
        Self {
            to_server_tx: tx,
            from_server_rx: Arc::new(Mutex::new(rx)),
            shared_state: Arc::new(Mutex::new(SharedState::new(
                initial_game_objs,
                initial_player_data,
            ))),
        }
    }

    pub async fn run(&mut self, tiles: Vec<Vec<TileE>>) {
        Self::set_raw_mode();
        Self::hide_cursor();

        // Spawn a task to listen for updates from the server
        let com_handle = tokio::spawn(Self::listen_for_server_updates(
            Arc::clone(&self.from_server_rx),
            Arc::clone(&self.shared_state),
        ));

        // Spawn a task to render the UI
        let ui_handle = tokio::spawn(Self::render_loop(Arc::clone(&self.shared_state), tiles));

        // Listen to user inputs
        InputHandler::run(&self.to_server_tx, Arc::clone(&self.shared_state)).await;

        // When input handling ends, abort other tasks and clean up.
        com_handle.abort();
        ui_handle.abort();
        Self::clear_screen();
        Self::show_cursor();
        Self::reset_mode();
    }

    async fn render_loop(shared_state: Arc<Mutex<SharedState>>, tiles: Vec<Vec<TileE>>) {
        let mut render_tick = time::interval(time::Duration::from_millis(16));
        let mut last_frame = time::Instant::now();
        let mut frame_dt: u64 = 0;
        let mut game_renderer = GameRenderer::new(tiles);
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
                let mut shared_state = shared_state.lock().await;
                game_renderer.render(&mut shared_state, frame_dt);
                let _ = std::io::stdout().flush();
            }

            render_tick.tick().await;
        }
    }

    async fn listen_for_server_updates(
        from_server_rx: Arc<Mutex<mpsc::UnboundedReceiver<S2C>>>,
        shared_state: Arc<Mutex<SharedState>>,
    ) {
        while let Some(msg) = from_server_rx.lock().await.recv().await {
            let mut state = shared_state.lock().await;
            match msg {
                S2C::L2S4C(L2S4C::GameObjs(objs)) => {
                    state.game_objs = objs;
                }
                S2C::L2S4C(L2S4C::Player(data)) => {
                    state.player_data = data;
                }
                S2C::L2S4C(L2S4C::Log(msg)) => {
                    state.add_log(msg);
                }
                _ => {}
            }
        }
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
