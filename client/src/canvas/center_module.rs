//! # TUI Central Module
//!
//! Defines the `CentralModule`, which handles the central area of the Canvas.


use rand::SeedableRng;
use rand::{self, Rng, rngs};
use std::collections::HashMap;

use crate::ansi::*;
use crate::assets::*;
use crate::canvas::r#const::*;
use common::r#const::{self, MAP_COLS, MAP_ROWS};

pub struct CentralModule {
    // Stores the tiles for the rest of the game, since they should be immutable
    map_tiles: Vec<Vec<common::TileE>>,
    world_map_tiles: Vec<Vec<common::TileE>>,
    wind_map: Vec<Vec<bool>>,
    rng: rngs::SmallRng,
}

impl CentralModule {
    const CONTENT_ROWS: usize = 32;
    const CONTENT_COLS: usize = 64;
    const ZOOM_FACTOR: usize = 8;

    // PUB
    pub fn new() -> Self {
        let map_tiles = vec![vec![common::TileE::Grass; r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles =
            vec![vec![common::TileE::Grass; r#const::MAP_COLS / ZOOM_FACTOR]; r#const::MAP_ROWS / ZOOM_FACTOR];

        // Wind
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let mut wind_map = vec![vec![false; r#const::MAP_COLS]; r#const::MAP_ROWS/2];
        for cell in wind_map.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.gen_bool(0.1);
        }

        Self {
            map_tiles,
            world_map_tiles,
            wind_map,
            rng,
        }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.set_tiles(tiles);
    }

pub fn to_renderable(
    &mut self,
    game_objs: &HashMap<common::GameID, common::GameObjE>,
    map_zoom: Option<(usize, usize)>,
    render_count: u32,
) -> Vec<Vec<TermCell>> {
    match map_zoom {
        Some(quadrant) => {
            let cut_tiles = self.get_map_slice(quadrant);
            let cut_wind = self.get_wind_slice(quadrant);
            let mut cells = Self::tiles_to_cells(&cut_tiles, &cut_wind);
            Self::apply_objects_to_cells(&mut cells, game_objs, quadrant);
            self.update_wind(render_count, quadrant);
            cells
        }
        None => {
            let cut_wind = self.get_wind_slice((7, 7));
            let mut cells = Self::tiles_to_cells(&self.world_map_tiles, &cut_wind);
            Self::apply_objects_to_world_cells(&mut cells, game_objs);
            self.update_wind(render_count, (7,7));
            cells
        }
    }
}

    // PRIVATE
fn set_tiles(&mut self, tiles: Vec<Vec<common::TileE>>) {
    self.world_map_tiles = (0..MAP_ROWS / ZOOM_FACTOR)
        .map(|world_map_row| {
            (0..MAP_COLS / ZOOM_FACTOR)
                .map(|world_map_col| {
                    let top_left_row = world_map_row * ZOOM_FACTOR;
                    let top_left_col = world_map_col * ZOOM_FACTOR;
                    let bottom_right_row = ((world_map_row + 1) * ZOOM_FACTOR).min(MAP_ROWS);
                    let bottom_right_col = ((world_map_col + 1) * ZOOM_FACTOR).min(MAP_COLS);

                    let mut grass_count = 0;
                    let mut water_count = 0;

                    for row in top_left_row..bottom_right_row {
                        for col in top_left_col..bottom_right_col {
                            match tiles[row][col] {
                                common::TileE::Grass => grass_count += 1,
                                common::TileE::Water => water_count += 1,
                                _ => {}
                            }
                        }
                    }

                    if grass_count >= water_count {
                        common::TileE::Grass
                    } else {
                        common::TileE::Water
                    }
                })
                .collect()
        })
        .collect();

    self.map_tiles = tiles;
}

    fn tiles_to_cells<'a>(
        tiles: &Vec<Vec<common::TileE>>,
        wind: &Vec<Vec<bool>>,
    ) -> Vec<Vec<TermCell>> {
        tiles.iter().step_by(2).enumerate().map(|(cells_row, tiles_row)|{
            tiles_row.iter().enumerate().map(|(cells_col, &tile_top)|{
                let tile_bottom = tiles[cells_row*2 + 1][cells_col];
                
                if (tile_top == tile_bottom) {
                    match tile_top {
                        common::TileE::Grass => {
                            if wind_map[cells_row][cells_col] {
                                GRASS_EL_2
                            } else {
                                GRASS_EL_1
                            }
                        }
                        common::TileE::Water => {
                            if wind_map[cells_row][cells_col] {
                                WATER_EL_2
                            } else {
                                WATER_EL_1
                            }
                        }
                        _ => {
                            ERR_EL
                        }
                    }
                } else {
                    let fg = match tile_top {
                            common::TileE::Grass => GRASS_FG,
                            common::TileE::Water => WATER_FG,
                            _ => ERR_FG,
                        };
                    let bg = match tile_bottom {
                            common::TileE::Grass => GRASS_BG,
                            common::TileE::Water => WATER_BG,
                            _ => ERR_BG,
                        };
                    TermCell::new(BLOCK, fg, bg)
                }
            }).collect()
        }).collect()
    }

    fn apply_objects_to_cells(
        world_map: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<common::GameID, common::GameObjE>,
    ) {
        for obj in objs.values() {
            match obj {
                common::GameObjE::PlayerCastle(castle) => {
                    let term_pos = (castle.pos.0 / 16, castle.pos.1 / 8);
                    for (row, cells_row) in CASTLE_ART_WORLD.iter().enumerate() {
                        for (col, cell) in cells_row.iter().enumerate() {
                            world_map[term_pos.0 + row][term_pos.1 + col] = cell.clone();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn get_map_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<common::TileE>> {
        self.map_tiles
            [quadrant.0 * CENTRAL_MODULE_ROWS * 2..(quadrant.0 + 1) * CENTRAL_MODULE_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * CENTRAL_MODULE_COLS..(quadrant.1 + 1) * CENTRAL_MODULE_COLS]
                    .to_vec()
            })
            .collect()
    }

    fn get_wind_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<bool>> {
        self.wind_map
            [quadrant.0 * CENTRAL_MODULE_ROWS * 2..(quadrant.0 + 1) * CENTRAL_MODULE_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * CENTRAL_MODULE_COLS..(quadrant.1 + 1) * CENTRAL_MODULE_COLS]
                    .to_vec()
            })
            .collect()
    }

    fn add_objs_to_map(
        map: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<common::GameID, common::GameObjE>,
        quadrant: (usize, usize),
    ) {
        for obj in objs.iter() {
            match obj {
                (_, common::GameObjE::PlayerCastle(castle)) => {
                    let str_term_pos = (castle.pos.0 / 2, castle.pos.1);
                    if str_term_pos.0 < (quadrant.0 + 1) * CENTRAL_MODULE_ROWS
                        && str_term_pos.0 >= (quadrant.0 * CENTRAL_MODULE_ROWS)
                    {
                        if str_term_pos.1 < (quadrant.1 + 1) * CENTRAL_MODULE_COLS
                            && str_term_pos.1 >= (quadrant.1 * CENTRAL_MODULE_COLS)
                        {
                            for ansi_art_row in 0..r#const::CASTLE_SIZE / 2 {
                                let output_row =
                                    str_term_pos.0 % CENTRAL_MODULE_ROWS + ansi_art_row;
                                for ansi_art_col in 0..r#const::CASTLE_SIZE {
                                    let output_col =
                                        str_term_pos.1 % CENTRAL_MODULE_COLS + ansi_art_col;
                                    map[output_row][output_col] =
                                        CASTLE_ART[ansi_art_row][ansi_art_col];
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update_wind(&mut self, render_count: u32, quadrant: (usize, usize)) {
        const CENTRAL_MODULE_CONTENT_ROWS: usize = common::r#const::MAP_ROWS / 8;
        const CENTRAL_MODULE_CONTENT_COLS: usize = common::r#const::MAP_COLS / 8;

        if render_count % 10 != 0 {
            return;
        }
        let row_start = quadrant.0 * CENTRAL_MODULE_CONTENT_ROWS;
        let row_end = (quadrant.0 + 1) * CENTRAL_MODULE_CONTENT_ROWS;
        let col_start = quadrant.1 * CENTRAL_MODULE_CONTENT_COLS;
        let col_end = (quadrant.1 + 1) * CENTRAL_MODULE_CONTENT_COLS;

        for row in row_start..row_end {
            for col in col_start..col_end {
                let next_col = if col < col_end - 1 {
                    col + 1
                } else {
                    col_start
                };
                let next_row = if row < row_end - 1 {
                    row + 1
                } else {
                    row_start
                };
                if self.rng.random_bool(0.05)
                    && !self.wind_map[row][col]
                    && self.wind_map[row][next_col]
                {
                    self.wind_map[row][col] = true;
                    self.wind_map[row][next_col] = false;
                } else if self.rng.random_bool(0.01)
                    && !self.wind_map[row][col]
                    && self.wind_map[next_row][col]
                {
                    self.wind_map[row][col] = true;
                    self.wind_map[next_row][col] = false;
                }
            }
        }
    }
}