//! # Core Game Logic and State
//!
//! This module defines the `Game` struct, which holds the entire state of a single
//! game instance, including the map, structures, and units. It also contains the
//! logic for procedural map generation.
use std::{any::Any, collections::HashMap};

use crate::game::{
    castle::Castle,
    game_obj::GameObj,
    map::Map,
    units::{DeployedUnits, UnitGroup},
};
use common::{
    GameCoord, GameID,
    exports::{game_object::GameObjE, player::PlayerE, tile::TileE, units::UnitGroupE},
};

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

    fn new_id(&mut self) -> GameID {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn step(&mut self) {
        for obj in self.game_objs.values_mut() {
            if let GameObj::DeployedUnits(deployed_units) = obj {
                deployed_units.move_along_path();
            }
        }
    }

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> GameID {
        let id = self.new_id();

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

    pub fn export_player(&self, castle_id: GameID) -> PlayerE {
        match self.game_objs.get(&castle_id) {
            Some(GameObj::Castle(castle)) => PlayerE {
                id: castle_id,
                name: castle.name.clone(),
                pos: castle.pos,
                units: castle.units.export(),
                peasants: castle.peasants,
            },
            _ => PlayerE::undef(),
        }
    }

    pub fn attack_castle(
        &mut self,
        attacker_id: GameID,
        target_id: GameID,
        unit_group_e: UnitGroupE,
    ) {
        let target_pos = match self.game_objs.get(&target_id) {
            Some(GameObj::Castle(c)) => c.pos,
            _ => return,
        };

        let attacker_castle = match self.game_objs.get_mut(&attacker_id) {
            Some(GameObj::Castle(c)) => c,
            _ => return,
        };

        let unit_group = UnitGroup::from_export(unit_group_e);
        if unit_group.is_empty() {
            return;
        }

        if !attacker_castle.units.subtract_if_enough(&unit_group) {
            return;
        }

        let attacker_name = attacker_castle.name.clone();
        let attacker_pos = attacker_castle.pos;
        if let Some(path) = self.map.find_path(attacker_castle.pos, target_pos) {
            let id = self.new_id();
            let deployed_units = DeployedUnits::new(attacker_name, attacker_pos, path, unit_group);
            self.game_objs
                .insert(id, GameObj::DeployedUnits(deployed_units));
        }
    }
}
