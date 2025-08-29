pub const CANVAS_ROWS: usize = 51;
pub const CANVAS_COLS: usize = 160;

// Accounting for frame.
pub const CENTRAL_MOD_POS: (usize, usize) = (1, LEFT_MODULE_COLS + 7);
pub const LEFT_MOD_POS: (usize, usize) = (CENTRAL_MOD_POS.0, 3);
pub const RIGHT_MOD_POS: (usize, usize) = (
    CENTRAL_MOD_POS.0,
    CENTRAL_MOD_POS.1 + CENTRAL_MODULE_COLS + 6,
);
pub const BOTTOM_MOD_POS: (usize, usize) = (
    CENTRAL_MOD_POS.0 + CENTRAL_MODULE_ROWS + 3,
    CENTRAL_MOD_POS.1,
);

// Content of modules, not total size accounting for frame.
pub const CENTRAL_MODULE_ROWS: usize = 34;
pub const CENTRAL_MODULE_COLS: usize = 66;

pub const LEFT_MODULE_ROWS: usize = CENTRAL_MODULE_ROWS;
pub const LEFT_MODULE_COLS: usize = 40;
pub const RIGHT_MODULE_ROWS: usize = CENTRAL_MODULE_ROWS;
pub const RIGHT_MODULE_COLS: usize = LEFT_MODULE_COLS;
pub const BOTTOM_MODULE_ROWS: usize = 13;
pub const BOTTOM_MODULE_COLS: usize = CENTRAL_MODULE_COLS;

// 94 available cols meno central con cornice
// 47 a sinistra 47 a destra
// 44 a sinistra meno bordi pagina
// 42 a sinistra meno cornice
// 40 meno spazio tra sinistra e modulo centrale
//
// expected: cols: 3 vuote, left module, due vuote (4 se senza cornice), central, due vuote, right,
// 3
//
// 17 rows senza central module con cornice,
// 13 rows disponibili totali
//
// expected: rows: 1 spazi sopra, central, 1 spazi, bottom, 2 spazi