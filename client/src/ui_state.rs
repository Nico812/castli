use common::{
    GameCoord, GameId, all_facilities,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS},
    courtyard::{Facility, FacilityType},
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
        let (bound_rows, bound_cols, camera_pos) = match self.location {
            CameraLocation::Map => (MAP_ROWS, MAP_COLS, &mut self.map),
            CameraLocation::Courtyard => (COURTYARD_ROWS, COURTYARD_COLS, &mut self.courtyard),
            CameraLocation::WorldMap => {
                return;
            }
        };

        camera_pos.x = (camera_pos.x as isize + 2 * dx)
            .max(0)
            .min(bound_cols.saturating_sub(Renderer::FOV_COLS) as isize)
            as usize;
        camera_pos.y = (camera_pos.y as isize + 2 * dy)
            .max(0)
            .min(bound_rows.saturating_sub(Renderer::FOV_ROWS * 2) as isize)
            as usize;
    }
}

pub enum UiMode {
    Std,
    Interact(InteractTarget),
    Inspect(Inspect),
    UnitSelection(UnitSelection),
    FacilitySelection(FacilitySelection),
}

pub struct Inspect {
    pub coord: GameCoord,
    pub selection: Option<GameId>,
}

#[derive(Clone)]
pub enum InteractTarget {
    MapPos(GameCoord),
    CourtyardPos(GameCoord),
    GameObj(GameId),
    // TODO: change this to take a facility id.
    Facility(GameId),
}

pub struct UnitSelection {
    pub interact_target: InteractTarget,
    pub active_input: (UnitType, Option<String>),
    pub selected_units: UnitGroup,
}

impl UnitSelection {
    pub fn from_interact(interact_target: InteractTarget) -> Self {
        Self {
            interact_target,
            active_input: (UnitType::form_index(0), None),
            selected_units: UnitGroup::new(),
        }
    }
}

pub struct FacilitySelection {
    pub pos: GameCoord,
    pub active: FacilityType,
}

impl FacilitySelection {
    pub fn new(pos: GameCoord) -> Self {
        Self {
            pos,
            active: all_facilities!()[0],
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
