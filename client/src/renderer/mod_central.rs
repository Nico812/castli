use std::collections::HashMap;

use common::r#const::{COURTYARD_COLS, COURTYARD_ROWS};
use common::courtyard::Facility;
use common::game_objs::GameObjE;
use common::map::Tile;
use common::{GameCoord, GameId};

use crate::assets::*;
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::r#const::{COURTYARD_BK_CELL, FOV_COLS, FOV_ROWS};
use crate::renderer::map_data::MapData;
use crate::renderer::module::Module;
use crate::renderer::renderer::Renderer;
use crate::ui_state::{Camera, CameraLocation, UiMode, UiState};

pub struct ModCentral {
    module: Module,
}

impl ModCentral {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn render(
        &mut self,
        game_state: &mut GameState,
        ui_state: &UiState,
        map_data: &MapData,
    ) -> &Vec<Vec<TermCell>> {
        let camera_coord = ui_state.camera.get_pos();

        let title = match ui_state.camera.location {
            CameraLocation::Map => {
                let title = format!("Castli | map {}", camera_coord);
                self.draw_map(
                    &game_state.map,
                    &map_data.variants,
                    game_state.time.night,
                    &ui_state.camera,
                );
                self.draw_objs(
                    &game_state.player.castle_id,
                    &game_state.objs,
                    &ui_state.camera,
                );
                title
            }
            CameraLocation::WorldMap => {
                let title = "Castli | world map".to_string();
                self.draw_world_map(
                    &map_data.tiles_wor,
                    &map_data.variants,
                    game_state.time.night,
                );

                self.draw_world_objs(
                    &game_state.player.castle_id,
                    &game_state.objs,
                    &ui_state.camera,
                );
                title
            }
            CameraLocation::Courtyard => {
                let title = format!("Castli | courtyard {}", camera_coord);
                self.draw_courtyard(&ui_state.camera);
                self.draw_facilities(
                    &game_state.facilities,
                    &ui_state.camera,
                    game_state.time.night,
                );
                title
            }
        };

        // Adding the cursor
        if let UiMode::Inspect(ref inspect) = ui_state.mode
            && let Some(term_coord) = TermCoord::from_game_coord(inspect.coord, &ui_state.camera)
        {
            self.module.draw_asset(&CURSOR, term_coord);
        }

        self.module.set_name(title);
        self.module.center();
        self.module.get_cells()
    }

    fn draw_map(
        &mut self,
        tiles: &[Vec<Tile>],
        variants: &[Vec<bool>],
        night: bool,
        camera: &Camera,
    ) {
        let drawable_size = self.module.drawable_size();
        let camera_pos = camera.map;

        for tile_row in camera_pos.y..camera_pos.y + drawable_size.y * 2 {
            if tile_row & 1 == 1 {
                continue;
            }
            for tile_col in camera_pos.x..camera_pos.x + drawable_size.x {
                let Some(term_pos) =
                    TermCoord::from_game_coord(GameCoord::new(tile_row, tile_col), camera)
                else {
                    continue;
                };
                let tile_top = tiles.get(tile_row).and_then(|row| row.get(tile_col));
                let tile_bot = tiles.get(tile_row + 1).and_then(|row| row.get(tile_col));

                if let (Some(tile_top_), Some(tile_bot_)) = (tile_top, tile_bot) {
                    let top_tile_asset = TileAsset::get_asset(*tile_top_, night);
                    let bot_tile_asset = TileAsset::get_asset(*tile_bot_, night);

                    let cell;
                    if tile_top_ == tile_bot_ {
                        let variant = variants
                            .get(tile_row)
                            .and_then(|row| row.get(tile_col))
                            .copied()
                            .unwrap_or(false);

                        cell = if variant {
                            top_tile_asset.wind
                        } else {
                            top_tile_asset.std
                        };
                    } else {
                        cell = TermCell::new(BLOCK, top_tile_asset.up, bot_tile_asset.down);
                    }
                    self.module.draw_cell(cell, term_pos);
                }
            }
        }
    }

