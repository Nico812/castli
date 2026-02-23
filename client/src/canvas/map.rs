use std::collections::HashMap;

use common::r#const::{MAP_COLS, MAP_ROWS};
use common::exports::game_object::GameObjE;
use common::exports::tile::TileE;
use common::{GameCoord, GameID};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::art::{self, WithArt};
use super::cell::{BLOCK, TermCell};
use super::frame;
use super::layout::{CENTRAL_MOD_POS, CENTRAL_MODULE_CONTENT_COLS, CENTRAL_MODULE_CONTENT_ROWS};

pub struct CentralModule {
    map_tiles: Vec<Vec<TileE>>,
    world_map_tiles: Vec<Vec<TileE>>,
    wind_map: Vec<Vec<bool>>,
    rng: SmallRng,
}

impl CentralModule {
    pub const CONTENT_ROWS: usize = CENTRAL_MODULE_CONTENT_ROWS;
    pub const CONTENT_COLS: usize = CENTRAL_MODULE_CONTENT_COLS;
    const ZOOM_FACTOR: usize = 8;

    pub fn new() -> Self {
        let map_tiles = vec![vec![TileE::Grass; MAP_COLS]; MAP_ROWS];
        let world_map_tiles =
            vec![vec![TileE::Grass; MAP_COLS / Self::ZOOM_FACTOR]; MAP_ROWS / Self::ZOOM_FACTOR];

        let mut rng = SmallRng::seed_from_u64(1);
        let mut wind_map = vec![vec![false; MAP_COLS]; MAP_ROWS / 2];
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

    pub fn init(&mut self, tiles: Vec<Vec<TileE>>) {
        self.set_tiles(tiles);
    }

    pub fn render_into(
        &mut self,
        frame: &mut [Vec<TermCell>],
        game_objs: &HashMap<GameID, GameObjE>,
        map_zoom: Option<GameCoord>,
        render_count: u32,
    ) {
        let content_top = CENTRAL_MOD_POS.0 + 1;
        let content_left = CENTRAL_MOD_POS.1 + 1;

        match map_zoom {
            Some(zoom_coord) => {
                self.render_zoomed_tiles(frame, content_top, content_left, zoom_coord);
                Self::render_objs(frame, content_top, content_left, game_objs, zoom_coord);
                self.update_wind(render_count, zoom_coord);
                frame::render_frame_into(
                    frame,
                    &format!("zoom (y:{}, x:{})", zoom_coord.y, zoom_coord.x),
                    CENTRAL_MOD_POS,
                    Self::CONTENT_ROWS,
                    Self::CONTENT_COLS,
                );
            }
            None => {
                let wind_origin = GameCoord { x: 0, y: 0 };
                self.render_world_tiles(frame, content_top, content_left);
                Self::render_world_objs(frame, content_top, content_left, game_objs);
                self.update_wind(render_count, wind_origin);
                frame::render_frame_into(
                    frame,
                    "world map",
                    CENTRAL_MOD_POS,
                    Self::CONTENT_ROWS,
                    Self::CONTENT_COLS,
                );
            }
        }
    }

    fn render_zoomed_tiles(
        &self,
        frame: &mut [Vec<TermCell>],
        top: usize,
        left: usize,
        zoom: GameCoord,
    ) {
        let wind_row_start = zoom.y / 2;
        for cr in 0..Self::CONTENT_ROWS {
            let tile_row = zoom.y + cr * 2;
            for cc in 0..Self::CONTENT_COLS {
                let tile_col = zoom.x + cc;
                frame[top + cr][left + cc] = tile_pair_to_cell(
                    self.map_tiles[tile_row][tile_col],
                    self.map_tiles[tile_row + 1][tile_col],
                    self.wind_map[wind_row_start + cr][tile_col],
                );
            }
        }
    }

    fn render_world_tiles(&self, frame: &mut [Vec<TermCell>], top: usize, left: usize) {
        let tiles = &self.world_map_tiles;
        let max_cell_rows = tiles.len() / 2;
        let max_cell_cols = if tiles.is_empty() { 0 } else { tiles[0].len() };

        for cr in 0..Self::CONTENT_ROWS.min(max_cell_rows) {
            let tile_row = cr * 2;
            for cc in 0..Self::CONTENT_COLS.min(max_cell_cols) {
                frame[top + cr][left + cc] = tile_pair_to_cell(
                    tiles[tile_row][cc],
                    tiles[tile_row + 1][cc],
                    self.wind_map[cr][cc],
                );
            }
        }
    }

    fn render_objs(
        frame: &mut [Vec<TermCell>],
        top: usize,
        left: usize,
        objs: &HashMap<GameID, GameObjE>,
        zoom_coord: GameCoord,
    ) {
        for obj in objs.values() {
            if !Self::is_in_view(obj.get_pos(), zoom_coord, obj.get_art_size(false)) {
                continue;
            }
            let pos = obj.get_pos();
            let art = obj.get_art(false);
            render_art(
                frame,
                art,
                top as isize + (pos.y as isize - zoom_coord.y as isize) / 2,
                left as isize + pos.x as isize - zoom_coord.x as isize,
            );
        }
    }

    fn render_world_objs(
        frame: &mut [Vec<TermCell>],
        top: usize,
        left: usize,
        world_objs: &HashMap<GameID, GameObjE>,
    ) {
        for obj in world_objs.values() {
            let pos = obj.get_pos();
            let art = obj.get_art(true);
            render_art(
                frame,
                art,
                top as isize + (pos.y / (Self::ZOOM_FACTOR * 2)) as isize,
                left as isize + (pos.x / Self::ZOOM_FACTOR) as isize,
            );
        }
    }

    fn update_wind(&mut self, render_count: u32, zoom_coord: GameCoord) {
        if !render_count.is_multiple_of(10) {
            return;
        }
        let row_start = zoom_coord.y / 2;
        let col_start = zoom_coord.x;

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

    fn set_tiles(&mut self, tiles: Vec<Vec<TileE>>) {
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

                        for tile_row in &tiles[top_left_row..bottom_right_row] {
                            for tile in &tile_row[top_left_col..bottom_right_col] {
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

        self.map_tiles = tiles;
    }

    fn is_in_view(pos: GameCoord, zoom_coord: GameCoord, obj_size: (usize, usize)) -> bool {
        let y = pos.y + obj_size.0 >= zoom_coord.y && pos.y < zoom_coord.y + Self::CONTENT_ROWS * 2;
        let x = pos.x + obj_size.1 >= zoom_coord.x && pos.x < zoom_coord.x + Self::CONTENT_COLS;
        y && x
    }
}

fn tile_pair_to_cell(top: TileE, bottom: TileE, wind: bool) -> TermCell {
    if top == bottom {
        match top {
            TileE::Grass => {
                if wind {
                    art::GRASS_EL_2
                } else {
                    art::GRASS_EL_1
                }
            }
            TileE::Water => {
                if wind {
                    art::WATER_EL_2
                } else {
                    art::WATER_EL_1
                }
            }
            _ => art::ERR_EL,
        }
    } else {
        let fg = match top {
            TileE::Grass => art::GRASS_FG,
            TileE::Water => art::WATER_FG,
            _ => art::ERR_FG,
        };
        let bg = match bottom {
            TileE::Grass => art::GRASS_BG,
            TileE::Water => art::WATER_BG,
            _ => art::ERR_BG,
        };
        TermCell::new(BLOCK, fg, bg)
    }
}

fn render_art(frame: &mut [Vec<TermCell>], art: &[&[TermCell]], row: isize, col: isize) {
    let frame_rows = frame.len() as isize;
    let frame_cols = frame[0].len() as isize;
    for (ar, art_row) in art.iter().enumerate() {
        for (ac, &cell) in art_row.iter().enumerate() {
            let r = row + ar as isize;
            let c = col + ac as isize;
            if r >= 0 && c >= 0 && r < frame_rows && c < frame_cols {
                frame[r as usize][c as usize] = cell;
            }
        }
    }
}
