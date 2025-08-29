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

  fn update_wind(&mut self, render_count: u32, quadrant: (usize, usize)) {
        if render_count % 10 != 0 {
            return;
        }
        let row_start = quadrant.0 * CONTENT_ROWS;
        let col_start = quadrant.1 * CONTENT_COLS;

        for row in row_start..row_start + CONTENT_ROWS {
            for col in col_start..col_start + CONTENT_COLS {
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

    fn add_objs_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<common::GameID, common::GameObjE>,
        quadrant: (usize, usize),
    ) {
        for obj in objs.values() {
            match obj {
                common::GameObjE::PlayerCastle(castle) => {
                    let pos_in_quadrant = (castle.pos.0 % CONTENT_ROWS, castle.pos.1%CONTENT_COLS);
                    add_art_to_cells(cells, CASTLE_ART, pos_in_quadrant);
                }
                _ => {}
            }
        }
    }

    fn add_world_objs_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        world_objs: &HashMap<common::GameID, common::GameObjE>,
    ) {
        for obj in world_objs.values() {
            match obj {
                common::GameObjE::PlayerCastle(castle) => {
                    let pos_in_world = (castle.pos.0 / (ZOOM_FACTOR*2), castle.pos.1/ZOOM_FACTOR);
                    add_art_to_cells(cells, CASTLE_ART_WORLD, pos_in_world);
                }
                _ => {}
            }
        }
    }

    // refactor to take also 1 single cell or one dimensional arrays
    fn add_art_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        art: &Vec<Vec<TermCell>>,
        pos: (usize, usize),
    ) {
                    for (art_row, art_row_iter) in art.iter().enumerate() {
                        for (art_col, art_cell) in art_row.iter().enumerate() {
                            cells[pos.0 + art_row][pos.1 + art_col] = art_cell;
                        }
                    }
    }

    // TODO: TAKE ONLY SLICES DONT CLONE
    fn get_map_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<common::TileE>> {
        self.map_tiles
            [quadrant.0 * CONTENT_ROWS * 2..(quadrant.0 + 1) * CONTENT_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * CONTENT_COLS..(quadrant.1 + 1) * CONTENT_COLS]
                    .to_vec()
            })
            .collect()
    }

    fn get_wind_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<bool>> {
        self.wind_map
            [quadrant.0 * CONTENT_ROWS * 2..(quadrant.0 + 1) * CONTENT_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * CONTENT_COLS..(quadrant.1 + 1) * CONTENT_COLS]
                    .to_vec()
            })
            .collect()
    }
}