use std::ops::{Add, Div, Mul, Sub};

use common::GameCoord;

use crate::camera::{Camera, CameraLocation};

// TermCoord are the 1-indexed terminal coordinates with origin at CANVAS_POS
// A TermCoord y-displacement corresponds to a GameCoord 2y-displacement
#[derive(Clone, Copy)]
pub struct TermCoord {
    pub x: isize,
    pub y: isize,
}

impl TermCoord {
    pub fn new(y: isize, x: isize) -> Self {
        Self { x, y }
    }

    pub fn from_game_coord(game_coord: GameCoord, camera: &Camera) -> Self {
        let (term_y, term_x) = if camera.location == CameraLocation::WorldMap {
            let term_y = game_coord.y / 2 / camera.zoom_factor;
            let term_x = game_coord.x / camera.zoom_factor;

            (term_y as isize, term_x as isize)
        } else {
            let camera_pos = camera.get_pos();
            let rel_game_y = game_coord.y as isize - camera_pos.y as isize;
            let rel_game_x = game_coord.x as isize - camera_pos.x as isize;

            (rel_game_y / 2, rel_game_x)
        };

        Self::new(term_y, term_x)
    }

    // This calculates the GameCoords of an object shown at a certain TermCoord
    // If the camera is in WorldMap gives the upper left GameCoords
    // If are_mod_central_relative it takes the given TermCoord as having origin at top left of ModCentral content
    // Returns None if the GameCoords are out of bounds
    // pub fn to_game_coord(&self, camera: &Camera) -> Option<GameCoord> {
    //     match camera.location {
    //         CameraLocation::WorldMap => {
    //             let game_y = self.y * 2 * camera.zoom_factor;
    //             let game_x = self.x * camera.zoom_factor;

    //             if game_y >= MAP_ROWS || game_x >= MAP_COLS {
    //                 None
    //             } else {
    //                 Some(GameCoord::new(game_y, game_x))
    //             }
    //         }
    //         CameraLocation::Map => {
    //             let camera_pos = camera.get_pos();
    //             let game_y = camera_pos.y + self.y * 2;
    //             let game_x = camera_pos.x + self.x;

    //             if game_y >= MAP_ROWS || game_x >= MAP_COLS {
    //                 None
    //             } else {
    //                 Some(GameCoord::new(game_y, game_x))
    //             }
    //         }
    //         CameraLocation::Courtyard => {
    //             let camera_pos = camera.get_pos();
    //             let game_y = camera_pos.y + self.y * 2;
    //             let game_x = camera_pos.x + self.x;

    //             if game_y >= COURTYARD_ROWS || game_x >= COURTYARD_COLS {
    //                 None
    //             } else {
    //                 Some(GameCoord::new(game_y, game_x))
    //             }
    //         }
    //     }
    // }
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
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl Mul<usize> for TermCoord {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            x: self.x * scalar as isize,
            y: self.y * scalar as isize,
        }
    }
}

impl Div<usize> for TermCoord {
    type Output = Self;

    fn div(self, scalar: usize) -> Self {
        Self {
            x: self.x / scalar as isize,
            y: self.y / scalar as isize,
        }
    }
}
