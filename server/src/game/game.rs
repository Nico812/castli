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
        structure::Structure,
        units::{DeployedUnits, DeployedUnitsEvent},
    },
    thread_pool::ThreadPool,
};
use common::{
    GameCoord, GameId, LimitedGroup, ResourceType, Time, UnitType, UnitsTarget,
    courtyard::FacilityType,
    game_objs::{GameObjE, StructureType},
    hitboxes::{CASTLE_HITBOX, STRUCTURE_HITBOX},
    map::Tile,
    packets::MapPayload,
};
use rand::{SeedableRng, TryRngCore, rngs::SmallRng};

pub enum KillRewards {
    Resources(LimitedGroup<ResourceType>),
    Units(LimitedGroup<UnitType>),
}

pub trait Attackable {
    fn get_strength(&self) -> u32;
    fn kill(&mut self);
    fn get_kill_rewards(&self) -> Vec<KillRewards>;
}

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
    rng: SmallRng,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let game_objs = HashMap::new();
        let pathfinding_tasks = Vec::new();
        let id_cnt = 0;
        let rng = SmallRng::seed_from_u64(60);

        Self {
            map,
            game_objs,
            pathfinding_tasks,
            id_cnt,
            time: Time::new(),
            rng,
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
            if let Some(ref resources) = deployed_units.wagon_contents {
                owner_castle.add_resources(resources);
            };
            self.game_objs.remove_entry(&id);
        }

        for (id, mut deployed_units) in units_to_dest.into_iter() {
            match deployed_units.get_target() {
                UnitsTarget::GameObj(_) => {
                    self.compute_attack(&mut deployed_units, &mut dead_castles);
                    if deployed_units.unit_group.is_empty() {
                        self.game_objs.remove_entry(&id);
                    } else if let Some(GameObj::DeployedUnits(obj)) = self.game_objs.get_mut(&id) {
                        *obj = deployed_units;
                    }
                }
                UnitsTarget::Coord(_) => {}
            }
        }

        // Spawn structures
        if self.time.tick_cnt == 0 && self.time.h == 12 {
            self.spawn_structures();
        };

        self.time.tick();
        dead_castles
    }

    pub fn request_send_units(
        &mut self,
        castle_id: GameId,
        target: UnitsTarget,
        unit_group: LimitedGroup<UnitType>,
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
            UnitsTarget::GameObj(target_id) => {
                let Some(target) = self.get_obj(target_id) else {
                    return false;
                };
                let Some(target_pos) = target.get_pos() else {
                    return false;
                };
                if !target.is_attackable() {
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

    // ATK: 10
    // DEF: 40
    // FACTOR1: 4
    // FACTOR2: 1

    // if FACTOR2 == 1 => ATK troops destroyes and DEF troops -= DEF troops/FACTOR1
    // else if FACTOR1 == 1 => DEF troops destroyed and dies

    // ATK: 40
    // DEF: 10
    // FACTOR2: 4
    // FACTOR1: 1

    pub fn compute_attack(
        &mut self,
        deployed_units: &mut DeployedUnits,
        dead_castles: &mut Vec<GameId>,
    ) {
        let UnitsTarget::GameObj(target_id) = deployed_units.get_target() else {
            return;
        };

        let Some(def_obj) = self.get_obj_mut(target_id) else {
            return;
        };

        let def_str = match &*def_obj {
            GameObj::Castle(target) => target.get_strength(),
            GameObj::Structure(target) => target.get_strength(),
            _ => return,
        };

        let atk_str = deployed_units.get_strength();
        let factor1 = def_str.div_ceil(atk_str);
        let factor2 = atk_str.div_ceil(def_str);

        match (factor1, factor2) {
            (_, 1) => {
                // Attack failed. All attacking troops die.
                deployed_units.unit_group = LimitedGroup::new(None);

                if let GameObj::Castle(castle) = &mut *def_obj {
                    let units = castle.get_units_mut();
                    units.subtract(&(units.clone() / factor1));
                }
            }
            (1, _) => {
                // Attack succeeded.
                deployed_units
                    .unit_group
                    .subtract(&(deployed_units.unit_group.clone() / factor2));

                let rewards = match &mut *def_obj {
                    GameObj::Castle(castle) => {
                        castle.kill();
                        dead_castles.push(target_id);
                        println!("Someone died :(");
                        castle.get_kill_rewards()
                    }
                    GameObj::Structure(structure) => {
                        structure.kill();
                        structure.get_kill_rewards()
                    }
                    _ => unreachable!("target type was checked above"),
                };

                for reward in rewards {
                    match reward {
                        KillRewards::Resources(resource_group) => {
                            if deployed_units.unit_group.contains(UnitType::Wagon) {
                                deployed_units.wagon_contents = Some(resource_group);
                            }
                        }
                        KillRewards::Units(unit_group) => {
                            deployed_units.unit_group.add(&unit_group);
                        }
                    };
                }
            }
            _ => println!("WTF there is a problem with the factors in compute attack"),
        }
    }

    // TODO: Manage max amount of objects
    fn new_id(id_cnt: &mut GameId) -> GameId {
        *id_cnt += 1;
        *id_cnt
    }

    fn spawn_structures(&mut self) {
        let structure_types = [
            StructureType::LoneFarm,
            StructureType::NonEuclideanTown,
            StructureType::DragonsLair,
        ];
        let structure_params: Vec<(Vec<Tile>, u32)> = structure_types
            .iter()
            .map(|&ty| (Structure::spawns_on(ty), Structure::spawn_prob(ty)))
            .collect();

        for (i, tile_row) in self.map.get_tiles().clone().iter().enumerate() {
            for (j, tile) in tile_row.iter().enumerate() {
                for (k, params) in structure_params.iter().enumerate() {
                    if params.0.contains(tile)
                        && Ok(0) == self.rng.try_next_u32().map(|numb| numb % params.1)
                    {
                        let ty = structure_types[k];
                        let pos = GameCoord::new(i, j);

                        self.add_structure(ty, pos);
                    }
                }
            }
        }
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

    pub fn add_structure(&mut self, ty: StructureType, pos: GameCoord) -> Option<GameId> {
        if !pos.is_even() || !self.map.can_build(pos, STRUCTURE_HITBOX) {
            return None;
        }
        self.map.set_occupied(pos, STRUCTURE_HITBOX);
        let id = Self::new_id(&mut self.id_cnt);
        let structure = Structure::new(ty, pos);
        self.game_objs.insert(id, GameObj::Structure(structure));
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

    pub fn upgrade_facility(&mut self, castle_id: GameId, facility_id: usize) -> bool {
        let Some(castle) = self.get_castle_mut(castle_id) else {
            return false;
        };

        castle.upgrade_facility(facility_id)
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

    pub fn get_obj_mut(&mut self, id: GameId) -> Option<&mut GameObj> {
        self.game_objs
            .iter_mut()
            .find(|obj| *obj.0 == id)
            .and_then(|(_, obj)| Some(obj))
    }

    pub fn get_obj(&self, id: GameId) -> Option<&GameObj> {
        self.game_objs
            .iter()
            .find(|obj| *obj.0 == id)
            .and_then(|(_, obj)| Some(obj))
    }

    pub fn get_castle_mut(&mut self, id: GameId) -> Option<&mut Castle> {
        self.get_obj_mut(id)
            .map(|obj| {
                if let GameObj::Castle(castle) = obj {
                    Some(castle)
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn get_structure_mut(&mut self, id: GameId) -> Option<&mut Structure> {
        self.get_obj_mut(id)
            .map(|obj| {
                if let GameObj::Structure(structure) = obj {
                    Some(structure)
                } else {
                    None
                }
            })
            .flatten()
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
}
