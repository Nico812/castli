//! # Core Game Logic and State
//!
//! This module defines the `Game` struct, which holds the entire state of a single
//! game instance, including the map, structures, and units. It also contains the
//! logic for procedural map generation.
use std::collections::HashMap;

use rand::Rng;

use crate::{
    r#const::{CA_ITER, PERCENT_ARE_WALLS},
    game::{map, player_castle},
};
use common::{
    r#const::{MAP_COLS, MAP_ROWS},
    GameObjE, PlayerCastleE, PlayerDataE, StructureE, TileE, UnitGroupE, GameID,
};

enum GameObj {
    PlayerCastle(PlayerCastle),
    Structure(StructureE),
    UnitGroup(UnitGroupE),
}

pub struct Game {
    map: Map,
    game_objs: HashMap<GameID, GameObj>,
    id_counter: GameID,
}

impl Game {
    pub fn new() -> Self {
        let map = map::Map::new();
        let game_objs = HashMap::new();
        let id_counter = 0;

        Self {
            map,
            game_objs,
            id_counter,
        }
    }

    pub fn add_player_castle(&mut self, name: &String, pos: (usize, usize)) -> GameID {
        let id = self.id_counter;
        self.id_counter += 1;

        let castle = PlayerCastle {
            name: name.to_string(),
            pos,
        };
        self.game_objs.insert(id, GameObj::PlayerCastle(castle));
        id
    }

    pub fn export_map(&self) -> Vec<Vec<TileE>> {
        self.map.export()
    }

    pub fn export_objs(&self) -> HashMap<GameID, GameObjE> {
        self.game_objs
            .iter()
            .map(|(&id, game_obj)| {
                let obj_e = match game_obj {
                    GameObj::PlayerCastle(castle) => {
                        println!("exporting a castle");
                        let castle_export = PlayerCastleE {
                            name: castle.name.clone(),
                            pos: castle.pos,
                        };
                        GameObjE::PlayerCastle(castle_export)
                    }
                    GameObj::Structure(structure) => GameObjE::Structure(structure.clone()),
                    GameObj::UnitGroup(unit_group) => GameObjE::UnitGroup(unit_group.clone()),
                };
                (id, obj_e)
            })
            .collect()
    }

    pub fn export_player_data(&self, id: GameID) -> PlayerDataE {
        println!(
            "Game is trying to export player_data for client_id {:?}",
            id
        );
        match self.game_objs.get(&id) {
            Some(GameObj::PlayerCastle(castle)) => PlayerDataE {
                id,
                name: castle.name.clone(),
                pos: castle.pos,
            },
            _ => PlayerDataE {
                id: 0,
                name: "undefined".to_string(),
                pos: (0, 0),
            },
        }
    }
}