use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
};

use crate::{
    game::{castle::Castle, game_obj::GameObj, map::Map, pathfinding, units::DeployedUnits},
    thread_pool::ThreadPool,
};
use common::{
    GameCoord, GameId, Time,
    courtyard::{Facility, FacilityType},
    game_objs::{GameObjE, OwnedCastleE},
    map::Tile,
    units::UnitGroup,
};

struct PathTask {
    pub units_id: GameId,
    pub rx: Receiver<Option<VecDeque<GameCoord>>>,
    pub completed: bool,
}

impl PathTask {
    fn new(rx: Receiver<Option<VecDeque<GameCoord>>>, units_id: GameId) -> Self {
        Self {
            rx,
            units_id,
            completed: false,
        }
    }
}

pub struct Game {
    map: Map,
    game_objs: HashMap<GameId, GameObj>,
    incomp_game_objs: HashMap<GameId, GameObj>,
    pathfinding_tasks: Vec<PathTask>,
    id_cnt: GameId,
    pub time: Time,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let game_objs = HashMap::new();
        let incomp_game_objs = HashMap::new();
        let pathfinding_tasks = Vec::new();
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

    pub fn step(&mut self) -> Vec<GameId> {
        // Manages finished path tasks
        let mut finished_path_tasks = Vec::new();
        for task in &mut self.pathfinding_tasks {
            if let Ok(result) = task.rx.try_recv() {
                finished_path_tasks.push((task.units_id, result));
                task.completed = true;
            }
        }
        self.pathfinding_tasks.retain(|task| !task.completed);

        for (units_id, path) in finished_path_tasks {
            if let Some(GameObj::DeployedUnits(mut depl_units)) =
                self.incomp_game_objs.remove(&units_id)
            {
                match path {
                    Some(path) => {
                        depl_units.set_path(path);
                        self.game_objs
                            .insert(units_id, GameObj::DeployedUnits(depl_units));
                    }
                    None => {
                        if let Some(GameObj::Castle(owner)) =
                            self.game_objs.get_mut(&depl_units.owner_id)
                        {
                            owner.units.saturating_add(&depl_units.unit_group);
                        }
                    }
                }
            }
        }

        // Manages units arrived at destination
        let mut pending_units_ids = vec![];

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
        attacker_id: GameId,
        target_id: GameId,
        unit_group_e: UnitGroup,
        pool: &ThreadPool,
    ) -> bool {
        let target_pos = match self.game_objs.get(&target_id) {
            Some(GameObj::Castle(c)) => c.pos,
            _ => return false,
        };
        self.request_send_units(attacker_id, target_pos, unit_group_e, Some(target_id), pool)
    }

    pub fn request_send_units(
        &mut self,
        attacker_id: GameId,
        target_pos: GameCoord,
        unit_group: UnitGroup,
        target_id: Option<GameId>,
        pool: &ThreadPool,
    ) -> bool {
        let Some(attacker_castle) = Self::get_castle_mut(&mut self.game_objs, attacker_id) else {
            return false;
        };

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
        let id = Self::new_id(&mut self.id_cnt);

        self.incomp_game_objs
            .insert(id, GameObj::DeployedUnits(deployed_units));

        let task = PathTask::new(
            pool.execute_tiwh_result(move || {
                pathfinding::a_star(attacker_pos, target_pos, &map_obstacles)
            }),
            id,
        );
        self.pathfinding_tasks.push(task);

        true
    }

    fn resolve_pending_units(&mut self, pending_units_ids: &Vec<GameId>) -> Vec<GameId> {
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

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> Option<GameId> {
        if self.map.is_obstacle(pos) {
            return None;
        }
        let id = Self::new_id(&mut self.id_cnt);
        let castle = Castle::new(name, pos);
        self.game_objs.insert(id, GameObj::Castle(castle));
        Some(id)
    }

    pub fn add_facility(
        &mut self,
        castle_id: GameId,
        facility_type: FacilityType,
        pos: GameCoord,
    ) -> bool {
        let Some(castle) = Self::get_castle_mut(&mut self.game_objs, castle_id) else {
            return false;
        };

        let facility = Facility::new(facility_type, pos);
        let id = Self::new_id(&mut self.id_cnt);

        castle.courtyard.add(&mut castle.resources, facility, id)
    }

    pub fn get_castle(&self, castle_id: GameId) -> Option<&Castle> {
        self.game_objs
            .iter()
            .find(|obj| *obj.0 == castle_id)
            .and_then(|obj| {
                if let GameObj::Castle(castle) = obj.1 {
                    Some(castle)
                } else {
                    None
                }
            })
    }

    pub fn get_castle_mut(
        game_objs: &mut HashMap<GameId, GameObj>,
        castle_id: GameId,
    ) -> Option<&mut Castle> {
        game_objs
            .iter_mut()
            .find(|obj| *obj.0 == castle_id)
            .and_then(|obj| {
                if let GameObj::Castle(castle) = obj.1 {
                    Some(castle)
                } else {
                    None
                }
            })
    }

    pub fn export_map(&self) -> Vec<Vec<Tile>> {
        self.map.export()
    }

    pub fn export_objs(&self) -> HashMap<GameId, GameObjE> {
        self.game_objs
            .iter()
            .map(|(&id, game_obj)| {
                let obj_e = game_obj.export();
                (id, obj_e)
            })
            .collect()
    }

    // TODO: Manage max amount of objects
    fn new_id(id_cnt: &mut GameId) -> GameId {
        *id_cnt += 1;
        *id_cnt
    }
}
