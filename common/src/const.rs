//! # Shared Constants
//!
//! This module defines constants that are shared across both the `server` and `client` crates.

pub const MAP_ROWS: usize = 896 / 2;
pub const MAP_COLS: usize = 896 / 2;

pub const MAX_LOBBY_PLAYERS: usize = 15;
pub const MAX_LOBBIES: usize = 2;

pub const ONLINE: bool = false;
pub const IP_LOCAL: &str = "127.0.0.1:7878";

pub const MAX_MSG_SIZE_BYTES: usize = 1048; // 2KB max

pub const KNIGHT_STR: u8 = 1;
pub const MAGE_STR: u8 = 3;
pub const DRAGON_STR: u8 = 10;
