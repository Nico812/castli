use std::{mem::swap, ops::DerefMut};

use common::{
    GameCoord, GameId, all_facilities,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS},
    courtyard::FacilityType,
    units::{UnitGroup, UnitType},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    camera::Camera,
    coord::TermCoord,
    game_state::GameState,
    tui::{T2C, Tui},
};
use crate::{camera::CameraLocation, renderer::ModPlayerInfoTab};

pub struct UiState {
    pub camera: Camera,
    pub tab: ModPlayerInfoTab,
    pub mode: UiMode,
    pub term_size_change: Option<TermCoord>,
    pub prev_modes: Vec<UiMode>,
}

impl UiState {
    pub fn new(fov_size: TermCoord, zoom_factor: usize) -> Self {
        Self {
            camera: Camera::new(fov_size, zoom_factor),
            tab: ModPlayerInfoTab::Castle,
            mode: UiMode::Std,
            term_size_change: None,
            prev_modes: Vec::new(),
        }
    }

    pub fn restore_prev_mode(&mut self) {
        if let Some(prev_mode) = self.prev_modes.pop() {
            self.mode = prev_mode;
        }
    }

    pub fn restore_inspect(&mut self) {
        while let Some(prev_mode) = self.prev_modes.pop() {
            if let UiMode::Inspect(_) = prev_mode {
                self.mode = prev_mode;
            }
        }
    }

    pub fn restore_std_mode(&mut self) {
        self.mode = UiMode::Std;
        self.prev_modes = Vec::new();
    }

    pub fn change_mode(&mut self, new_mode: UiMode) {
        let mut current_mode = new_mode;
        std::mem::swap(&mut current_mode, &mut self.mode);
        self.prev_modes.push(current_mode);
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
            .min(bound_cols.saturating_sub(self.camera.fov_size.x as usize - 1) as isize)
            as usize;
        let new_y = (camera_pos.y as isize + 2 * dy)
            .max(0)
            .min(bound_rows.saturating_sub((self.camera.fov_size.y as usize - 1) * 2) as isize)
            as usize;

        let new_pos = GameCoord::new(new_y - (new_y % 2), new_x - (new_x % 2));
        self.camera.set_pos(new_pos);
    }

    pub fn change_camera_location(&mut self, new: CameraLocation, tx: &UnboundedSender<T2C>) {
        if self.camera.location == new {
            return;
        }
        if new == CameraLocation::Courtyard {
            let _ = tx.send(T2C::InCourtyard);
        };
        if self.camera.location == CameraLocation::Courtyard {
            let _ = tx.send(T2C::OutCourtyard);
        };
        if new == CameraLocation::Courtyard
            && let UiMode::Inspect(ref mut inspect) = self.mode
        {
            inspect.coord = GameCoord { x: 0, y: 0 };
        };
        if new == CameraLocation::WorldMap
            && self.camera.location == CameraLocation::Map
            && let UiMode::Inspect(ref mut inspect) = self.mode
        {
            inspect.coord.y -= inspect.coord.y % self.camera.zoom_factor;
            inspect.coord.x -= inspect.coord.x % self.camera.zoom_factor;
        }
        self.camera.location = new;
    }

    pub fn toggle_inspect(&mut self) {
        if let UiMode::Std = self.mode {
            let coord = self.camera.get_pos();
            self.change_mode(UiMode::Inspect(Inspect {
                coord,
                selection: None,
            }));
        } else if let UiMode::Inspect(_) = self.mode {
            self.restore_std_mode();
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

impl Inspect {
    pub fn displace(
        &mut self,
        mut dx: isize,
        mut dy: isize,
        camera: &Camera,
        game_state: &GameState,
    ) {
        if let Some(ref mut selection) = self.selection {
            let looked_objs = Tui::get_looked_objs(game_state, self.coord, camera);
            let new_selection = {
                let current_pos = looked_objs.iter().position(|(id, _)| *id == *selection);
                match dy {
                    dy if dy > 0 => current_pos.and_then(|pos| {
                        let new_pos = (pos + 1).min(looked_objs.len() - 1);
                        looked_objs.get(new_pos).map(|(id, _)| *id)
                    }),
                    dy if dy < 0 => current_pos.and_then(|pos| {
                        let new_pos = pos.saturating_sub(1);
                        looked_objs.get(new_pos).map(|(id, _)| *id)
                    }),
                    _ => None,
                }
            };

            if let Some(new_id) = new_selection {
                *selection = new_id;
            }
        } else {
            let camera_pos = camera.get_pos();
            match camera.location {
                CameraLocation::WorldMap => {
                    dx *= camera.zoom_factor as isize;
                    dy *= camera.zoom_factor as isize;
                    self.coord.x = (self.coord.x as isize + dx)
                        .max(0)
                        .min(MAP_COLS as isize - 1) as usize;
                    self.coord.y = (self.coord.y as isize + dy)
                        .max(0)
                        .min(MAP_ROWS as isize - 1) as usize;
                }
                CameraLocation::Map => {
                    self.coord.x = (self.coord.x as isize + dx)
                        .max(camera_pos.x as isize)
                        .min(camera_pos.x as isize + camera.fov_size.x - 1)
                        as usize;
                    self.coord.y = (self.coord.y as isize + dy)
                        .max(camera_pos.y as isize)
                        .min(camera_pos.y as isize + camera.fov_size.y * 2 - 1)
                        as usize;
                }
                CameraLocation::Courtyard => {
                    self.coord.x = (self.coord.x as isize + dx).max(camera_pos.x as isize).min(
                        camera_pos.x as isize + camera.fov_size.x.min(COURTYARD_COLS as isize) - 1,
                    ) as usize;
                    self.coord.y = (self.coord.y as isize + dy).max(camera_pos.y as isize).min(
                        camera_pos.y as isize
                            + (camera.fov_size.y * 2).min(COURTYARD_ROWS as isize)
                            - 1,
                    ) as usize;
                }
            }
            self.coord.x -= self.coord.x % 2;
            self.coord.y -= self.coord.y % 2;
        }
    }
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

    pub fn move_selection(&mut self, dy: isize) {
        if self.active_input.1.is_some() {
            return;
        }

        match dy {
            dy if dy > 0 => {
                let new_unit_index = (self.active_input.0.as_index() + 1).min(UnitType::COUNT - 1);
                self.active_input.0 = UnitType::from_index(new_unit_index);
            }
            dy if dy < 0 => {
                let new_unit_index = self.active_input.0.as_index().saturating_sub(1);
                self.active_input.0 = UnitType::from_index(new_unit_index);
            }
            _ => {}
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

    pub fn move_selection(&mut self, dy: isize) {
        match dy {
            dy if dy > 0 => {
                let new_facility_index = (self.active.as_index() + 1).min(FacilityType::COUNT - 1);
                self.active = FacilityType::from_index(new_facility_index);
            }
            dy if dy < 0 => {
                let new_facility_index = self.active.as_index().saturating_sub(1);
                self.active = FacilityType::from_index(new_facility_index);
            }
            _ => {}
        }
    }
}
