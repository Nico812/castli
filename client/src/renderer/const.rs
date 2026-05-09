#![allow(dead_code)]

use crossterm::style::Color;

use crate::ansi::BLACK;

pub const FRAME_BK_COLOR: Color = BLACK;
pub const MOD_BK_COLOR: Color = BLACK;

pub const CANVAS_ROWS: usize = 60;
pub const CANVAS_COLS: usize = 160;

pub const FRAME_WIDTH: usize = 1;

pub const MOD_CENTRAL_POS: (usize, usize) = (0, 0);
pub const MOD_CENTRAL_ROWS: usize = 44;
pub const MOD_CENTRAL_COLS: usize = CANVAS_COLS;

pub const MOD_PLAYER_INFO_POS: (usize, usize) = (MOD_CENTRAL_ROWS + 1, 0);
pub const MOD_PLAYER_INFO_ROWS: usize = CANVAS_ROWS - MOD_PLAYER_INFO_POS.0;
pub const MOD_PLAYER_INFO_COLS: usize = MOD_CENTRAL_COLS;

pub const MOD_INSPECT_COLS: usize = 30;
pub const MOD_INSPECT_POS: (usize, usize) = (MOD_CENTRAL_POS.0, 120);

pub const MOD_INTERACT_POS: (usize, usize) = (
    MOD_CENTRAL_POS.0 + 10,
    MOD_CENTRAL_POS.1 + (MOD_CENTRAL_COLS - MOD_INTERACT_COLS) / 2,
);
pub const MOD_INTERACT_COLS: usize = MOD_CENTRAL_COLS * 2 / 5;
