use common::{
    GameCoord,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS},
};

use crate::{
    renderer::{
        r#const::{CANVAS_COLS, CANVAS_ROWS, MOD_CENTRAL_POS},
        renderer::Renderer,
    },
    ui_state::{Camera, CameraLocation},
};

// TermCoord are the 1-indexed terminal coordinates with origin at CANVAS_POS
// A TermCoord y-displacement corresponds to a GameCoord 2y-displacement
#[derive(Clone, Copy)]
pub struct TermCoord {
    pub x: usize,
    pub y: usize,
}

impl TermCoord {
    pub fn new(y: usize, x: usize) -> Self {
        Self { x, y }
    }

    pub fn from_game_coord(
        game_coord: GameCoord,
        camera: &Camera,
        get_mod_central_relative: bool,
    ) -> Option<Self> {
        let (mut term_y, mut term_x) = if camera.location == CameraLocation::WorldMap {
            let term_y = game_coord.y / 2 / Renderer::ZOOM_FACTOR;
            let term_x = game_coord.x / Renderer::ZOOM_FACTOR;

            (term_y, term_x)
        } else {
            let camera_pos = camera.get_pos();
            if game_coord.y < camera_pos.y || game_coord.x < camera_pos.x {
                return None;
            }
            let rel_game_y = game_coord.y - camera_pos.y;
            let rel_game_x = game_coord.x - camera_pos.x;

            (rel_game_y / 2, rel_game_x)
        };

        // Check out of boundary
        if (get_mod_central_relative
            && (term_y >= Renderer::FOV_ROWS || term_x >= Renderer::FOV_COLS))
            || (!get_mod_central_relative && (term_y >= CANVAS_ROWS || term_x >= CANVAS_COLS))
        {
            return None;
        };

        // +1 to account for frame
        // +1 o account for 1-indexing of terminal coords
        if !get_mod_central_relative {
            term_y += MOD_CENTRAL_POS.0 + 1;
            term_x += MOD_CENTRAL_POS.1 + 1;
        };

        Some(Self::new(term_y, term_x))
    }

    // This calculates the GameCoords of an object shown at a certain TermCoord
    // If the camera is in WorldMap gives the upper left GameCoords
    // If are_mod_central_relative it takes the given TermCoord as having origin at top left of ModCentral content
    // Returns None if the GameCoords are out of bounds
    pub fn to_game_coord(
        &self,
        camera: &Camera,
        are_mod_central_relative: bool,
    ) -> Option<GameCoord> {
        let (rel_term_y, rel_term_x) = if !are_mod_central_relative {
            let Some(rel_term_y) = self.y.checked_sub(MOD_CENTRAL_POS.0 + 1) else {
                return None;
            };
            let Some(rel_term_x) = self.x.checked_sub(MOD_CENTRAL_POS.1 + 1) else {
                return None;
            };

            (rel_term_y, rel_term_x)
        } else {
            (self.y, self.x)
        };

        if camera.location == CameraLocation::WorldMap {
            return Some(GameCoord::new(
                rel_term_y * Renderer::ZOOM_FACTOR * 2,
                rel_term_x * Renderer::ZOOM_FACTOR,
            ));
        } else {
            let camera_pos = camera.get_pos();
            let game_y = camera_pos.y + rel_term_y * 2;
            let game_x = camera_pos.x + rel_term_x;

            // Checking if GameCoord is out of bounds
            if (camera.location == CameraLocation::Map
                && (game_y >= MAP_ROWS || game_x >= MAP_COLS))
                || (camera.location == CameraLocation::Courtyard
                    && (game_y >= COURTYARD_ROWS || game_x >= COURTYARD_COLS))
            {
                return None;
            } else {
                return Some(GameCoord::new(game_y, game_x));
            }
        };
    }
}
