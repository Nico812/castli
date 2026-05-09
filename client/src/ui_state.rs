use common::{
    GameCoord, GameId, all_facilities,
    courtyard::FacilityType,
    units::{UnitGroup, UnitType},
};

use crate::camera::Camera;
use crate::renderer::ModPlayerInfoTab;

// State shared between input handler and renderer
pub struct UiState {
    pub camera: Camera,
    pub tab: ModPlayerInfoTab,
    pub mode: UiMode,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            tab: ModPlayerInfoTab::Castle,
            mode: UiMode::Std,
        }
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
    Facility(u8),
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
