use std::collections::{HashMap, VecDeque};

use common::{
    GameCoord, GameID,
    exports::{game_object::GameObjE, player::PlayerE},
};

use crate::game_renderer::{ModRightTab, game_renderer::GameRenderer};

// Variables shared between input handler, renderer, server comunication handler.
pub struct SharedState {
    pub game_objs: HashMap<GameID, GameObjE>,
    pub logs: VecDeque<String>,
    pub player_data: PlayerE,
    pub map_zoom: Option<GameCoord>,
    pub map_look: Option<GameCoord>,
    pub mod_right_tab: ModRightTab,
    pub inspect_select: Option<GameID>,
    pub interact_target: Option<InteractTarget>,
}

pub struct InteractTarget {
    pub obj_id: Option<GameID>,
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

    pub fn get_looked_objs(&self) -> Vec<(GameID, &GameObjE)> {
        let Some(look_coord) = self.map_look else {
            return Vec::new();
        };

        let mut looked_objs: Vec<(GameID, &GameObjE)> = self
            .game_objs
            .iter()
            .filter_map(|(game_id, game_obj)| {
                if self.map_zoom.is_some() && game_obj.get_pos() == look_coord {
                    Some((*game_id, game_obj))
                } else if self.map_zoom.is_none()
                    && game_obj.get_pos().y >= look_coord.y
                    && game_obj.get_pos().x >= look_coord.x
                    && game_obj.get_pos().y < look_coord.y + GameRenderer::ZOOM_FACTOR
                    && game_obj.get_pos().x < look_coord.x + GameRenderer::ZOOM_FACTOR
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
