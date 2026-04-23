use common::{
    GameCoord, GameID,
    exports::units::{UnitGroupE, UnitType},
};

use crate::renderer::ModRightTab;

// State shared between input handler and renderer
pub struct UiState {
    pub zoom: Option<GameCoord>,
    pub tab: ModRightTab,
    pub mode: UiMode,
}

pub enum UiMode {
    Std,
    Interact(Interact),
    Inspect(Inspect),
    UnitSelection(UnitSelection),
}

pub struct Inspect {
    pub coord: GameCoord,
    pub selection: Option<GameID>,
}

#[derive(Clone)]
pub struct Interact {
    pub obj_id: Option<GameID>,
    pub coord: GameCoord,
}

pub struct UnitSelection {
    pub obj_id: Option<GameID>,
    pub coord: GameCoord,
    pub active_input: (UnitType, Option<String>),
    pub selected_units: UnitGroupE,
}

impl UnitSelection {
    pub fn from_interact(interact: Interact) -> Self {
        Self {
            obj_id: interact.obj_id,
            coord: interact.coord,
            active_input: (UnitType::form_index(0), None),
            selected_units: UnitGroupE {
                quantities: [0, 0, 0],
            },
        }
    }
}

impl UiState {
    pub fn new() -> Self {
        Self {
            zoom: Some(GameCoord { x: 0, y: 0 }),
            tab: ModRightTab::Castle,
            mode: UiMode::Std,
        }
    }
}
