use common::{
    GameCoord, GameId,
    r#const::{MAP_COLS, MAP_ROWS},
    courtyard::{COURTYARD_COLS, COURTYARD_ROWS},
    units::{UnitGroup, UnitType},
};

use crate::renderer::{ModRightTab, renderer::Renderer};

// State shared between input handler and renderer
pub struct UiState {
    pub camera: Camera,
    pub tab: ModRightTab,
    pub mode: UiMode,
}

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
    pub fn get_pos(&self) -> GameCoord {
        match self.location {
            CameraLocation::Map => self.map,
            CameraLocation::WorldMap => GameCoord::new(0, 0),
            CameraLocation::Courtyard => self.courtyard,
        }
    }

    pub fn move_camera(&mut self, dx: isize, dy: isize) {
        let (bound_rows, bound_cols, ref mut camera_pos) = match self.location {
            CameraLocation::Map => (MAP_ROWS, MAP_COLS, self.map),
            CameraLocation::Courtyard => (COURTYARD_ROWS, COURTYARD_COLS, self.courtyard),
            CameraLocation::WorldMap => {
                return;
            }
        };

        camera_pos.x = (camera_pos.x as isize + 2 * dx)
            .max(0)
            .min(bound_cols as isize - Renderer::FOV_COLS as isize) as usize;
        camera_pos.y = (camera_pos.y as isize + 2 * dy)
            .max(0)
            .min((bound_rows) as isize - (Renderer::FOV_ROWS * 2) as isize)
            as usize;
    }
}

pub enum UiMode {
    Std,
    Interact(Interact),
    Inspect(Inspect),
    UnitSelection(UnitSelection),
}

pub struct Inspect {
    pub coord: GameCoord,
    pub selection: Option<GameId>,
}

#[derive(Clone)]
pub struct Interact {
    pub obj_id: Option<GameId>,
    pub coord: GameCoord,
}

pub struct UnitSelection {
    pub obj_id: Option<GameId>,
    pub coord: GameCoord,
    pub active_input: (UnitType, Option<String>),
    pub selected_units: UnitGroup,
}

impl UnitSelection {
    pub fn from_interact(interact: Interact) -> Self {
        Self {
            obj_id: interact.obj_id,
            coord: interact.coord,
            active_input: (UnitType::form_index(0), None),
            selected_units: UnitGroup::new(),
        }
    }
}

impl UiState {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                map: GameCoord::new(0, 0),
                courtyard: GameCoord::new(0, 0),
                location: CameraLocation::Map,
            },
            tab: ModRightTab::Castle,
            mode: UiMode::Std,
        }
    }
}
