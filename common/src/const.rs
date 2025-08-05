//! # Shared Constants
//!
//! This module defines constants that are shared across both the `server` and `client` crates.

pub const MAP_ROWS: usize = 512;
pub const MAP_COLS: usize = 512;
pub const CASTLE_SIZE: usize = 8;

pub const MAX_LOBBY_PLAYERS: usize = 2;
pub const MAX_LOBBIES: usize = 2;

pub const ONLINE: bool = false;
pub const IP_LOCAL: &str = "127.0.0.1:7878";

pub const MAX_MSG_SIZE_BYTES: usize = 1048; // 2KB max
