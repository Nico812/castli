use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Receiver,
};

use crate::{
    game::{
        castle::Castle,
        game_obj::GameObj,
        map::Map,
        pathfinding::{self, a_star},
        units::{DeployedUnits, DeployedUnitsEvent},
    },
    thread_pool::ThreadPool,
};
use common::{
    GameCoord, GameId, Time,
    courtyard::FacilityType,
    game_objs::GameObjE,
    hitboxes::CASTLE_HITBOX,
    packets::MapPayload,
    units::{UnitGroup, UnitsTarget},
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
                    let units = depl_units.unit_group.clone();
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

        for (id, deployed_units) in units_to_home.into_iter() {
            let owner_id = deployed_units.get_owner_id();
            let Some(owner_castle) = self.get_castle_mut(owner_id) else {
                continue;
            };
            owner_castle.add_units(&deployed_units.unit_group);
            self.game_objs.remove_entry(&id);
        }

        for (id, mut deployed_units) in units_to_dest.into_iter() {
            match deployed_units.get_target() {
                UnitsTarget::Castle(_) => {
                    self.compute_attack(&mut deployed_units, &mut dead_castles);
                    if let Some(GameObj::DeployedUnits(obj)) = self.game_objs.get_mut(&id) {
                        *obj = deployed_units;
                    }
                }
                UnitsTarget::Coord(_) => {}
            }
        }

        self.time.tick();
        dead_castles
    }

    pub fn request_send_units(
        &mut self,
        castle_id: GameId,
        target: UnitsTarget,
        unit_group: UnitGroup,
        pool: &ThreadPool,
    ) -> bool {
        if unit_group.is_empty() {
            return false;
        }
        let Some(attacker_castle) = self.get_castle_mut(castle_id) else {
            return false;
        };
        if !attacker_castle.subtract_units_if_enough(&unit_group) {
            return false;
        }
        let attacker_pos = attacker_castle.get_pos();

        let deployed_units = DeployedUnits::new(castle_id, target, None, unit_group);

        let mut map_obstacles =
            deployed_units.get_obstacles(self.map.get_tiles(), self.map.get_occupied_map());

        // Need to remove starting and target position (if target is a solid object)
        // from obstacles or the pathfinding will not work
        let target_pos = match deployed_units.get_target() {
            UnitsTarget::Coord(target_pos) => target_pos,
            UnitsTarget::Castle(target_id) => {
                let Some(target_pos) = self.get_castle(target_id).map(|castle| castle.get_pos())
                else {
                    return false;
                };

                map_obstacles[target_pos.y][target_pos.x] = false;
                target_pos
            }
        };
        map_obstacles[attacker_pos.y][attacker_pos.x] = false;

        let id = Self::new_id(&mut self.id_cnt);
        self.game_objs
            .insert(id, GameObj::DeployedUnits(deployed_units));

        let task = PathTask::new(
            pool.execute_with_result(move || {
                pathfinding::a_star(attacker_pos, target_pos, map_obstacles)
            }),
            id,
        );
        self.pathfinding_tasks.push(task);
        true
    }

    pub fn add_player_castle(&mut self, name: String, pos: GameCoord) -> Option<GameId> {
        if !pos.is_even() || !self.map.can_build(pos, CASTLE_HITBOX) {
            return None;
        }
        self.map.set_occupied(pos, CASTLE_HITBOX);
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

    fn compute_attack(
        &mut self,
        deployed_units: &mut DeployedUnits,
        dead_castles: &mut Vec<GameId>,
    ) {
        let UnitsTarget::Castle(target_id) = deployed_units.get_target() else {
            return;
        };
        let Some(target) = self.get_castle_mut(target_id) else {
            return;
        };
        let target_str = target.get_strength();
        let attack_str = deployed_units.get_strength();
        if target_str < attack_str {
            let defender_unit_group = target.get_units();
            deployed_units
                .unit_group
                .subtract_if_enough(defender_unit_group);

            target.kill();
            dead_castles.push(target_id);
            println!("Someone died :(");
            println!("Attack str: {} | Def str: {}", attack_str, target_str);
        }
    }

    // TODO: Manage max amount of objects
    fn new_id(id_cnt: &mut GameId) -> GameId {
        *id_cnt += 1;
        *id_cnt
    }
}
