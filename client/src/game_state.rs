use std::collections::{HashMap, VecDeque};

use common::{
    GameID, Time,
    exports::{client::ClientE, game_object::GameObjE, owned_castle::OwnedCastleE, tile::TileE},
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
    pub time: Time,
    pub map: Vec<Vec<TileE>>,
    pub client: ClientE,
    pub castle: Option<OwnedCastleE>,
    pub objs: HashMap<GameID, GameObjE>,
    pub logs: Logs,
}

impl GameState {
    pub fn new(
        time: Time,
        objs: HashMap<usize, GameObjE>,
        map: Vec<Vec<TileE>>,
        client: ClientE,
        castle: Option<OwnedCastleE>,
    ) -> Self {
        Self {
            time,
            map,
            client,
            castle,
            objs,
            logs: Logs::new(LOGS_CAPACITY),
        }
    }
    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.add(message.into());
    }
}
