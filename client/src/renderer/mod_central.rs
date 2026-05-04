use std::collections::HashMap;

use common::r#const::{COURTYARD_COLS, COURTYARD_ROWS};
use common::courtyard::{Facility, FacilityType};
use common::game_objs::GameObjE;
use common::map::Tile;
use common::{GameCoord, GameId};

use super::module_utility;
use crate::assets::*;
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::map_data::MapData;
use crate::renderer::module_utility::WithArt;
use crate::renderer::renderer::Renderer;
use crate::ui_state::{Camera, CameraLocation, UiState};

pub struct ModCentral;

impl ModCentral {
    pub fn update(
        game_state: &mut GameState,
        ui_state: &UiState,
        map_data: &MapData,
    ) -> Vec<Vec<TermCell>> {
        let camera_coord = ui_state.camera.get_pos();

        let (mut cells, frame_title) = match ui_state.camera.location {
            CameraLocation::Map => {
                let title = format!("Castli | map {}", camera_coord);
                let mut cells = Self::draw_map(
                    &map_data.tiles,
                    &map_data.wind,
                    game_state.time.night,
                    &ui_state.camera,
                );
                Self::add_objs_to_map(
                    &game_state.player.castle_id,
                    &mut cells,
                    &game_state.objs,
                    &ui_state.camera,
                );
                (cells, title)
            }
            CameraLocation::WorldMap => {
                let title = "Castli | world map".to_string();
                let mut cells = Self::draw_map(
                    &map_data.tiles,
                    &map_data.wind,
                    game_state.time.night,
                    &ui_state.camera,
                );

                Self::add_objs_to_world_map(
                    &game_state.player.castle_id,
                    &mut cells,
                    &game_state.objs,
                    &ui_state.camera,
                );
                (cells, title)
            }
            CameraLocation::Courtyard => {
                let title = format!("Castli | courtyard {}", camera_coord);
                let mut cells = Self::draw_courtyard();
                Self::add_facilities_to_courtyard(
                    &mut cells,
                    &game_state.facilities,
                    &ui_state.camera,
                    game_state.time.night,
                );
                (cells, title)
            }
        };

        module_utility::add_frame(&frame_title, &mut cells);
        cells
    }

    fn draw_map(
        tiles: &[Vec<Tile>],
        wind: &[Vec<bool>],
        night: bool,
        camera: &Camera,
    ) -> Vec<Vec<TermCell>> {
        let mut cells = vec![vec![TermCell::ERR; Renderer::FOV_COLS]; Renderer::FOV_ROWS];

        for (row, cell_row) in cells.iter_mut().enumerate() {
            for (col, cell) in cell_row.iter_mut().enumerate() {
                let term_coord = TermCoord::new(row, col);
                let game_coord = match term_coord.to_game_coord(camera, true) {
                    Some(coord) => coord,
                    None => continue,
                };

                let tile_top = tiles
                    .get(game_coord.y)
                    .and_then(|tile_row| tile_row.get(game_coord.x));
                let tile_bot = tiles
                    .get(game_coord.y + 1)
                    .and_then(|tile_row| tile_row.get(game_coord.x));

                if let (Some(tile_top_), Some(tile_bot_)) = (tile_top, tile_bot) {
                    let top_tile_asset = TileAsset::get_asset(*tile_top_, night);
                    let bot_tile_asset = TileAsset::get_asset(*tile_bot_, night);

                    if tile_top_ == tile_bot_ {
                        let has_wind = wind
                            .get(game_coord.y)
                            .and_then(|wind_row| wind_row.get(game_coord.x))
                            .copied()
                            .unwrap_or(false);

                        *cell = if has_wind {
                            top_tile_asset.wind
                        } else {
                            top_tile_asset.std
                        };
                    } else {
                        *cell = TermCell::new(BLOCK, top_tile_asset.fg, bot_tile_asset.bg);
                    }
                }
            }
        }

        cells
    }

    fn draw_courtyard() -> Vec<Vec<TermCell>> {
        let mut cells = vec![vec![TermCell::ERR; Renderer::FOV_COLS]; Renderer::FOV_ROWS];
        for i in 0..COURTYARD_ROWS.min(Renderer::FOV_ROWS) {
            for j in 0..COURTYARD_COLS.min(Renderer::FOV_COLS) {
                cells[i][j] = BKG_EL;
            }
        }

        cells
    }

