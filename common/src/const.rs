use crate::GameCoord;

pub const MAP_ROWS: usize = 896;
pub const MAP_COLS: usize = 896;

pub const COURTYARD_ROWS: usize = 60;
pub const COURTYARD_COLS: usize = 60;

pub const MAX_LOBBY_PLAYERS: usize = 15;
pub const MAX_LOBBIES: usize = 10;

pub const ONLINE: bool = false;
pub const IP_LOCAL: &str = "127.0.0.1:7878";

pub const KNIGHT_STR: u32 = 1;
pub const MAGE_STR: u32 = 3;
pub const DRAGON_STR: u32 = 10;
pub const SHIP_STR: u32 = 0;

pub const CASTLE_SIZE: GameCoord = GameCoord::new(2, 1);
pub const FARM_PLOT_SIZE: GameCoord = GameCoord::new(10, 9);
