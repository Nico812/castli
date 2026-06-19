use common::GameCoord;
use crossterm::style::Color;

use crate::ansi::BLACK;

pub const LOGS_CAPACITY: usize = 100;

pub const CURSOR_SIZE: GameCoord = GameCoord::new(2, 2);

pub const FRAME_BK_COLOR: Color = BLACK;
pub const MOD_BK_COLOR: Color = BLACK;

pub const FRAME_WIDTH: usize = 1;

pub const MOD_INSPECT_COLS: usize = 30;
pub const MOD_INTERACT_COLS: usize = 50;
pub const MOD_PLAYER_INFO_ROWS: usize = 16;
