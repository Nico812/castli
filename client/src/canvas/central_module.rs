use common::exports::game_object::GameObjE;
use common::exports::tile::TileE;
use common::{GameCoord, GameID};
use rand::SeedableRng;
use rand::{self, Rng, rngs};
use std::collections::HashMap;

use super::module_utility;
use crate::ansi::*;
use crate::assets::*;
use crate::canvas::r#const::{CENTRAL_MODULE_CONTENT_COLS, CENTRAL_MODULE_CONTENT_ROWS};
use crate::canvas::module_utility::WithArt;

use common::r#const::{self, MAP_COLS, MAP_ROWS};

pub struct CentralModule {
    // Stores the tiles for the rest of the game, since they should be immutable
    map_tiles: Vec<Vec<TileE>>,
    world_map_tiles: Vec<Vec<TileE>>,
    wind_map: Vec<Vec<bool>>,
    rng: rngs::SmallRng,
}

impl CentralModule {
    pub const CONTENT_ROWS: usize = CENTRAL_MODULE_CONTENT_ROWS;
    pub const CONTENT_COLS: usize = CENTRAL_MODULE_CONTENT_COLS;
    pub const WIND_ROWS: usize = MAP_ROWS / 2;
    pub const WIND_COLS: usize = MAP_COLS;
    const ZOOM_FACTOR: usize = 8;

