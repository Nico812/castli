use std::ops::{Add, Mul, Sub};

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

    pub fn from_game_coord(game_coord: GameCoord, camera: &Camera) -> Option<Self> {
        let (term_y, term_x) = if camera.location == CameraLocation::WorldMap {
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
        if term_y >= CANVAS_ROWS || term_x >= CANVAS_COLS {
            return None;
        };

        Some(Self::new(term_y, term_x))
    }

    // This calculates the GameCoords of an object shown at a certain TermCoord
    // If the camera is in WorldMap gives the upper left GameCoords
    // If are_mod_central_relative it takes the given TermCoord as having origin at top left of ModCentral content
    // Returns None if the GameCoords are out of bounds
    pub fn to_game_coord(&self, camera: &Camera) -> Option<GameCoord> {
        if camera.location == CameraLocation::WorldMap {
            return Some(GameCoord::new(
                self.y * Renderer::ZOOM_FACTOR * 2,
                self.x * Renderer::ZOOM_FACTOR,
            ));
        } else {
            let camera_pos = camera.get_pos();
            let game_y = camera_pos.y + self.y * 2;
            let game_x = camera_pos.x + self.x;

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

impl Add for TermCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for TermCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<usize> for TermCoord {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
