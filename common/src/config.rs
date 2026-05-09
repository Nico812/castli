use std::sync::OnceLock;

use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize)]
pub struct CommonConfig {
    pub network: NetworkConfig,
    pub world: WorldConfig,
    pub units: UnitsConfig,
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub address: String,
}

#[derive(Deserialize)]
pub struct WorldConfig {
    pub map_rows: usize,
    pub map_cols: usize,
}

#[derive(Deserialize)]
pub struct UnitsConfig {
    pub knight_strength: u32,
    pub mage_strength: u32,
    pub dragon_strength: u32,
    pub ship_strength: u32,
}

const DEFAULT_PATH: &str = "castli.common.toml";
const ENV_VAR: &str = "CASTLI_COMMON_CONFIG";

static CONFIG: OnceLock<CommonConfig> = OnceLock::new();

pub fn config() -> &'static CommonConfig {
    CONFIG.get_or_init(|| load_from(DEFAULT_PATH, ENV_VAR))
}

/// Generic loader used by per-crate configs.
pub fn load_from<T: DeserializeOwned>(default_path: &str, env_var: &str) -> T {
    let path = std::env::var(env_var).unwrap_or_else(|_| default_path.to_string());
    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read config at {}: {}", path, e));
    toml::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse config at {}: {}", path, e))
}
