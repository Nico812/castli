//! # Shared Constants
//!
//! This module defines constants that are shared across both the `server` and `client` crates.

// Max map rows/cols is 896 or it will break
pub const MAP_ROWS: usize = 896 / 4;
pub const MAP_COLS: usize = 896 / 4;
pub const COURTYARD_ROWS: usize = 64;
pub const COURTYARD_COLS: usize = 64;

pub const MAX_LOBBY_PLAYERS: usize = 15;
pub const MAX_LOBBIES: usize = 10;

pub const ONLINE: bool = false;
pub const IP_LOCAL: &str = "127.0.0.1:7878";

pub const MAX_MSG_SIZE_BYTES: usize = 1048; // 2KB max

pub const KNIGHT_STR: u32 = 1;
pub const MAGE_STR: u32 = 3;
pub const DRAGON_STR: u32 = 10;
