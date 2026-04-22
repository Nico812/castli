use std::collections::{HashMap, VecDeque};

use common::{
    GameID,
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE},
};

use crate::r#const::LOGS_CAPACITY;

pub struct Logs {
    pub content: VecDeque<String>,
    max_len: usize,
}

impl Logs {
    fn new(max_len: usize) -> Self {
        Self {
            content: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    fn add(&mut self, item: String) {
        if self.content.len() >= self.max_len {
            let _ = self.content.pop_front();
        }
        self.content.push_back(item);
    }
}

pub struct GameState {
    pub map: Vec<Vec<TileE>>,
    pub player: PlayerE,
    pub objs: HashMap<GameID, GameObjE>,
    pub logs: Logs,
}

impl GameState {
    pub fn new(
        objs: HashMap<usize, GameObjE>,
        player: Option<PlayerE>,
        map: Vec<Vec<TileE>>,
    ) -> Self {
        Self {
            map,
            player: player.unwrap_or(PlayerE::undef()),
            objs,
            logs: Logs::new(LOGS_CAPACITY),
        }
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.add(message.into());
    }
}
