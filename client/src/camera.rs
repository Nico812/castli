use common::{
    GameCoord,
    config::config as common_config,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
};

use crate::config::config as client_config;

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
}

impl Camera {
    pub fn new() -> Self {
        Self {
            map: GameCoord::new(0, 0),
            courtyard: GameCoord::new(0, 0),
            location: CameraLocation::Map,
        }
    }

    pub fn get_pos(&self) -> GameCoord {
        match self.location {
            CameraLocation::Map => self.map,
            CameraLocation::WorldMap => GameCoord::new(0, 0),
            CameraLocation::Courtyard => self.courtyard,
        }
    }

    pub fn move_camera(&mut self, dx: isize, dy: isize) {
        let (bound_rows, bound_cols, camera_pos) = match self.location {
            CameraLocation::Map => (
                common_config().world.map_rows,
                common_config().world.map_cols,
                &mut self.map,
            ),
            CameraLocation::Courtyard => (COURTYARD_ROWS, COURTYARD_COLS, &mut self.courtyard),
            CameraLocation::WorldMap => {
                return;
            }
        };

        let ui = &client_config().ui;
        let new_x = (camera_pos.x as isize + 2 * dx)
            .max(0)
            .min(bound_cols.saturating_sub(ui.fov_cols() - 1) as isize)
            as usize;
        let new_y = (camera_pos.y as isize + 2 * dy)
            .max(0)
            .min(bound_rows.saturating_sub(ui.fov_rows() * 2 - 1) as isize)
            as usize;

        // I'm angry at odd numbers
        camera_pos.x = new_x - (new_x % 2);
        camera_pos.y = new_y - (new_y % 2);
    }
}
