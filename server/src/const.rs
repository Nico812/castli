//! # Server-Specific Constants
//!
//! This module defines constants that are only used by the `server` crate,
//! particularly for game logic and map generation.
pub const CA_ITER_WATER: usize = 20;
pub const CA_ITER_WOODS: usize = 10;
pub const PERCENT_IS_WATER: u8 = 50;
pub const PERCENT_IS_WOODS: u8 = 35;
pub const COUNTS_TO_SPREAD_WATER: u8 = 5;
pub const COUNTS_TO_SURVIVE_WATER: u8 = 4;
pub const COUNTS_TO_SPREAD_WOODS: u8 = 4;
pub const COUNTS_TO_SURVIVE_WOODS: u8 = 4;
