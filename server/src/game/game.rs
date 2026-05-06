use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
};

use crate::{
    game::{
        castle::Castle,
        game_obj::GameObj,
        map::Map,
        pathfinding,
        units::{DeployedUnits, DeployedUnitsEvent},
    },
    thread_pool::ThreadPool,
};
use common::{
    GameCoord, GameId, Time, r#const::CASTLE_SIZE, courtyard::FacilityType, game_objs::GameObjE,
    packets::MapPayload, units::UnitGroup,
};

struct PathTask {
    pub units_id: GameId,
    pub rx: Receiver<Option<VecDeque<GameCoord>>>,
}

impl PathTask {
    fn new(rx: Receiver<Option<VecDeque<GameCoord>>>, units_id: GameId) -> Self {
        Self { rx, units_id }
    }
}

pub struct Game {
    map: Map,
    game_objs: HashMap<GameId, GameObj>,
    pathfinding_tasks: Vec<PathTask>,
    id_cnt: GameId,
    time: Time,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let game_objs = HashMap::new();
        let pathfinding_tasks = Vec::new();
        let id_cnt = 0;

        Self {
            map,
            game_objs,
            pathfinding_tasks,
            id_cnt,
            time: Time::new(),
        }
    }

    // Returns a collection of dead castles ids
    pub fn step(&mut self) -> Vec<GameId> {
        // Management of finished path tasks
        self.pathfinding_tasks.retain(|task| {
            let Ok(path) = task.rx.try_recv() else {
                return true;
            };

            let Some(GameObj::DeployedUnits(depl_units)) = self.game_objs.get_mut(&task.units_id)
            else {
                return false;
            };

            match path {
                // The path was found
                Some(path) => {
                    depl_units.set_path(path);
                }
                // No path found, give back units to owner
                None => {
                    let owner_id = depl_units.get_owner_id();
                    let units = depl_units.get_unit_group().clone();
                    if let Some(GameObj::Castle(owner)) = self.game_objs.get_mut(&owner_id) {
                        owner.add_units(&units);
                    }
                }
            }

            false
        });

        // Update game objects
        let mut dead_castles = Vec::new();
        let mut units_to_home = Vec::new();
        let mut units_to_dest = Vec::new();

        for (id, obj) in self.game_objs.iter_mut() {
            match obj {
                GameObj::DeployedUnits(deployed_units) => match deployed_units.step() {
                    Some(DeployedUnitsEvent::AtDest) => {
                        println!("SOME UNITS ARRIVED AT DEST, id:{}", id);
                        units_to_dest.push((*id, deployed_units.clone()));
                    }
                    Some(DeployedUnitsEvent::AtHome) => {
                        println!("SOME UNITS ARRIVED AT HOME, id:{}", id);
                        units_to_home.push((*id, deployed_units.clone()));
                    }
                    _ => {}
                },
                GameObj::Castle(castle) => castle.update(),
                _ => {}
            }
        }

        for (id, deployed_units) in units_to_home.iter() {
            let owner_id = deployed_units.get_owner_id();
            let Some(owner_castle) = self.get_castle_mut(owner_id) else {
                continue;
            };
            owner_castle.add_units(deployed_units.get_unit_group());
            self.game_objs.remove_entry(id);
        }
        for (_, deployed_units) in units_to_dest.iter() {
            let Some(target_id) = deployed_units.get_target() else {
                continue;
            };
            let Some(target) = self.get_castle_mut(target_id) else {
                continue;
            };
            let target_str = target.get_strength();
            let attack_str = deployed_units.get_strength();
            if target_str < attack_str {
                target.kill();
                dead_castles.push(target_id);
                println!("Someone died :(");
                println!("Attack str: {} | Def str: {}", attack_str, target_str);
            }
        }

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
        let Some(target_pos) = self.get_castle(target_id).map(|castle| castle.get_pos()) else {
            return false;
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
        if unit_group.is_empty() {
            return false;
        }

        if self.map.is_obstacle(target_pos) {
            return false;
        }

        let Some(attacker_castle) = self.get_castle_mut(attacker_id) else {
            return false;
        };

        if !attacker_castle.subtract_units_if_enough(&unit_group) {
            return false;
        }

        let deployed_units = DeployedUnits::new(attacker_id, target_id, None, unit_group);
        let attacker_pos = attacker_castle.get_pos();
        let map_obstacles = self.map.get_obstacles().clone();
        let id = Self::new_id(&mut self.id_cnt);

        self.game_objs
            .insert(id, GameObj::DeployedUnits(deployed_units));

        let task = PathTask::new(
            pool.execute_with_result(move || {
                pathfinding::a_star(attacker_pos, target_pos, &map_obstacles)
            }),
            id,
        );
        self.pathfinding_tasks.push(task);

        true
    }

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> Option<GameId> {
        if !pos.is_even() || !self.map.can_build(pos, CASTLE_SIZE) {
            return None;
        }
        self.map.set_occupied(pos, CASTLE_SIZE);
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
        let Some(castle) = self.get_castle_mut(castle_id) else {
            return false;
        };

        castle.new_facility(facility_type, pos)
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

    pub fn get_castle_mut(&mut self, castle_id: GameId) -> Option<&mut Castle> {
        self.game_objs
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

    pub fn get_time(&self) -> Time {
        self.time
    }

    pub fn export_map(&self) -> MapPayload {
        self.map.export()
    }

    pub fn export_objs(&self) -> HashMap<GameId, GameObjE> {
        self.game_objs
            .iter()
            .filter_map(|(&id, game_obj)| game_obj.export().map(|obj_e| (id, obj_e)))
            .collect()
    }

    // TODO: Manage max amount of objects
    fn new_id(id_cnt: &mut GameId) -> GameId {
        *id_cnt += 1;
        *id_cnt
    }
}