    pub fn new() -> Self {
        let map_tiles = vec![vec![TileE::Grass; r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles = vec![
            vec![TileE::Grass; r#const::MAP_COLS / Self::ZOOM_FACTOR];
            r#const::MAP_ROWS / Self::ZOOM_FACTOR
        ];

        // Wind
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let mut wind_map = vec![vec![false; Self::WIND_COLS]; Self::WIND_ROWS];
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

    pub fn get_renderable_and_update(
        &mut self,
        game_objs: &HashMap<GameID, GameObjE>,
        map_zoom: Option<GameCoord>,
        render_count: u32,
    ) -> Vec<Vec<TermCell>> {
        let (tiles, zoom_coord, frame_title) = match map_zoom {
            Some(coord) => {
                let tiles = self.get_map_slice(coord);
                let title = format!("zoom (y:{}, x:{})", coord.y, coord.x);
                (tiles, Some(coord), title)
            }
            None => {
                let tiles = self.world_map_tiles.clone();
                let title = "world map".to_string();
                (tiles, None, title)
            }
        };

        let wind_pos = zoom_coord.unwrap_or(GameCoord { x: 0, y: 0 });
        let cut_wind = self.get_wind_slice(wind_pos);

        let mut cells = Self::tiles_to_cells(&tiles, &cut_wind);

        match zoom_coord {
            Some(coord) => Self::add_objs_to_cells(&mut cells, game_objs, coord),
            None => Self::add_world_objs_to_cells(&mut cells, game_objs),
        }

        self.update_wind(render_count, wind_pos);
        module_utility::add_frame(&frame_title, &mut cells);

        cells
    }

    fn update_wind(&mut self, render_count: u32, zoom_coord: GameCoord) {
        if render_count % 10 != 0 {
            return;
        }

        let mut tmp_wind = self.wind_map.clone();
        let row_start = zoom_coord.y / 2;
        let row_end = (row_start + Self::CONTENT_ROWS - 1).min(Self::WIND_ROWS);
        let col_start = zoom_coord.x;
        let col_end = (col_start + Self::CONTENT_COLS - 1).min(Self::WIND_COLS);

        for row in row_start..=row_end {
            for col in col_start..=col_end {
                if self.wind_map[row][col] {
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
        self.wind_map = tmp_wind;
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

                        for row in top_left_row..bottom_right_row {
                            for col in top_left_col..bottom_right_col {
                                match tiles[row][col] {
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

    fn tiles_to_cells<'a>(tiles: &Vec<Vec<TileE>>, wind: &Vec<Vec<bool>>) -> Vec<Vec<TermCell>> {
        tiles
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(cells_row, tiles_row)| {
                tiles_row
                    .iter()
                    .enumerate()
                    .map(|(cells_col, &tile_top)| {
                        let Some(tile_bot) = tiles
                            .get(cells_row * 2 + 1)
                            .map(|next_row| next_row[cells_col])
                        else {
                            return ERR_EL;
                        };

                        let top_tile_asset = TileAsset::get_asset(tile_top);
                        let bot_tile_asset = TileAsset::get_asset(tile_bot);
                        if tile_top == tile_bot {
                            match wind[cells_row][cells_col] {
                                true => top_tile_asset.wind,
                                false => top_tile_asset.std,
                            }
                        } else {
                            TermCell::new(BLOCK, top_tile_asset.fg, bot_tile_asset.bg)
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn add_objs_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<GameID, GameObjE>,
        zoom_coord: GameCoord,
    ) {
        for obj in objs.values() {
            if !Self::is_in_view(obj.get_pos(), zoom_coord, obj.get_art_size(false)) {
                continue;
            };
            let pos = obj.get_pos();
            let rel_pos_in_quad: (isize, isize) = (
                ((pos.y as isize - zoom_coord.y as isize) / 2),
                (pos.x as isize - zoom_coord.x as isize),
            );
            let art = obj.get_art(false);
            Self::add_art_to_cells(cells, art, rel_pos_in_quad);
        }
    }

    fn add_world_objs_to_cells(
        cells: &mut Vec<Vec<TermCell>>,
        world_objs: &HashMap<GameID, GameObjE>,
    ) {
        for obj in world_objs.values() {
            let pos = obj.get_pos();
            let rel_pos_in_quad: (isize, isize) = (
                (pos.y / (Self::ZOOM_FACTOR * 2)) as isize,
                (pos.x / Self::ZOOM_FACTOR) as isize,
            );
            let art = obj.get_art(true);
            Self::add_art_to_cells(cells, art, rel_pos_in_quad);
        }
    }

    // This can take negative positions to account for the objects that have origin outsize of view but
    // with art that enters the view
    fn add_art_to_cells(cells: &mut Vec<Vec<TermCell>>, art: &[&[TermCell]], pos: (isize, isize)) {
        for (art_row, art_row_iter) in art.iter().enumerate() {
            for (art_col, art_cell) in art_row_iter.iter().enumerate() {
                let cell_pos_y = pos.0 + art_row as isize;
                let cell_pos_x = pos.1 + art_col as isize;
                if cell_pos_y >= 0
                    && cell_pos_x >= 0
                    && cell_pos_y < cells.len() as isize
                    && cell_pos_x < cells[0].len() as isize
                {
                    cells[cell_pos_y as usize][cell_pos_x as usize] = *art_cell;
                }
            }
        }
    }

    fn get_map_slice(&self, zoom_coord: GameCoord) -> Vec<Vec<TileE>> {
        self.map_tiles[zoom_coord.y..(zoom_coord.y + Self::CONTENT_ROWS * 2).min(MAP_ROWS)]
            .iter()
            .map(|row| {
                row[zoom_coord.x..(zoom_coord.x + Self::CONTENT_COLS).min(MAP_COLS)].to_vec()
            })
            .collect()
    }

    fn get_wind_slice(&self, zoom_coord: GameCoord) -> Vec<Vec<bool>> {
        self.wind_map
            [zoom_coord.y / 2..(zoom_coord.y / 2 + Self::CONTENT_ROWS).min(Self::WIND_ROWS)]
            .iter()
            .map(|row| {
                row[zoom_coord.x..(zoom_coord.x + Self::CONTENT_COLS).min(Self::WIND_COLS)].to_vec()
            })
            .collect()
    }

    fn is_in_view(pos: GameCoord, zoom_coord: GameCoord, obj_size: (usize, usize)) -> bool {
        let y = pos.y + obj_size.0 >= zoom_coord.y && pos.y < zoom_coord.y + Self::CONTENT_ROWS * 2;
        let x = pos.x + obj_size.1 >= zoom_coord.x && pos.x < zoom_coord.x + Self::CONTENT_COLS;
        y && x
    }
}
