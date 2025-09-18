use std::collections::VecDeque;

use rand::Rng;

use crate::{
    r#const::{CA_ITER, PERCENT_ARE_WALLS},
    game::pathfinding,
};
use common::{
    TileE,
    r#const::{MAP_COLS, MAP_ROWS},
};

pub struct Map {
    pub tiles: Vec<Vec<TileE>>,
    pub obstacles: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = Self::cellular_automata();

        let mut obstacles = vec![vec![false; MAP_COLS]; MAP_ROWS];
        for (r, row) in tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == TileE::Water {
                    obstacles[r][c] = true;
                }
            }
        }

        Self { tiles, obstacles }
    }

    pub fn export(&self) -> Vec<Vec<TileE>> {
        self.tiles.clone()
    }

    pub fn find_path(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<VecDeque<(usize, usize)>> {
        pathfinding::bds::<MAP_ROWS, MAP_COLS>(start, end, &self.obstacles)
    }

    fn cellular_automata() -> Vec<Vec<TileE>> {
        let mut tiles = vec![vec![TileE::Grass; MAP_COLS]; MAP_ROWS];
        Self::fill_random(&mut tiles);
        let mut temp_tiles = tiles.clone();

        for _ in 0..CA_ITER {
            Self::step_life(&tiles, &mut temp_tiles);
            tiles.clone_from(&temp_tiles);
        }
        tiles
    }

    fn fill_random(tiles: &mut Vec<Vec<TileE>>) {
        let mut rng = rand::rng();

        for row in 0..MAP_ROWS {
            for col in 0..MAP_COLS {
                if row == 0
                    || col == 0
                    || row == MAP_ROWS - 1
                    || col == MAP_COLS - 1
                    || rng.random_range(1..=100) <= PERCENT_ARE_WALLS
                {
                    tiles[row][col] = TileE::Water;
                }
            }
        }
    }

    fn step_life(tiles: &Vec<Vec<TileE>>, temp_tiles: &mut Vec<Vec<TileE>>) {
        for row in 0..MAP_ROWS {
            for col in 0..MAP_COLS {
                let mut wall_count = 0;

                for i in row.saturating_sub(1)..=(row + 1).min(MAP_ROWS - 1) {
                    for j in col.saturating_sub(1)..=(col + 1).min(MAP_COLS - 1) {
                        if i == row && j == col {
                            continue;
                        }
                        if tiles[i][j] == TileE::Water {
                            wall_count += 1;
                        }
                    }
                }

                if wall_count >= 4 && temp_tiles[row][col] != TileE::Water {
                    temp_tiles[row][col] = TileE::Water;
                }
                if wall_count <= 2 && temp_tiles[row][col] == TileE::Water {
                    temp_tiles[row][col] = TileE::Grass;
                }
            }
        }
    }

    fn _print(&self) {
        for rows in self.tiles.iter() {
            for tile in rows {
                if *tile == TileE::Grass {
                    print!("G");
                } else {
                    print!("W");
                }
            }
            print!("\n");
        }
    }
}
