use common::{
    GameCoord, GameId,
    units::{UnitGroup, UnitType},
};

use crate::renderer::ModRightTab;

// State shared between input handler and renderer
pub struct UiState {
    pub camera_map: GameCoord,
    pub camera_courtyard: GameCoord,
    pub camera_location: CameraLocation,
    pub tab: ModRightTab,
    pub mode: UiMode,
}

#[derive(PartialEq, Copy, Clone)]
pub enum CameraLocation {
    Map,
    WorldMap,
    Courtyard,
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
            zoom: Some(GameCoord { x: 0, y: 0 }),
            c_zoom: None,
            tab: ModRightTab::Castle,
            mode: UiMode::Std,
        }
    }
}
