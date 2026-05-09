use common::{config::config as common_config, map::Tile};
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::config::config as client_config;

pub struct MapData {
    pub tiles_wor: Vec<Vec<Tile>>,
    pub variants: Vec<Vec<bool>>,
}

impl MapData {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);
        let map_rows = common_config().world.map_rows;
        let map_cols = common_config().world.map_cols;
        let zoom = client_config().ui.zoom_factor;
        let wind_rows = map_rows / 2;
        let wind_cols = map_cols;

        let tiles_wor = (0..map_rows / zoom)
            .map(|world_map_row| {
                (0..map_cols / zoom)
                    .map(|world_map_col| {
                        let top_left_row = world_map_row * zoom;
                        let top_left_col = world_map_col * zoom;
                        let bottom_right_row = ((world_map_row + 1) * zoom).min(map_rows) - 1;
                        let bottom_right_col = ((world_map_col + 1) * zoom).min(map_cols) - 1;

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

        let mut variants = vec![vec![false; wind_cols]; wind_rows];
        for cell in variants.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self {
            tiles_wor,
            variants,
        }
    }
}
