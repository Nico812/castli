use common::{
    GameCoord, GameId, all_facilities,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS},
    courtyard::FacilityType,
    units::{UnitGroup, UnitType},
};

use crate::{camera::Camera, coord::TermCoord};
use crate::{camera::CameraLocation, renderer::ModPlayerInfoTab};

pub struct UiState {
    pub camera: Camera,
    pub tab: ModPlayerInfoTab,
    pub mode: UiMode,
    pub term_size_change: Option<TermCoord>,
}

impl UiState {
    pub fn new(fov_size: TermCoord, zoom_factor: usize) -> Self {
        Self {
            camera: Camera::new(fov_size, zoom_factor),
            tab: ModPlayerInfoTab::Castle,
            mode: UiMode::Std,
            term_size_change: None,
        }
    }

    pub fn move_camera(&mut self, dx: isize, dy: isize) {
        let camera_pos = self.camera.get_pos();
        let (bound_rows, bound_cols) = match self.camera.location {
            CameraLocation::Map => (MAP_ROWS - 1, MAP_COLS - 1),
            CameraLocation::Courtyard => (COURTYARD_ROWS - 1, COURTYARD_COLS - 1),
            CameraLocation::WorldMap => {
                return;
            }
        };

        let new_x = (camera_pos.x as isize + 2 * dx)
            .max(0)
            .min(bound_cols.saturating_sub(self.camera.fov_size.x - 1) as isize)
            as usize;
        let new_y = (camera_pos.y as isize + 2 * dy)
            .max(0)
            .min(bound_rows.saturating_sub((self.camera.fov_size.y - 1) * 2) as isize)
            as usize;

        let new_pos = GameCoord::new(new_y - (new_y % 2), new_x - (new_x % 2));
        self.camera.set_pos(new_pos);
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
    Facility(usize),
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
