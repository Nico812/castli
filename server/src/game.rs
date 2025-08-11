//! # Core Game Logic and State
//!
//! This module defines the `Game` struct, which holds the entire state of a single
//! game instance, including the map, structures, and units. It also contains the
//! logic for procedural map generation.
use rand::Rng;
use std::collections::HashMap;

use crate::r#const::{CA_ITER, PERCENT_ARE_WALLS};
use common::{
    GameObjE,
    r#const::{MAP_COLS, MAP_ROWS},
};

enum GameObj {
    PlayerCastle(common::PlayerCastleE),
    Structure(common::StructureE),
    UnitGroup(common::UnitGroupE),
}

pub struct Game {
    map: Vec<Vec<common::TileE>>,
    game_objs: HashMap<common::ID, GameObj>,
}

impl Game {
    pub fn new() -> Self {
        let map = Self::cellular_automata();
        let mut game_objs = HashMap::new();

        // For debugging
        game_objs.insert(
            1,
            GameObj::PlayerCastle(common::PlayerCastleE {
                name: "nico".to_string(),
                pos: (2, 7),
            }),
        );

        Self { map, game_objs }
    }

    pub fn export_map(&self) -> Vec<Vec<common::TileE>> {
        self.map.clone()
    }

    pub fn export_objs(&self) -> HashMap<common::ID, common::GameObjE> {
        let mut exports = HashMap::new();
        for obj in &self.game_objs {
            let obj_e;
            match obj.1 {
                GameObj::PlayerCastle(castle) => {
                    println!("exporting a castle");
                    obj_e = GameObjE::PlayerCastle(castle.clone());
                }
                GameObj::Structure(structure) => obj_e = GameObjE::Structure(structure.clone()),
                GameObj::UnitGroup(unit_group) => obj_e = GameObjE::UnitGroup(unit_group.clone()),
            }
            exports.insert(*obj.0, obj_e);
        }
        exports
    }

    pub fn export_player_data(&self, id: common::ID) -> common::PlayerDataE {
        match &self.game_objs[&id] {
            GameObj::PlayerCastle(castle) => common::PlayerDataE {
                name: castle.name.clone(),
                pos: castle.pos,
            },
            _ => common::PlayerDataE {
                name: "undefined".to_string(),
                pos: (0, 0),
            },
        }
    }

    fn cellular_automata() -> Vec<Vec<common::TileE>> {
        fn fill_random(tiles: &mut Vec<Vec<common::TileE>>) {
            let mut rng = rand::rng();

            for row in 0..MAP_ROWS {
                for col in 0..MAP_COLS {
                    if row == 0
                        || col == 0
                        || row == MAP_ROWS - 1
                        || col == MAP_COLS - 1
                        || rng.random_range(1..=100) <= PERCENT_ARE_WALLS
                    {
                        tiles[row][col] = common::TileE::Water;
                    }
                }
            }
        }

        fn step_life(tiles: &Vec<Vec<common::TileE>>, temp_tiles: &mut Vec<Vec<common::TileE>>) {
            for row in 0..MAP_ROWS {
                for col in 0..MAP_COLS {
                    let mut wall_count = 0;

                    for i in row.saturating_sub(1)..=(row + 1).min(MAP_ROWS - 1) {
                        for j in col.saturating_sub(1)..=(col + 1).min(MAP_COLS - 1) {
                            if i == row && j == col {
                                continue;
                            }
                            if tiles[i][j] == common::TileE::Water {
                                wall_count += 1;
                            }
                        }
                    }

                    if wall_count >= 4 && temp_tiles[row][col] != common::TileE::Water {
                        temp_tiles[row][col] = common::TileE::Water;
                    }
                    if wall_count <= 2 && temp_tiles[row][col] == common::TileE::Water {
                        temp_tiles[row][col] = common::TileE::Grass;
                    }
                }
            }
        }

        let mut tiles = vec![vec![common::TileE::Grass; MAP_COLS]; MAP_ROWS];
        fill_random(&mut tiles);
        let mut temp_tiles = tiles.clone();

        for _ in 0..CA_ITER {
            step_life(&tiles, &mut temp_tiles);

            for row in 0..MAP_ROWS {
                for col in 0..MAP_COLS {
                    tiles[row][col] = temp_tiles[row][col];
                }
            }
        }
        tiles
    }

    fn print_map(tiles: &Vec<Vec<common::TileE>>) {
        for rows in tiles.iter() {
            for tile in rows {
                if *tile == common::TileE::Grass {
                    print!("G");
                } else {
                    print!("W");
                }
            }
            print!("\n");
        }
    }
}
