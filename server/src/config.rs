use std::sync::OnceLock;

use common::{config::load_from, map::Tile};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub server: ServerSection,
    pub lobby: LobbyConfig,
    pub map_gen: MapGenConfig,
}

#[derive(Deserialize)]
pub struct ServerSection {
    pub tick_ms: u64,
}

#[derive(Deserialize)]
pub struct LobbyConfig {
    pub tick_ms: u64,
    pub pool_size: usize,
    pub max_players: usize,
}

#[derive(Deserialize)]
pub struct MapGenConfig {
    pub water: TerrainGenConfig,
    pub woods: TerrainGenConfig,
    pub mountain: TerrainGenConfig,
    pub high_mountain: TerrainGenConfig,
}

#[derive(Deserialize)]
pub struct TerrainGenConfig {
    pub spreading: Tile,
    pub spreads_on: Vec<Tile>,
    pub iterations: usize,
    pub percent: u8,
    pub counts_to_spread: u8,
    pub counts_to_survive: u8,
}

const DEFAULT_PATH: &str = "castli.server.toml";
const ENV_VAR: &str = "CASTLI_SERVER_CONFIG";

static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn config() -> &'static ServerConfig {
    CONFIG.get_or_init(|| load_from(DEFAULT_PATH, ENV_VAR))
}
