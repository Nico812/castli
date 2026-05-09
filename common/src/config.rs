use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub world: WorldConfig,
    pub server: ServerConfig,
    pub lobby: LobbyConfig,
    pub units: UnitsConfig,
    pub client: ClientConfig,
    pub map_gen: MapGenConfig,
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub address: String,
    pub online: bool,
}

#[derive(Deserialize)]
pub struct WorldConfig {
    pub map_rows: usize,
    pub map_cols: usize,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub tick_ms: u64,
}

#[derive(Deserialize)]
pub struct LobbyConfig {
    pub max_players: usize,
    pub tick_ms: u64,
    pub pool_size: usize,
}

#[derive(Deserialize)]
pub struct UnitsConfig {
    pub knight_strength: u32,
    pub mage_strength: u32,
    pub dragon_strength: u32,
    pub ship_strength: u32,
}

#[derive(Deserialize)]
pub struct ClientConfig {
    pub logs_capacity: usize,
    pub debug_mode: bool,
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
    pub iterations: usize,
    pub percent: u8,
    pub counts_to_spread: u8,
    pub counts_to_survive: u8,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

const DEFAULT_PATH: &str = "castli.toml";
const ENV_VAR: &str = "CASTLI_CONFIG";

pub fn config() -> &'static Config {
    CONFIG.get_or_init(load)
}

fn load() -> Config {
    let path = std::env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_PATH.to_string());
    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read config at {}: {}", path, e));
    toml::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse config at {}: {}", path, e))
}
