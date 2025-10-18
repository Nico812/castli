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
        let mut pending_units_ids = vec![];

        for (id, obj) in self.game_objs.values_mut().enumerate() {
            match obj {
                GameObj::DeployedUnits(deployed_units) => {
                    if deployed_units.pending() {
                        pending_units_ids.push(id);
                    } else {
                        deployed_units.move_along_path();
                    }
                }
                _ => {}
            }
        }

        self.resolve_pending_units(pending_units_ids);
    }

    fn resolve_pending_units(&mut self, pending_units_ids: Vec<GameID>) {
        let mut to_home = vec![];
        let mut to_target = vec![];

        for units_id in pending_units_ids {
            if let Some(GameObj::DeployedUnits(units)) = self.game_objs.get_mut(&units_id) {
                if units.arrived_home() {
                    to_home.push((units_id, units.owner_id, units.unit_group.clone()));
                } else if units.arrived_target() {
                    if let Some(target_id) = units.target_id {
                        to_target.push((target_id, units.get_strength()));
                    }
                    units.r#return();
                }
            }
        }

        for (units_id, owner_id, units) in to_home {
            if let Some(GameObj::Castle(owner)) = self.game_objs.get_mut(&owner_id) {
                owner.units.saturating_add(&units);
            }
            self.game_objs.remove(&units_id);
        }

        for (target_id, strength) in to_target {
            if let Some(GameObj::Castle(target)) = self.game_objs.get_mut(&target_id) {
                let target_strength = target.units.get_strength();
                if target_strength < strength {
                    target.is_alive = false;
                }
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
        self.send_troops(attacker_id, target_pos, unit_group_e, Some(target_id));
    }

    pub fn send_troops(
        &mut self,
        attacker_id: GameID,
        target_pos: GameCoord,
        unit_group_e: UnitGroupE,
        target_id: Option<GameID>,
    ) {
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

        if let Some(path) = self.map.find_path(attacker_castle.pos, target_pos) {
            let id = self.new_id();
            let deployed_units = DeployedUnits::new(attacker_id, target_id, path, unit_group);
            self.game_objs
                .insert(id, GameObj::DeployedUnits(deployed_units));
        }
    }

    pub fn is_alive(&self, castle_id: &GameID) -> bool {
        if let Some(GameObj::Castle(castle)) = self.game_objs.get(castle_id) {
            return castle.is_alive;
        }
        false
    }
}