    fn add_objs_to_map(
        castle_id: &Option<GameId>,
        cells: &mut Vec<Vec<TermCell>>,
        objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
    ) {
        for (id, obj) in objs.iter() {
            if !Self::is_in_view(obj.get_pos(), camera.map, obj.get_art_size(false)) {
                continue;
            };
            let pos = obj.get_pos();
            let Some(term_coord_rel) = TermCoord::from_game_coord(pos, camera, true) else {
                continue;
            };

            let owned = match obj {
                GameObjE::Castle(_) => *castle_id == Some(*id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == *castle_id,
                _ => false,
            };

            let art = obj.get_art(false, owned);
            Self::add_art_to_cells(cells, art, term_coord_rel);
        }
    }

    fn add_objs_to_world_map(
        castle_id: &Option<GameId>,
        cells: &mut Vec<Vec<TermCell>>,
        world_objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
    ) {
        for (id, obj) in world_objs.iter() {
            let pos = obj.get_pos();
            let Some(term_coord_rel) = TermCoord::from_game_coord(pos, camera, true) else {
                continue;
            };

            let owned = *castle_id == Some(*id);
            let art = obj.get_art(true, owned);
            Self::add_art_to_cells(cells, art, term_coord_rel);
        }
    }

    fn add_facilities_to_courtyard(
        cells: &mut Vec<Vec<TermCell>>,
        facilities: &Option<[Vec<Facility>; FacilityType::COUNT]>,
        camera: &Camera,
        night: bool,
    ) {
        let Some(facilities) = facilities else {
            return;
        };

        for facilities_of_one_type in facilities.iter() {
            for facility in facilities_of_one_type.iter() {
                let art = FacilityAsset::get_asset(facility, night);
                let pos = facility.pos;
                let Some(term_coord_rel) = TermCoord::from_game_coord(pos, camera, true) else {
                    continue;
                };
                Self::add_art_to_cells(cells, art, term_coord_rel);
            }
        }
    }

    // TODO: fix this. This can take negative positions to account for the objects that have origin outsize of view but
    // with art that enters the view
    fn add_art_to_cells(cells: &mut [Vec<TermCell>], art: &[&[TermCell]], pos: TermCoord) {
        for (art_row, art_row_iter) in art.iter().enumerate() {
            for (art_col, art_cell) in art_row_iter.iter().enumerate() {
                let cell_pos_y = pos.y + art_row;
                let cell_pos_x = pos.x + art_col;
                if cell_pos_y >= 0
                    && cell_pos_x >= 0
                    && cell_pos_y < cells.len()
                    && cell_pos_x < cells[0].len()
                {
                    cells[cell_pos_y][cell_pos_x] = *art_cell;
                }
            }
        }
    }

    // fn get_map_slice(tiles: &[Vec<Tile>], camera_pos: GameCoord) -> Vec<Vec<Tile>> {
    //     tiles[camera_pos.y..(camera_pos.y + Renderer::FOV_ROWS * 2).min(MAP_ROWS)]
    //         .iter()
    //         .map(|row| {
    //             row[camera_pos.x..(camera_pos.x + Renderer::FOV_COLS).min(MAP_COLS)].to_vec()
    //         })
    //         .collect()
    // }

    // fn get_wind_slice(wind: &[Vec<bool>], camera_pos: GameCoord) -> Vec<Vec<bool>> {
    //     wind[camera_pos.y / 2..(camera_pos.y / 2 + Renderer::FOV_ROWS).min(MapData::WIND_ROWS)]
    //         .iter()
    //         .map(|row| {
    //             row[camera_pos.x..(camera_pos.x + Renderer::FOV_COLS).min(MapData::WIND_COLS)]
    //                 .to_vec()
    //         })
    //         .collect()
    // }

    fn is_in_view(pos: GameCoord, camera_pos: GameCoord, obj_size: (usize, usize)) -> bool {
        let y = pos.y + obj_size.0 >= camera_pos.y && pos.y < camera_pos.y + Renderer::FOV_ROWS * 2;
        let x = pos.x + obj_size.1 >= camera_pos.x && pos.x < camera_pos.x + Renderer::FOV_COLS;
        y && x
    }
}
