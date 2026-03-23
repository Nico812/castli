#![allow(dead_code)]

pub const CANVAS_ROWS: usize = 58;
pub const CANVAS_COLS: usize = 160;

// Accounting for frame.

// Content of modules, not total size accounting for frame.
pub const MOD_CENTRAL_POS: (usize, usize) = (0, 0);
pub const MOD_CENTRAL_ROWS: usize = CANVAS_ROWS;
pub const MOD_CENTRAL_COLS: usize = CANVAS_ROWS * 2 - 2; // -2 because the frame is thin horizontally and i want map squared.

pub const MOD_RIGHT_POS: (usize, usize) = (0, MOD_CENTRAL_COLS - 1);
pub const MOD_RIGHT_ROWS: usize = MOD_CENTRAL_ROWS;
pub const MOD_RIGHT_COLS: usize = CANVAS_COLS - MOD_CENTRAL_COLS + 1;

pub const MOD_INSPECT_COLS: usize = 30;
