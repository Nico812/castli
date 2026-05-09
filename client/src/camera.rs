use common::GameCoord;

use crate::coord::TermCoord;

#[derive(PartialEq, Copy, Clone)]
pub enum CameraLocation {
    Map,
    WorldMap,
    Courtyard,
}

pub struct Camera {
    pub location: CameraLocation,
    pub map: GameCoord,
    pub courtyard: GameCoord,
    pub zoom_factor: usize,
    pub fov_size: TermCoord,
}

impl Camera {
    pub fn new(fov_size: TermCoord, zoom_factor: usize) -> Self {
        Self {
            map: GameCoord::new(0, 0),
            courtyard: GameCoord::new(0, 0),
            location: CameraLocation::Map,
            fov_size,
            zoom_factor,
        }
    }

    pub fn get_pos(&self) -> GameCoord {
        match self.location {
            CameraLocation::Map => self.map,
            CameraLocation::WorldMap => GameCoord::new(0, 0),
            CameraLocation::Courtyard => self.courtyard,
        }
    }
}
