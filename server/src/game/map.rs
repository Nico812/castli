use rand::Rng;
use rayon::prelude::*;

use crate::r#const::{
    CA_ITER_HIGH_MOUNTAINS, CA_ITER_MOUNTAINS, CA_ITER_WATER, CA_ITER_WOODS,
    COUNTS_TO_SPREAD_HIGH_MOUNTAINS, COUNTS_TO_SPREAD_MOUNTAINS, COUNTS_TO_SPREAD_WATER,
    COUNTS_TO_SPREAD_WOODS, COUNTS_TO_SURVIVE_HIGH_MOUNTAINS, COUNTS_TO_SURVIVE_MOUNTAINS,
    COUNTS_TO_SURVIVE_WATER, COUNTS_TO_SURVIVE_WOODS, PERCENT_IS_HIGH_MOUNTAINS,
    PERCENT_IS_MOUNTAINS, PERCENT_IS_WATER, PERCENT_IS_WOODS,
};
use common::{
    GameCoord,
    r#const::{MAP_COLS, MAP_ROWS},
    map::Tile,
    packets::MapPayload,
};

struct TerrainParams {
    spreading: Tile,
    spreads_on: &'static [Tile],
    iters: usize,
    percent: u8,
    counts_to_spread: u8,
    counts_to_survive: u8,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    obstacles: Vec<Vec<bool>>,
    occupied: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = Self::cellular_automata();
        let obstacles: Vec<Vec<bool>> = tiles
            .par_iter()
            .map(|row| row.iter().map(|t| *t == Tile::Water).collect())
            .collect();
        let occupied = vec![vec![false; MAP_COLS]; MAP_ROWS];

