//! # TUI Central Module
//!
//! Defines the `CentralModule`, which handles the central area of the Canvas.

use rand::SeedableRng;
use rand::{self, Rng, rngs};
use std::collections::HashMap;

use super::module_utility;
use crate::ansi::*;
use crate::assets::*;
use crate::r#const::{QUADRANT_COLS, QUADRANT_ROWS};
use common::r#const::{self, MAP_COLS, MAP_ROWS};

pub struct CentralModule {
    // Stores the tiles for the rest of the game, since they should be immutable
    map_tiles: Vec<Vec<common::TileE>>,
    world_map_tiles: Vec<Vec<common::TileE>>,
    wind_map: Vec<Vec<bool>>,
    rng: rngs::SmallRng,
}

impl CentralModule {
    pub const CONTENT_ROWS: usize = QUADRANT_ROWS;
    pub const CONTENT_COLS: usize = QUADRANT_COLS;
    const ZOOM_FACTOR: usize = 8;

    // PUB
    pub fn new() -> Self {
        let map_tiles = vec![vec![common::TileE::Grass; r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles = vec![
            vec![common::TileE::Grass; r#const::MAP_COLS / Self::ZOOM_FACTOR];
            r#const::MAP_ROWS / Self::ZOOM_FACTOR
        ];

        // Wind
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let mut wind_map = vec![vec![false; r#const::MAP_COLS]; r#const::MAP_ROWS / 2];
        for cell in wind_map.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self {
            map_tiles,
            world_map_tiles,
            wind_map,
            rng,
        }
    }

    pub fn init(&mut self, tiles: Vec<Vec<common::TileE>>) {
        self.set_tiles(tiles);
    }

    pub fn get_renderable_and_update(
        &mut self,
        game_objs: &HashMap<common::GameID, common::GameObjE>,
        map_zoom: Option<(usize, usize)>,
        render_count: u32,
    ) -> Vec<Vec<TermCell>> {
        let mut cells;

        match map_zoom {
            Some(quadrant) => {
                let cut_tiles = self.get_map_slice(quadrant);
                let cut_wind = self.get_wind_slice(quadrant);
                cells = Self::tiles_to_cells(&cut_tiles, &cut_wind);
                Self::add_objs_to_cells(&mut cells, game_objs, quadrant);
                self.update_wind(render_count, quadrant);
                module_utility::add_frame(&format!("({}, {})", quadrant.0, quadrant.1), &mut cells);
            }
            None => {
                let cut_wind = self.get_wind_slice((7, 7));
                cells = Self::tiles_to_cells(&self.world_map_tiles, &cut_wind);
                Self::add_world_objs_to_cells(&mut cells, game_objs);
                self.update_wind(render_count, (7, 7));
                module_utility::add_frame("world map", &mut cells);
            }
        }
        cells
    }

    // PRIVATE
    fn update_wind(&mut self, render_count: u32, quadrant: (usize, usize)) {
        if render_count % 10 != 0 {
            return;
        }
        let row_start = quadrant.0 * Self::CONTENT_ROWS;
        let col_start = quadrant.1 * Self::CONTENT_COLS;

        for row in row_start..row_start + Self::CONTENT_ROWS {
            for col in col_start..col_start + Self::CONTENT_COLS {
                let next_col = if col < col_start + Self::CONTENT_COLS - 1 {
                    col + 1
                } else {
                    col_start
                };
                let next_row = if row < row_start + Self::CONTENT_ROWS - 1 {
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
        self.world_map_tiles = (0..MAP_ROWS / Self::ZOOM_FACTOR)
            .map(|world_map_row| {
                (0..MAP_COLS / Self::ZOOM_FACTOR)
                    .map(|world_map_col| {
                        let top_left_row = world_map_row * Self::ZOOM_FACTOR;
                        let top_left_col = world_map_col * Self::ZOOM_FACTOR;
                        let bottom_right_row =
                            ((world_map_row + 1) * Self::ZOOM_FACTOR).min(MAP_ROWS);
                        let bottom_right_col =
                            ((world_map_col + 1) * Self::ZOOM_FACTOR).min(MAP_COLS);

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
        tiles
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(cells_row, tiles_row)| {
                tiles_row
                    .iter()
                    .enumerate()
                    .map(|(cells_col, &tile_top)| {
                        let tile_bottom = tiles[cells_row * 2 + 1][cells_col];

                        if tile_top == tile_bottom {
                            match tile_top {
                                common::TileE::Grass => {
                                    if wind[cells_row][cells_col] {
                                        GRASS_EL_2
                                    } else {
                                        GRASS_EL_1
                                    }
                                }
                                common::TileE::Water => {
                                    if wind[cells_row][cells_col] {
                                        WATER_EL_2
                                    } else {
                                        WATER_EL_1
                                    }
                                }
                                _ => ERR_EL,
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
                    })
                    .collect()
            })
            .collect()
    }

    fn add_objs_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<common::GameID, common::GameObjE>,
        quadrant: (usize, usize),
    ) {
        for obj in objs.values() {
            if !Self::is_in_quadrant_from_game_coord(obj.get_pos(), quadrant) {
                continue;
            };
            let pos = obj.get_pos();
            let pos_in_quadrant = ((pos.0 / 2) % Self::CONTENT_ROWS, pos.1 % Self::CONTENT_COLS);
            // TODO: simplify adding a function in common that gets you the right art for any obj
            match obj {
                common::GameObjE::Castle(_) => {
                    Self::add_art_to_cells(cells, &CASTLE_ART, pos_in_quadrant);
                }
                common::GameObjE::UnitGroup(_) => {
                    Self::add_art_to_cells(cells, &UNIT_GROUP_ART, pos_in_quadrant);
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
            let pos = obj.get_pos();
            let pos_in_world = (pos.0 / (Self::ZOOM_FACTOR * 2), pos.1 / Self::ZOOM_FACTOR);
            match obj {
                common::GameObjE::Castle(_) => {
                    Self::add_art_to_cells(cells, &CASTLE_ART_WORLD, pos_in_world);
                }
                common::GameObjE::UnitGroup(_) => {
                    Self::add_art_to_cells(cells, &UNIT_GROUP_ART, pos_in_world);
                }
                _ => {}
            }
        }
    }

    // refactor to take also 1 single cell or one dimensional arrays
    fn add_art_to_cells<const M: usize, const N: usize>(
        cells: &mut Vec<Vec<TermCell>>,
        art: &[[TermCell; N]; M],
        pos: (usize, usize),
    ) {
        for (art_row, art_row_iter) in art.iter().enumerate() {
            for (art_col, art_cell) in art_row_iter.iter().enumerate() {
                cells[pos.0 + art_row][pos.1 + art_col] = *art_cell;
            }
        }
    }

    // TODO: TAKE ONLY SLICES DONT CLONE
    fn get_map_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<common::TileE>> {
        self.map_tiles
            [quadrant.0 * Self::CONTENT_ROWS * 2..(quadrant.0 + 1) * Self::CONTENT_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * Self::CONTENT_COLS..(quadrant.1 + 1) * Self::CONTENT_COLS].to_vec()
            })
            .collect()
    }

    fn get_wind_slice(&self, quadrant: (usize, usize)) -> Vec<Vec<bool>> {
        self.wind_map[quadrant.0 * Self::CONTENT_ROWS..(quadrant.0 + 1) * Self::CONTENT_ROWS]
            .iter()
            .map(|row| {
                row[quadrant.1 * Self::CONTENT_COLS..(quadrant.1 + 1) * Self::CONTENT_COLS].to_vec()
            })
            .collect()
    }

    fn is_in_quadrant_from_game_coord(pos: (usize, usize), quadrant: (usize, usize)) -> bool {
        if pos.0 < quadrant.0 * Self::CONTENT_ROWS * 2
            || pos.0 >= (quadrant.0 + 1) * Self::CONTENT_ROWS * 2
        {
            return false;
        }
        if pos.1 < quadrant.1 * Self::CONTENT_COLS || pos.1 >= (quadrant.1 + 1) * Self::CONTENT_COLS
        {
            return false;
        }
        true
    }
}
