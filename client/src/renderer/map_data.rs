use common::{
    r#const::{MAP_COLS, MAP_ROWS},
    map::Tile,
};
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::renderer::{r#const::ZOOM_FACTOR, renderer::Renderer};

pub struct MapData {
    pub tiles_wor: Vec<Vec<Tile>>,
    pub variants: Vec<Vec<bool>>,
}

impl MapData {
    pub const WIND_ROWS: usize = MAP_ROWS / 2;
    pub const WIND_COLS: usize = MAP_COLS;

    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);

        let tiles_wor = (0..MAP_ROWS / ZOOM_FACTOR)
            .map(|world_map_row| {
                (0..MAP_COLS / ZOOM_FACTOR)
                    .map(|world_map_col| {
                        let top_left_row = world_map_row * ZOOM_FACTOR;
                        let top_left_col = world_map_col * ZOOM_FACTOR;
                        let bottom_right_row =
                            ((world_map_row + 1) * ZOOM_FACTOR).min(MAP_ROWS) - 1;
                        let bottom_right_col =
                            ((world_map_col + 1) * ZOOM_FACTOR).min(MAP_COLS) - 1;

                        let mut grass_count = 0;
                        let mut water_count = 0;

                        for row in tiles.iter().take(bottom_right_row + 1).skip(top_left_row) {
                            for tile in row.iter().take(bottom_right_col + 1).skip(top_left_col) {
                                match tile {
                                    Tile::Grass => grass_count += 1,
                                    Tile::Water => water_count += 1,
                                    _ => {}
                                }
                            }
                        }
                        if grass_count >= water_count {
                            Tile::Grass
                        } else {
                            Tile::Water
                        }
                    })
                    .collect()
            })
            .collect();

        let mut variants = vec![vec![false; Self::WIND_COLS]; Self::WIND_ROWS];
        for cell in variants.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self {
            tiles_wor,
            variants,
        }
    }
}
