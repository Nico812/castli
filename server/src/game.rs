//! # Core Game Logic and State
//!
//! This module defines the `Game` struct, which holds the entire state of a single
//! game instance, including the map, structures, and units. It also contains the
//! logic for procedural map generation.
use rand::Rng;

use crate::r#const::{CA_ITER, PERCENT_ARE_WALLS};
use common::r#const::{MAP_COLS, MAP_ROWS};

pub struct Game {
    map: Vec<Vec<common::TileE>>,
    structures: Vec<common::StructureE>,
    unit_groups: Vec<common::UnitGroupE>,
}

impl Game {
    pub fn new() -> Self {
        let map = Self::cellular_automata();
        let structures = vec![common::StructureE {
            name: "nico".to_string(),
            struc_type: common::StructureTypeE::Castle,
            pos: (42, 110),
        }];
        let unit_groups = Vec::new();
        Self {
            map,
            structures,
            unit_groups,
        }
    }
    pub fn export_map(&self) -> Vec<Vec<common::TileE>> {
        self.map.clone()
    }
    pub fn export_objs(&self) -> common::MapObjsE {
        common::MapObjsE {
            structures: self.structures.clone(),
            unit_groups: self.unit_groups.clone(),
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
