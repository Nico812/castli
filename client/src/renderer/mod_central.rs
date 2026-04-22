use std::collections::HashMap;

use common::r#const::{MAP_COLS, MAP_ROWS};
use common::exports::game_object::GameObjE;
use common::exports::tile::TileE;
use common::{GameCoord, GameID};

use super::module_utility;
use crate::ansi::*;
use crate::assets::*;
use crate::client::GameState;
use crate::renderer::map_data::MapData;
use crate::renderer::module_utility::WithArt;
use crate::renderer::renderer::Renderer;
use crate::shared_state::UiState;

pub struct ModCentral {}

impl ModCentral {
    pub fn update(
        game_state: &GameState,
        ui_state: &UiState,
        map_data: &MapData,
    ) -> Vec<Vec<TermCell>> {
        let (tiles, zoom_coord, frame_title) = match ui_state.zoom {
            Some(coord) => {
                let tiles = Self::get_map_slice(&map_data.tiles, coord);
                let title = format!("Castli | zoom: {}", coord);
                (tiles, Some(coord), title)
            }
            None => {
                let tiles = map_data.tiles_wor.clone();
                let title = "Castli | world map".to_string();
                (tiles, None, title)
            }
        };

        let wind_pos = zoom_coord.unwrap_or(GameCoord { x: 0, y: 0 });
        let cut_wind = Self::get_wind_slice(&map_data.wind, wind_pos);

        let mut cells = Self::tiles_to_cells(&tiles, &cut_wind);

        match zoom_coord {
            Some(coord) => Self::add_objs_to_cells(&mut cells, &game_state.objs, coord),
            None => Self::add_world_objs_to_cells(&mut cells, &game_state.objs),
        }

        module_utility::add_frame(&frame_title, &mut cells);

        cells
    }

    fn tiles_to_cells(tiles: &[Vec<TileE>], wind: &[Vec<bool>]) -> Vec<Vec<TermCell>> {
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
                            return ERR.std;
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
                (pos.y / (Renderer::ZOOM_FACTOR * 2)) as isize,
                (pos.x / Renderer::ZOOM_FACTOR) as isize,
            );
            let art = obj.get_art(true);
            Self::add_art_to_cells(cells, art, rel_pos_in_quad);
        }
    }

    // This can take negative positions to account for the objects that have origin outsize of view but
    // with art that enters the view
    fn add_art_to_cells(cells: &mut [Vec<TermCell>], art: &[&[TermCell]], pos: (isize, isize)) {
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

    fn get_map_slice(tiles: &[Vec<TileE>], zoom: GameCoord) -> Vec<Vec<TileE>> {
        tiles[zoom.y..(zoom.y + Renderer::FOV_ROWS * 2).min(MAP_ROWS)]
            .iter()
            .map(|row| row[zoom.x..(zoom.x + Renderer::FOV_COLS).min(MAP_COLS)].to_vec())
            .collect()
    }

    fn get_wind_slice(wind: &[Vec<bool>], zoom: GameCoord) -> Vec<Vec<bool>> {
        wind[zoom.y / 2..(zoom.y / 2 + Renderer::FOV_ROWS).min(MapData::WIND_ROWS)]
            .iter()
            .map(|row| row[zoom.x..(zoom.x + Renderer::FOV_COLS).min(MapData::WIND_COLS)].to_vec())
            .collect()
    }

    fn is_in_view(pos: GameCoord, zoom: GameCoord, obj_size: (usize, usize)) -> bool {
        let y = pos.y + obj_size.0 >= zoom.y && pos.y < zoom.y + Renderer::FOV_ROWS * 2;
        let x = pos.x + obj_size.1 >= zoom.x && pos.x < zoom.x + Renderer::FOV_COLS;
        y && x
    }
}
