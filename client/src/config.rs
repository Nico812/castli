use std::sync::OnceLock;

use common::config::load_from;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientConfig {
    pub client: ClientSection,
    pub ui: UiConfig,
}

#[derive(Deserialize)]
pub struct ClientSection {
    pub logs_capacity: usize,
}

/// Pure layout values for the renderer. Only base values live in TOML;
/// derived sizes/positions are computed by the methods below.
#[derive(Deserialize)]
pub struct UiConfig {
    pub canvas_rows: usize,
    pub canvas_cols: usize,
    pub zoom_factor: usize,
    pub frame_width: usize,
    pub mod_central_rows: usize,
    pub mod_inspect_cols: usize,
    pub mod_inspect_col: usize,
    pub mod_interact_row_offset: usize,
    pub mod_interact_col_num: usize,
    pub mod_interact_col_den: usize,
}

impl UiConfig {
    pub const MOD_CENTRAL_POS: (usize, usize) = (0, 0);

    pub fn mod_central_cols(&self) -> usize {
        self.canvas_cols
    }

    pub fn fov_rows(&self) -> usize {
        self.mod_central_rows - 4
    }

    pub fn fov_cols(&self) -> usize {
        self.mod_central_cols() - 6
    }

    pub fn mod_player_info_pos(&self) -> (usize, usize) {
        (self.mod_central_rows + 1, 0)
    }

    pub fn mod_player_info_rows(&self) -> usize {
        self.canvas_rows - self.mod_player_info_pos().0
    }

    pub fn mod_player_info_cols(&self) -> usize {
        self.mod_central_cols()
    }

    pub fn mod_inspect_pos(&self) -> (usize, usize) {
        (Self::MOD_CENTRAL_POS.0, self.mod_inspect_col)
    }

    pub fn mod_interact_cols(&self) -> usize {
        self.mod_central_cols() * self.mod_interact_col_num / self.mod_interact_col_den
    }

    pub fn mod_interact_pos(&self) -> (usize, usize) {
        (
            Self::MOD_CENTRAL_POS.0 + self.mod_interact_row_offset,
            Self::MOD_CENTRAL_POS.1 + (self.mod_central_cols() - self.mod_interact_cols()) / 2,
        )
    }
}

const DEFAULT_PATH: &str = "castli.client.toml";
const ENV_VAR: &str = "CASTLI_CLIENT_CONFIG";

static CONFIG: OnceLock<ClientConfig> = OnceLock::new();

pub fn config() -> &'static ClientConfig {
    CONFIG.get_or_init(|| load_from(DEFAULT_PATH, ENV_VAR))
}
