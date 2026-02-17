use common::GameCoord;

use crate::canvas::r#const::{CANVAS_COLS, CANVAS_ROWS, CENTRAL_MOD_POS};

// TermCoord are the 1-indexed terminal coordinates with origin at CANVAS_POS
// A TermCoord y-displacement corresponds to a GameCoord 2y-displacement
#[derive(Clone, Copy)]
pub struct TermCoord {
    pub x: usize,
    pub y: usize,
}

impl TermCoord {
    pub fn from_game_coord(game_coord: GameCoord, zoom_coord: GameCoord) -> Option<Self> {
        let mut y = game_coord.y / 2;
        let mut x = game_coord.x;
        // +1 to account for frame
        // +1 o account for 1-indexing of terminal coords
        y += CENTRAL_MOD_POS.0 + 1;
        x += CENTRAL_MOD_POS.1 + 1;
        if y < zoom_coord.y || y < zoom_coord.x || y >= CANVAS_ROWS || x >= CANVAS_COLS {
            return None;
        }
        y -= zoom_coord.y;
        x -= zoom_coord.x;

        Some(Self { y, x })
    }
}

// GameCoord are defined in the server crate and are the game's map coordinates
pub trait GameCoordExtension {
    fn from_zoom_coord(term_coord: TermCoord, y_shift: bool) -> Self;
}

impl GameCoordExtension for GameCoord {
    fn from_zoom_coord(term_coord: TermCoord, y_shift: bool) -> Self {
        let x = term_coord.x;
        let mut y = term_coord.y * 2;

        if y_shift {
            y += 1;
        }

        Self { x, y }
    }
}
