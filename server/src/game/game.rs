//! # Core Game Logic and State
//!
//! This module defines the `Game` struct, which holds the entire state of a single
//! game instance, including the map, structures, and units. It also contains the
//! logic for procedural map generation.
use std::collections::HashMap;

use crate::game::{
    castle::Castle,
    game_obj::GameObj,
    map::Map,
    units::{DeployedUnits, UnitGroup},
};
use common::{GameCoord, GameID, GameObjE, PlayerE, TileE};

pub struct Game {
    map: Map,
    game_objs: HashMap<GameID, GameObj>,
    id_counter: GameID,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let game_objs = HashMap::new();
        let id_counter = 0;

        Self {
            map,
            game_objs,
            id_counter,
        }
    }

    pub fn step(&mut self) {
        for obj in self.game_objs.values_mut() {
            if let GameObj::DeployedUnits(deployed_units) = obj {
                deployed_units.move_along_path();
            }
        }
    }

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> GameID {
        let id = self.id_counter;
        self.id_counter += 1;

        let castle = Castle::new(name, pos);
        self.game_objs.insert(id, GameObj::Castle(castle));
        id
    }

    pub fn export_map(&self) -> Vec<Vec<TileE>> {
        self.map.export()
    }

    pub fn export_objs(&self) -> HashMap<GameID, GameObjE> {
        self.game_objs
            .iter()
            .map(|(&id, game_obj)| {
                let obj_e = game_obj.export();
                (id, obj_e)
            })
            .collect()
    }

    pub fn export_player(&self, id: GameID) -> PlayerE {
        match self.game_objs.get(&id) {
            Some(GameObj::Castle(castle)) => PlayerE {
                id: id,
                name: castle.name.clone(),
                pos: castle.pos,
            },
            _ => PlayerE {
                id: 0,
                name: "undefined".to_string(),
                pos: GameCoord { x: 0, y: 0 },
            },
        }
    }

    pub fn attack_castle(&mut self, attacker_id: GameID, target_id: GameID) {
        let (attacker_pos, attacker_name) =
            if let Some(GameObj::Castle(castle)) = self.game_objs.get(&attacker_id) {
                (castle.pos, castle.name.clone())
            } else {
                return;
            };

        let target_pos = if let Some(GameObj::Castle(castle)) = self.game_objs.get(&target_id) {
            castle.pos
        } else {
            return;
        };

        let path = self.map.find_path(attacker_pos, target_pos);
        println!("{:?}", path);

        if let Some(path) = path {
            let id = self.id_counter;
            self.id_counter += 1;

            let unit_group = UnitGroup::new();
            let deployed_units = DeployedUnits::new(attacker_name, attacker_pos, path, unit_group);
            self.game_objs
                .insert(id, GameObj::DeployedUnits(deployed_units));
        }
    }
}