    fn draw_world_map(&mut self, tiles: &[Vec<Tile>], variants: &[Vec<bool>], night: bool) {
        let drawable_size = self.module.drawable_size();

        for tile_row in 0..drawable_size.y * 2 {
            if tile_row & 1 == 1 {
                continue;
            }
            for tile_col in 0..drawable_size.x {
                let term_pos = TermCoord::new(tile_row / 2, tile_col);
                let tile_top = tiles.get(tile_row).and_then(|row| row.get(tile_col));
                let tile_bot = tiles.get(tile_row + 1).and_then(|row| row.get(tile_col));

                if let (Some(tile_top_), Some(tile_bot_)) = (tile_top, tile_bot) {
                    let top_tile_asset = TileAsset::get_asset(*tile_top_, night);
                    let bot_tile_asset = TileAsset::get_asset(*tile_bot_, night);

                    let cell;
                    if tile_top_ == tile_bot_ {
                        let variant = variants
                            .get(tile_row)
                            .and_then(|row| row.get(tile_col))
                            .copied()
                            .unwrap_or(false);

                        cell = if variant {
                            top_tile_asset.wind
                        } else {
                            top_tile_asset.std
                        };
                    } else {
                        cell = TermCell::new(BLOCK, top_tile_asset.up, bot_tile_asset.down);
                    }
                    self.module.draw_cell(cell, term_pos);
                }
            }
        }
    }

    fn draw_courtyard(&mut self, camera: &Camera) {
        let drawable_size = self.module.drawable_size();
        let camera_pos = camera.courtyard;

        for tile_row in camera_pos.y..camera_pos.y + drawable_size.y * 2 {
            if tile_row & 1 == 1 {
                continue;
            }
            for tile_col in camera_pos.x..camera_pos.x + drawable_size.x {
                let Some(term_pos) =
                    TermCoord::from_game_coord(GameCoord::new(tile_row, tile_col), camera)
                else {
                    continue;
                };

                if tile_row < COURTYARD_ROWS && tile_col < COURTYARD_COLS {
                    self.module.draw_cell(COURTYARD_BK_CELL, term_pos);
                }
            }
        }
    }

    fn draw_objs(
        &mut self,
        castle_id: &Option<GameId>,
        objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
    ) {
        let drawable_size = self.module.drawable_size();
        for (id, obj) in objs.iter() {
            let pos = obj.get_pos();
            if pos.y < camera.map.y
                || pos.x < camera.map.x
                || pos.y >= camera.map.y + drawable_size.y * 2
                || pos.x >= camera.map.x + drawable_size.x
            {
                continue;
            };
            let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
                continue;
            };

            let owned = match obj {
                GameObjE::Castle(_) => *castle_id == Some(*id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == *castle_id,
                _ => false,
            };

            let asset = GameObjAsset::get_asset(obj, owned);
            self.module.draw_asset(asset, term_coord);
        }
    }

    fn draw_world_objs(
        &mut self,
        castle_id: &Option<GameId>,
        world_objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
    ) {
        let drawable_size = self.module.drawable_size();
        for (id, obj) in world_objs.iter() {
            let pos = obj.get_pos();
            if pos.y >= (drawable_size.y * FOV_ROWS) * 2 || pos.x >= drawable_size.x * FOV_COLS {
                continue;
            };

            let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
                continue;
            };

            let owned = match obj {
                GameObjE::Castle(_) => *castle_id == Some(*id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == *castle_id,
                _ => false,
            };

            let asset = GameObjAsset::get_asset(obj, owned);
            self.module.draw_asset(asset, term_coord);
        }
    }

    fn draw_facilities(
        &mut self,
        facilities: &HashMap<u8, Facility>,
        camera: &Camera,
        night: bool,
    ) {
        for (_, facility) in facilities.iter() {
            let asset = FacilityAsset::get_asset(facility, night);
            let pos = facility.pos;
            let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
                continue;
            };

            self.module.draw_asset(asset, term_coord);
        }
    }
}
