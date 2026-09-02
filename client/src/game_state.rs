use std::collections::HashMap;

use common::{
    GameCoord, GameId, Time,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::Facility,
    game_objs::{GameObjE, OwnedCastleE},
    hitboxes::Hitbox,
    map::Tile,
    player::PlayerE,
};

use crate::r#const::LOGS_CAPACITY;
use crate::logs::Logs;

pub struct GameState {
    pub time: Time,
    pub map: Vec<Vec<Tile>>,
    pub hitbox_map: Vec<Vec<Option<GameId>>>,
    pub courtyard_hitbox_map: Vec<Vec<Option<GameId>>>,
    pub player: PlayerE,
    pub castle: Option<OwnedCastleE>,
    facilities: HashMap<usize, Facility>,
    objs: HashMap<GameId, GameObjE>,
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
        let facilities = HashMap::new();

        Self {
            time,
            hitbox_map: Self::init_hitbox_map(&objs, &map),
            courtyard_hitbox_map: Self::init_courtyard_hitbox_map(&facilities),
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

    pub fn get_obj(&self, obj_id: GameId) -> Option<&GameObjE> {
        self.objs.get(&obj_id)
    }

    pub fn get_facility(&self, facility_id: usize) -> Option<&Facility> {
        self.facilities.get(&facility_id)
    }

    pub fn get_objs(&self) -> &HashMap<GameId, GameObjE> {
        &self.objs
    }

    pub fn get_facilities(&self) -> &HashMap<GameId, Facility> {
        &self.facilities
    }

    pub fn update_objs(&mut self, objs: HashMap<GameId, GameObjE>) {
        self.hitbox_map = Self::init_hitbox_map(&objs, &self.map);
        self.objs = objs;
    }

    pub fn update_facilities(&mut self, facilities: HashMap<usize, Facility>) {
        for new_facility in facilities.iter() {
            if self.facilities.get(&new_facility.0).is_none() {
                let hitbox = new_facility.1.r#type.get_hitbox();
                let pos = new_facility.1.pos;

                Self::add_hitbox_entry(
                    &mut self.courtyard_hitbox_map,
                    hitbox,
                    pos,
                    *new_facility.0,
                );
            }
        }

        self.facilities = facilities;
    }

    fn init_hitbox_map(
        objs: &HashMap<GameId, GameObjE>,
        map: &Vec<Vec<Tile>>,
    ) -> Vec<Vec<Option<GameId>>> {
        let map_rows = map.len();
        let map_cols = map[0].len();

        let mut hitbox_map = vec![vec![None; map_cols]; map_rows];
        for obj in objs {
            let hitbox = obj.1.get_hitbox();
            let pos = obj.1.get_pos();

            Self::add_hitbox_entry(&mut hitbox_map, hitbox, pos, *obj.0);
        }

        hitbox_map
    }

    fn init_courtyard_hitbox_map(
        facilities: &HashMap<GameId, Facility>,
    ) -> Vec<Vec<Option<GameId>>> {
        let mut courtyard_hitbox_map = vec![vec![None; COURTYARD_COLS]; COURTYARD_ROWS];

        for facility in facilities {
            let hitbox = facility.1.r#type.get_hitbox();
            let pos = facility.1.pos;

            Self::add_hitbox_entry(&mut courtyard_hitbox_map, hitbox, pos, *facility.0);
        }

        courtyard_hitbox_map
    }

    fn add_hitbox_entry(
        hitbox_map: &mut Vec<Vec<Option<GameId>>>,
        hitbox_entry: Hitbox,
        pos: GameCoord,
        id: GameId,
    ) {
        for (i, row) in hitbox_entry.iter().enumerate() {
            for (j, hit) in row.iter().enumerate() {
                if *hit {
                    hitbox_map[pos.y + i][pos.x + j] = Some(id);
                }
            }
        }
    }
}
