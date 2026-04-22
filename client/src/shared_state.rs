use std::collections::{HashMap, VecDeque};

use common::{
    GameCoord, GameID,
    exports::{
        game_object::GameObjE,
        player::PlayerE,
        units::{UnitGroupE, UnitType},
    },
};

use crate::game_renderer::{ModRightTab, game_renderer::GameRenderer};

// Variables shared between input handler, renderer, server comunication handler.
pub struct SharedState {
    pub game_objs: HashMap<GameID, GameObjE>,
    pub logs: VecDeque<String>,
    pub player_data: PlayerE,
    pub map_zoom: Option<GameCoord>,
    pub mod_right_tab: ModRightTab,
    pub ui_state: UIState,
}

pub enum UIState {
    Std,
    Interact(UIInteract),
    Inspect(UIInspect),
    UnitSelection(UIUnitSelection),
}

pub struct UIInspect {
    pub coord: GameCoord,
    pub selection: Option<GameID>,
}

#[derive(Clone)]
pub struct UIInteract {
    pub obj_id: Option<GameID>,
    pub coord: GameCoord,
}

pub struct UIUnitSelection {
    pub interact: UIInteract,
    pub active_input: (UnitType, String),
    pub selected_units: UnitGroupE,
}

impl UIUnitSelection {
    pub fn from_interact(interact: UIInteract) -> Self {
        Self {
            interact,
            active_input: (UnitType::form_index(1), String::new()),
            selected_units: UnitGroupE {
                quantities: [0, 0, 0],
            },
        }
    }
}

impl SharedState {
    pub fn new(
        initial_game_objs: HashMap<usize, GameObjE>,
        initial_player_data: Option<PlayerE>,
    ) -> Self {
        Self {
            game_objs: initial_game_objs,
            player_data: initial_player_data.unwrap_or(PlayerE::undef()),
            map_zoom: Some(GameCoord { x: 0, y: 0 }),
            logs: VecDeque::new(),
            mod_right_tab: ModRightTab::Castle,
            ui_state: UIState::Std,
        }
    }

    pub fn get_looked_objs<'a>(
        coord: GameCoord,
        zoom: &Option<GameCoord>,
        game_objs: &'a HashMap<GameID, GameObjE>,
    ) -> Vec<(GameID, &'a GameObjE)> {
        let mut looked_objs: Vec<(GameID, &GameObjE)> = game_objs
            .iter()
            .filter_map(|(game_id, game_obj)| {
                if (zoom.is_some() && game_obj.get_pos() == coord)
                    || (zoom.is_none()
                        && game_obj.get_pos().y >= coord.y
                        && game_obj.get_pos().x >= coord.x
                        && game_obj.get_pos().y < coord.y + GameRenderer::ZOOM_FACTOR
                        && game_obj.get_pos().x < coord.x + GameRenderer::ZOOM_FACTOR)
                {
                    Some((*game_id, game_obj))
                } else {
                    None
                }
            })
            .collect();

        looked_objs.sort_by(|a, b| a.0.cmp(&b.0));
        looked_objs.sort_by_key(|a| match a.1 {
            GameObjE::Castle(_) => 0,
            GameObjE::Structure(_) => 1,
            GameObjE::DeployedUnits(_) => 2,
        });
        looked_objs
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.push_back(message.into());
    }
}
