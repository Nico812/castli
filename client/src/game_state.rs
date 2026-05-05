use std::collections::{HashMap, VecDeque};

use common::{
    GameCoord, GameId, Time,
    courtyard::{Facility, FacilityType},
    game_objs::{GameObjE, OwnedCastleE},
    map::Tile,
    player::PlayerE,
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
    pub map: Vec<Vec<Tile>>,
    pub player: PlayerE,
    pub castle: Option<OwnedCastleE>,
    pub facilities: HashMap<GameId, Facility>,
    pub objs: HashMap<GameId, GameObjE>,
    pub logs: Logs,
}

impl GameState {
    pub fn new(
        time: Time,
        objs: HashMap<usize, GameObjE>,
        map: Vec<Vec<Tile>>,
        player: PlayerE,
        castle: Option<OwnedCastleE>,
    ) -> Self {
        Self {
            time,
            map,
            player,
            castle,
            facilities: HashMap::new(),
            objs,
            logs: Logs::new(LOGS_CAPACITY),
        }
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.add(message.into());
    }

    pub fn get_tile(&self, coord: GameCoord) -> Tile {
        self.map
            .get(coord.y)
            .and_then(|row| row.get(coord.x))
            .copied()
            .unwrap_or(Tile::Err)
    }

    pub fn get_facility(&self, facility_id: GameId) -> Option<&Facility> {
        self.facilities.get(&facility_id)
    }
}
