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
        if game_coord.y < zoom_coord.y || game_coord.x < zoom_coord.x {
            return None;
        }
        let rel_game_y = game_coord.y - zoom_coord.y;
        let rel_game_x = game_coord.x - zoom_coord.x;

        let mut term_y = rel_game_y / 2;
        let mut term_x = rel_game_x;
        // +1 to account for frame
        // +1 o account for 1-indexing of terminal coords
        term_y += CENTRAL_MOD_POS.0 + 1;
        term_x += CENTRAL_MOD_POS.1 + 1;

        if term_y > CANVAS_ROWS || term_x > CANVAS_COLS {
            return None;
        }

        Some(Self {
            y: term_y,
            x: term_x,
        })
    }
}
