use std::collections::{HashMap, VecDeque};

use crate::game::{
    castle::Castle,
    game_obj::GameObj,
    map::Map,
    pathfinding,
    units::{DeployedUnits, UnitGroup},
};
use common::{
    GameCoord, GameID, Time,
    exports::{game_object::GameObjE, owned_castle::OwnedCastleE, tile::TileE, units::UnitGroupE},
};
use tokio::task::JoinHandle;

pub struct Game {
    map: Map,
    game_objs: HashMap<GameID, GameObj>,
    incomp_game_objs: HashMap<GameID, GameObj>,
    pathfinding_tasks: HashMap<GameID, JoinHandle<Option<VecDeque<GameCoord>>>>,
    id_cnt: GameID,
    pub time: Time,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let game_objs = HashMap::new();
        let incomp_game_objs = HashMap::new();
        let pathfinding_tasks = HashMap::new();
        let id_cnt = 0;

        Self {
            map,
            game_objs,
            incomp_game_objs,
            pathfinding_tasks,
            id_cnt,
            time: Time::new(),
        }
    }

    pub async fn step(&mut self) -> Vec<GameID> {
        let mut pending_units_ids = vec![];

        // Give computed paths to units
        let finished_path_tasks: Vec<_> = self
            .pathfinding_tasks
            .iter_mut()
            .filter(|(_, task)| task.is_finished())
            .map(|(units_id, _)| *units_id)
            .collect();

        for units_id in finished_path_tasks {
            if let Some(GameObj::DeployedUnits(mut depl_units)) =
                self.incomp_game_objs.remove(&units_id)
                && let Some(task) = self.pathfinding_tasks.remove(&units_id)
            {
                if let Ok(Some(path)) = task.await {
                    depl_units.set_path(path);
                    self.game_objs
                        .insert(units_id, GameObj::DeployedUnits(depl_units));
                } else {
                    if let Some(GameObj::Castle(owner)) =
                        self.game_objs.get_mut(&depl_units.owner_id)
                    {
                        owner.units.saturating_add(&depl_units.unit_group);
                    }
                }
            }
        }

        // Solves units arrived at destination
        for (id, obj) in self.game_objs.iter_mut() {
            if let GameObj::DeployedUnits(deployed_units) = obj {
                if deployed_units.pending() {
                    pending_units_ids.push(*id);
                } else {
                    deployed_units.move_along_path();
                }
            }
        }

        let dead_castles = if !pending_units_ids.is_empty() {
            self.resolve_pending_units(&pending_units_ids)
        } else {
            Vec::new()
        };

        self.time.tick();
        dead_castles
    }

    pub fn attack_castle(
        &mut self,
        attacker_id: GameID,
        target_id: GameID,
        unit_group_e: UnitGroupE,
    ) -> bool {
        let target_pos = match self.game_objs.get(&target_id) {
            Some(GameObj::Castle(c)) => c.pos,
            _ => return false,
        };
        self.request_send_units(attacker_id, target_pos, unit_group_e, Some(target_id))
    }

    pub fn request_send_units(
        &mut self,
        attacker_id: GameID,
        target_pos: GameCoord,
        unit_group_e: UnitGroupE,
        target_id: Option<GameID>,
    ) -> bool {
        let attacker_castle = match self.game_objs.get_mut(&attacker_id) {
            Some(GameObj::Castle(c)) => c,
            _ => {
                return false;
            }
        };
        let unit_group = UnitGroup::from_export(unit_group_e);

        if unit_group.is_empty() {
            return false;
        }

        if self.map.is_obstacle(target_pos) {
            return false;
        }

        if !attacker_castle.units.subtract_if_enough(&unit_group) {
            return false;
        }

        let deployed_units = DeployedUnits::new(attacker_id, target_id, None, unit_group);
        let map_obstacles = self.map.obstacles.clone();
        let attacker_pos = attacker_castle.pos;
        let id = self.new_id();

        self.incomp_game_objs
            .insert(id, GameObj::DeployedUnits(deployed_units));

        let task = tokio::task::spawn_blocking(move || {
            pathfinding::a_star(attacker_pos, target_pos, &map_obstacles)
        });
        self.pathfinding_tasks.insert(id, task);

        true
    }

    fn resolve_pending_units(&mut self, pending_units_ids: &Vec<GameID>) -> Vec<GameID> {
        let mut to_home = vec![];
        let mut to_attack = vec![];
        let mut dead_castles = vec![];

        for units_id in pending_units_ids {
            println!("SOME UNITS ARRIVED AT DEST, id:{}", units_id);
            if let Some(GameObj::DeployedUnits(units)) = self.game_objs.get_mut(units_id) {
                if units.arrived_home() {
                    to_home.push((units_id, units.owner_id, units.unit_group.clone()));
                } else if units.arrived_target() {
                    if let Some(target_id) = units.target_id {
                        to_attack.push((target_id, units.get_strength()));
                    }
                    units.r#return();
                }
            }
        }
        println!(
            "found {} pending units arrived home, and {} pending units arrived at target",
            to_home.len(),
            to_attack.len()
        );

        for (units_id, owner_id, units) in to_home {
            if let Some(GameObj::Castle(owner)) = self.game_objs.get_mut(&owner_id) {
                owner.units.saturating_add(&units);
            }
            self.game_objs.remove(units_id);
        }
        for (target_id, strength) in to_attack {
            if let Some(GameObj::Castle(target)) = self.game_objs.get_mut(&target_id) {
                let target_strength = target.units.get_strength();
                if target_strength < strength {
                    target.alive = false;
                    dead_castles.push(target_id);
                    println!("Someone died :(");
                    println!("Attack str: {} | Def str: {}", strength, target_strength);
                }
            }
        }
        dead_castles
    }

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> Option<GameID> {
        if self.map.is_obstacle(pos) {
            return None;
        }
        let id = self.new_id();
        let castle = Castle::new(name, pos);
        self.game_objs.insert(id, GameObj::Castle(castle));
        Some(id)
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

    pub fn export_owned_castle(&self, castle_id: GameID) -> Option<OwnedCastleE> {
        self.game_objs
            .iter()
            .find(|obj| *obj.0 == castle_id)
            .and_then(|obj| {
                if let GameObj::Castle(castle) = obj.1 {
                    Some(castle.export_owned())
                } else {
                    None
                }
            })
    }

    // TODO: Manage max amount of objects
    fn new_id(&mut self) -> GameID {
        self.id_cnt += 1;
        self.id_cnt
    }
}
