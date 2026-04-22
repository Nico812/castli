use common::{
    GameCoord,
    r#const::{MAP_COLS, MAP_ROWS},
    exports::tile::TileE,
};
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::renderer::renderer::Renderer;

pub struct MapData {
    pub tiles: Vec<Vec<TileE>>,
    pub tiles_wor: Vec<Vec<TileE>>,
    pub wind: Vec<Vec<bool>>,
    rng: SmallRng,
}

impl MapData {
    pub const WIND_ROWS: usize = MAP_ROWS / 2;
    pub const WIND_COLS: usize = MAP_COLS;

    pub fn new(tiles: Vec<Vec<TileE>>) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);

        let tiles_wor = (0..MAP_ROWS / Renderer::ZOOM_FACTOR)
            .map(|world_map_row| {
                (0..MAP_COLS / Renderer::ZOOM_FACTOR)
                    .map(|world_map_col| {
                        let top_left_row = world_map_row * Renderer::ZOOM_FACTOR;
                        let top_left_col = world_map_col * Renderer::ZOOM_FACTOR;
                        let bottom_right_row =
                            ((world_map_row + 1) * Renderer::ZOOM_FACTOR).min(MAP_ROWS) - 1;
                        let bottom_right_col =
                            ((world_map_col + 1) * Renderer::ZOOM_FACTOR).min(MAP_COLS) - 1;

                        let mut grass_count = 0;
                        let mut water_count = 0;

                        for row in tiles.iter().take(bottom_right_row + 1).skip(top_left_row) {
                            for tile in row.iter().take(bottom_right_col + 1).skip(top_left_col) {
                                match tile {
                                    TileE::Grass => grass_count += 1,
                                    TileE::Water => water_count += 1,
                                    _ => {}
                                }
                            }
                        }
                        if grass_count >= water_count {
                            TileE::Grass
                        } else {
                            TileE::Water
                        }
                    })
                    .collect()
            })
            .collect();

        let mut wind = vec![vec![false; Self::WIND_COLS]; Self::WIND_ROWS];
        for cell in wind.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self {
            tiles,
            tiles_wor,
            wind,
            rng,
        }
    }

    pub fn update_wind(&mut self, render_count: u32, zoom_coord: Option<GameCoord>) {
        if !render_count.is_multiple_of(10) {
            return;
        }

        let mut tmp_wind = self.wind.clone();

        let (row_start, col_start) = zoom_coord
            .map(|coord| (coord.y / 2, coord.x))
            .unwrap_or((0, 0));
        let row_end = (row_start + Renderer::FOV_ROWS - 1).min(Self::WIND_ROWS);
        let col_end = (col_start + Renderer::FOV_COLS - 1).min(Self::WIND_COLS);

        for row in row_start..=row_end {
            for col in col_start..=col_end {
                if self.wind[row][col] {
                    let mut next_col = col;
                    let mut next_row = row;
                    if row == row_end {
                        next_row = row_start;
                    } else {
                        next_row += 1;
                    }
                    if col == col_end {
                        next_col = col_start;
                    } else {
                        next_col += 1;
                    }

                    if self.rng.random_bool(0.3) && !tmp_wind[row][next_col] {
                        tmp_wind[row][col] = false;
                        tmp_wind[row][next_col] = true;
                    } else if self.rng.random_bool(0.1) && !tmp_wind[next_row][col] {
                        tmp_wind[row][col] = false;
                        tmp_wind[next_row][col] = true;
                    }
                }
            }
        }
        self.wind = tmp_wind;
    }

    pub fn get_tile(&self, coord: GameCoord) -> TileE {
        self.tiles
            .get(coord.y)
            .and_then(|row| row.get(coord.x))
            .copied()
            .unwrap_or(TileE::Err)
    }
}