        println!("mappa caricata LOL");
        Self {
            tiles,
            obstacles,
            occupied,
        }
    }

    pub fn is_obstacle(&self, pos: GameCoord) -> bool {
        self.obstacles
            .get(pos.y)
            .map(|row| row.get(pos.x))
            .flatten()
            .copied()
            .unwrap_or(true)
    }

    pub fn is_occupied(&self, pos: GameCoord) -> bool {
        self.occupied
            .get(pos.y)
            .map(|row| row.get(pos.x))
            .flatten()
            .copied()
            .unwrap_or(true)
    }

    pub fn can_build(&self, pos: GameCoord, size: GameCoord) -> bool {
        let Some(end_y) = pos.y.checked_add(size.y) else {
            return false;
        };
        let Some(end_x) = pos.x.checked_add(size.x) else {
            return false;
        };

        for row in pos.y..end_y {
            for col in pos.x..end_x {
                let curr_pos = GameCoord::new(row, col);
                if self.is_occupied(curr_pos) || self.is_obstacle(curr_pos) {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_occupied(&mut self, pos: GameCoord, size: GameCoord) {
        let end_y = pos.y.saturating_add(size.y);
        let end_x = pos.x.saturating_add(size.x);

        for row in pos.y..end_y {
            if row >= MAP_ROWS {
                continue;
            }
            for col in pos.x..end_x {
                if col >= MAP_COLS {
                    continue;
                }
                self.occupied[row][col] = true;
            }
        }
    }

    pub fn get_obstacles(&self) -> &Vec<Vec<bool>> {
        &self.obstacles
    }

    pub fn get_tile(&self, pos: GameCoord) -> Option<Tile> {
        self.tiles
            .get(pos.y)
            .map(|row| row.get(pos.x))
            .flatten()
            .copied()
    }

    fn cellular_automata() -> Vec<Vec<Tile>> {
        let mut a = vec![vec![Tile::Grass; MAP_COLS]; MAP_ROWS];
        let mut b = a.clone();

        let terrains = [
            TerrainParams {
                spreading: Tile::Water,
                spreads_on: &[Tile::Grass],
                iters: CA_ITER_WATER,
                percent: PERCENT_IS_WATER,
                counts_to_spread: COUNTS_TO_SPREAD_WATER,
                counts_to_survive: COUNTS_TO_SURVIVE_WATER,
            },
            TerrainParams {
                spreading: Tile::Woods,
                spreads_on: &[Tile::Grass],
                iters: CA_ITER_WOODS,
                percent: PERCENT_IS_WOODS,
                counts_to_spread: COUNTS_TO_SPREAD_WOODS,
                counts_to_survive: COUNTS_TO_SURVIVE_WOODS,
            },
            TerrainParams {
                spreading: Tile::Mountain,
                spreads_on: &[Tile::Grass, Tile::Woods],
                iters: CA_ITER_MOUNTAINS,
                percent: PERCENT_IS_MOUNTAINS,
                counts_to_spread: COUNTS_TO_SPREAD_MOUNTAINS,
                counts_to_survive: COUNTS_TO_SURVIVE_MOUNTAINS,
            },
            TerrainParams {
                spreading: Tile::HighMountain,
                spreads_on: &[Tile::Mountain],
                iters: CA_ITER_HIGH_MOUNTAINS,
                percent: PERCENT_IS_HIGH_MOUNTAINS,
                counts_to_spread: COUNTS_TO_SPREAD_HIGH_MOUNTAINS,
                counts_to_survive: COUNTS_TO_SURVIVE_HIGH_MOUNTAINS,
            },
        ];

        for params in &terrains {
            Self::run_terrain(&mut a, &mut b, params);
        }

        a
    }

    fn run_terrain(a: &mut Vec<Vec<Tile>>, b: &mut Vec<Vec<Tile>>, params: &TerrainParams) {
        let before_add_random = a.clone();
        Self::add_random(a, params.spreading, params.spreads_on, params.percent);
        b.clone_from(a);

        for _ in 0..params.iters {
            Self::step_life(a, b, &before_add_random, params);
            std::mem::swap(a, b);
        }
    }

    fn add_random(tiles: &mut [Vec<Tile>], add_type: Tile, add_on: &[Tile], percent: u8) {
        tiles
            .par_iter_mut()
            .enumerate()
            .for_each(|(row, tile_row)| {
                let mut rng = rand::rng();
                for (col, tile) in tile_row.iter_mut().enumerate() {
                    let is_edge =
                        row == 0 || col == 0 || row == MAP_ROWS - 1 || col == MAP_COLS - 1;
                    let random_hit = rng.random_range(1..=100) <= percent;
                    let is_valid_tile = add_on.contains(tile);

                    if (is_edge || random_hit) && is_valid_tile {
                        *tile = add_type;
                    }
                }
            });
    }

    fn step_life(
        a: &[Vec<Tile>],
        b: &mut [Vec<Tile>],
        before_add_random: &[Vec<Tile>],
        params: &TerrainParams,
    ) {
        let spreading = params.spreading;
        let spreads_on = params.spreads_on;
        let counts_to_spread = params.counts_to_spread;
        let counts_to_survive = params.counts_to_survive;

        b.par_iter_mut().enumerate().for_each(|(row, b_row)| {
            if row == 0 || row == MAP_ROWS - 1 {
                return;
            }
            let prev = &a[row - 1];
            let curr = &a[row];
            let next = &a[row + 1];
            let before_row = &before_add_random[row];
            for col in 1..MAP_COLS - 1 {
                let mut neightb_count = 0u8;
                for c in (col - 1)..=(col + 1) {
                    if prev[c] == spreading {
                        neightb_count += 1;
                    }
                    if next[c] == spreading {
                        neightb_count += 1;
                    }
                }
                if curr[col - 1] == spreading {
                    neightb_count += 1;
                }
                if curr[col + 1] == spreading {
                    neightb_count += 1;
                }

                let mut new_val = curr[col];
                if neightb_count >= counts_to_spread && spreads_on.contains(&new_val) {
                    new_val = spreading;
                }
                if neightb_count < counts_to_survive && new_val == spreading {
                    new_val = before_row[col];
                }
                b_row[col] = new_val;
            }
        });
    }

    pub fn export(&self) -> MapPayload {
        let rows = self.tiles.len();
        let cols = self.tiles.first().map(|row| row.len()).unwrap_or(0);
        let mut tiles = Vec::with_capacity(rows * cols);
        for row in &self.tiles {
            tiles.extend_from_slice(row);
        }
        MapPayload {
            rows: rows as u32,
            cols: cols as u32,
            tiles,
        }
    }
}
