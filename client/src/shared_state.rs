use std::collections::{HashMap, VecDeque};

use common::{
    GameCoord, GameID,
    exports::{
        game_object::GameObjE,
        player::PlayerE,
        units::{UnitGroupE, UnitType},
    },
};

use crate::renderer::{ModRightTab, renderer::Renderer};

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
    pub interact: Interact,
    pub active_input: (UnitType, String),
    pub selected_units: UnitGroupE,
}

impl UnitSelection {
    pub fn from_interact(interact: Interact) -> Self {
        Self {
            interact,
            active_input: (UnitType::form_index(0), "0".to_string()),
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
