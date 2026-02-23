#![allow(dead_code)]

pub const CANVAS_ROWS: usize = 58;
pub const CANVAS_COLS: usize = 160;

// Accounting for frame.
pub const CENTRAL_MOD_POS: (usize, usize) = (0, 0);
pub const LEFT_MOD_POS: (usize, usize) = (CENTRAL_MOD_POS.0, 0);
pub const RIGHT_MOD_POS: (usize, usize) = (0, CENTRAL_MODULE_COLS);
pub const BOTTOM_MOD_POS: (usize, usize) = (CANVAS_ROWS - BOTTOM_MODULE_ROWS, 0);

// Content of modules, not total size accounting for frame.
pub const CENTRAL_MODULE_ROWS: usize = CANVAS_ROWS;
pub const CENTRAL_MODULE_COLS: usize = CANVAS_ROWS * 2;
pub const CENTRAL_MODULE_CONTENT_ROWS: usize = CENTRAL_MODULE_ROWS - 2;
pub const CENTRAL_MODULE_CONTENT_COLS: usize = CENTRAL_MODULE_COLS - 4;

pub const LEFT_MODULE_ROWS: usize = 0;
pub const LEFT_MODULE_COLS: usize = 0;
pub const RIGHT_MODULE_ROWS: usize = CENTRAL_MODULE_ROWS;
pub const RIGHT_MODULE_COLS: usize = CANVAS_COLS - CENTRAL_MODULE_COLS;
pub const BOTTOM_MODULE_ROWS: usize = 0;
pub const BOTTOM_MODULE_COLS: usize = 0;
