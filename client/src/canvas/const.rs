#![allow(dead_code)]

pub const CANVAS_ROWS: usize = 58;
pub const CANVAS_COLS: usize = 160;

// Accounting for frame.

// Content of modules, not total size accounting for frame.
pub const CENTRAL_MOD_POS: (usize, usize) = (0, 0);
pub const CENTRAL_MODULE_ROWS: usize = CANVAS_ROWS;
pub const CENTRAL_MODULE_COLS: usize = CANVAS_ROWS * 2 - 2; // -2 because the frame is thin horizontally and i want map squared.

pub const RIGHT_MOD_POS: (usize, usize) = (0, CENTRAL_MODULE_COLS - 1);
pub const RIGHT_MODULE_ROWS: usize = CENTRAL_MODULE_ROWS;
pub const RIGHT_MODULE_COLS: usize = CANVAS_COLS - CENTRAL_MODULE_COLS + 1;
