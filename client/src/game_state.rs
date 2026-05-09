use std::collections::HashMap;

use common::{
    GameCoord, GameId, Time,
    config::config,
    courtyard::Facility,
    game_objs::{GameObjE, OwnedCastleE},
    map::Tile,
    player::PlayerE,
};

use crate::logs::Logs;

pub struct GameState {
    pub time: Time,
    pub map: Vec<Vec<Tile>>,
    pub player: PlayerE,
    pub castle: Option<OwnedCastleE>,
    pub facilities: HashMap<u8, Facility>,
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
            logs: Logs::new(config().client.logs_capacity),
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

    pub fn get_facility(&self, facility_id: u8) -> Option<&Facility> {
        self.facilities.get(&facility_id)
    }
}
