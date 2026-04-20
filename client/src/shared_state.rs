use std::collections::{HashMap, VecDeque};

use common::{
    GameCoord, GameID,
    exports::{game_object::GameObjE, player::PlayerE},
};

use crate::game_renderer::ModRightTab;

// Variables shared between input handler, renderer, server comunication handler.
pub struct SharedState {
    pub game_objs: HashMap<GameID, GameObjE>,
    pub logs: VecDeque<String>,
    pub player_data: PlayerE,
    pub map_zoom: Option<GameCoord>,
    pub map_look: Option<GameCoord>,
    pub mod_right_tab: ModRightTab,
    pub inspect_select: Option<InspectSelect>,
    pub interact_target: Option<InteractTarget>,
}

pub struct InspectSelect {
    pub next: bool,
    pub prev: bool,
    pub obj_id: Option<GameID>,
}

pub struct InteractTarget {
    pub obj: Option<GameObjE>,
    pub pos: GameCoord,
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
            map_look: None,
            logs: VecDeque::new(),
            mod_right_tab: ModRightTab::Castle,
            inspect_select: None,
            interact_target: None,
        }
    }

    pub fn get_looked_obj(&self) -> Option<(&GameID, &GameObjE)> {
        match self.map_look {
            Some(look_pos) => self
                .game_objs
                .iter()
                .find(|(_, obj)| obj.get_pos() == look_pos),
            None => None,
        }
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.push_back(message.into());
    }
}
