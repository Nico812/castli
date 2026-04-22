use common::GameCoord;

use crate::renderer::{
    r#const::{CANVAS_COLS, CANVAS_ROWS, MOD_CENTRAL_POS},
    renderer::Renderer,
};

// TermCoord are the 1-indexed terminal coordinates with origin at CANVAS_POS
// A TermCoord y-displacement corresponds to a GameCoord 2y-displacement
#[derive(Clone, Copy)]
pub struct TermCoord {
    pub x: usize,
    pub y: usize,
}

impl TermCoord {
    pub fn from_game_coord(game_coord: GameCoord, map_zoom: Option<GameCoord>) -> Option<Self> {
        let (mut term_y, mut term_x) = match map_zoom {
            Some(zoom_coord) => {
                if game_coord.y < zoom_coord.y || game_coord.x < zoom_coord.x {
                    return None;
                }
                let rel_game_y = game_coord.y - zoom_coord.y;
                let rel_game_x = game_coord.x - zoom_coord.x;

                (rel_game_y / 2, rel_game_x)
            }
            None => {
                let term_y = game_coord.y / 2 / Renderer::ZOOM_FACTOR;
                let term_x = game_coord.x / Renderer::ZOOM_FACTOR;

                (term_y, term_x)
            }
        };
        // +1 to account for frame
        // +1 o account for 1-indexing of terminal coords
        term_y += MOD_CENTRAL_POS.0 + 1;
        term_x += MOD_CENTRAL_POS.1 + 1;

        if term_y > CANVAS_ROWS || term_x > CANVAS_COLS {
            return None;
        }

        Some(Self {
            y: term_y,
            x: term_x,
        })
    }
}
