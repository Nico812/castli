use std::collections::HashMap;

use common::r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS};
use common::courtyard::{Facility, FacilityType};
use common::game_objs::GameObjE;
use common::map::Tile;
use common::{GameCoord, GameId};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::ansi::{BLACK, WHITE};
use crate::assets::*;
use crate::camera::{Camera, CameraLocation};
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::module::Module;
use crate::renderer::renderer::DrawableCell;
use crate::ui_state::{UiMode, UiState};

pub struct ModCentral {
    module: Module,
    variants: Vec<Vec<bool>>,
}

impl ModCentral {
    pub fn new(module: Module) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);

        let mut variants = vec![vec![false; MAP_COLS]; MAP_ROWS.div_ceil(2)];
        for cell in variants.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self { module, variants }
    }

    pub fn render(
        &mut self,
        assets: &Assets,
        game_state: &mut GameState,
        ui_state: &UiState,
    ) -> Vec<Vec<DrawableCell>> {
        let camera_coord = ui_state.camera.get_pos();

        let title = match ui_state.camera.location {
            CameraLocation::Map => {
                let title = format!("Castli | map {}", camera_coord);
                self.draw_map(
                    &game_state.map,
                    game_state.time.night,
                    &ui_state.camera,
                    &assets,
                );
                self.draw_objs(
                    &game_state.player.castle_id,
                    &game_state.objs,
                    &ui_state.camera,
                    game_state.time.night,
                    &assets,
                );
                title
            }
            CameraLocation::WorldMap => {
                let title = "Castli | world map".to_string();
                self.draw_world_map(&ui_state.camera, &assets);

                self.draw_world_objs(
                    &game_state.player.castle_id,
                    &game_state.objs,
                    &ui_state.camera,
                    game_state.time.night,
                    &assets,
                );
                title
            }
            CameraLocation::Courtyard => {
                let title = format!("Castli | courtyard {}", camera_coord);
                self.draw_courtyard(&ui_state.camera, &assets);
                self.draw_facilities(
                    &game_state.facilities,
                    &ui_state.camera,
                    game_state.time.night,
                    &assets,
                );
                title
            }
        };

        // Adding the cursor
        if let UiMode::Inspect(ref inspect) = ui_state.mode
            && let Some(term_coord) = TermCoord::from_game_coord(inspect.coord, &ui_state.camera)
        {
            self.module
                .draw_cell(assets.get_ui_element_drawable("CURSOR_L"), term_coord);
            self.module.draw_cell(
                assets.get_ui_element_drawable("CURSOR_R"),
                term_coord + TermCoord::new(0, 1),
            );
        }

        self.module.set_name(title);
        self.module.center();
        self.module.get_cells()
    }

    pub fn fov_size(&self) -> TermCoord {
        self.module.drawable_size()
    }

    pub fn zoom_factor(&self) -> usize {
        let fov_size = self.fov_size();
        let y_factor = MAP_ROWS.div_ceil((fov_size.y * 2).max(1));
        let x_factor = MAP_COLS.div_ceil((fov_size.x).max(1));
        x_factor.max(y_factor).max(1)
    }

    fn draw_map(&mut self, tiles: &[Vec<Tile>], night: bool, camera: &Camera, assets: &Assets) {
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
                    let top_tile_asset = assets.get_tile_asset(tile_top_);
                    let bot_tile_asset = assets.get_tile_asset(tile_bot_);

                    let cell;
                    if tile_top_ == tile_bot_ {
                        let variant = self
                            .variants
                            .get(tile_row)
                            .and_then(|row| row.get(tile_col))
                            .copied()
                            .unwrap_or(false);

                        cell = if variant {
                            top_tile_asset.var.to_drawable_cell(night)
                        } else {
                            top_tile_asset.std.to_drawable_cell(night)
                        };
                    } else {
                        cell = DrawableCell::new(
                            BLOCK,
                            top_tile_asset.up.to_crossterm_color(night),
                            bot_tile_asset.down.to_crossterm_color(night),
                        );
                    }
                    self.module.draw_cell(cell, term_pos);
                }
            }
        }
    }

    fn draw_world_map(&mut self, camera: &Camera, assets: &Assets) {
        let map_term_size =
            TermCoord::from_game_coord(GameCoord::new(MAP_ROWS, MAP_COLS), camera).unwrap();

        for tile_row in 0..map_term_size.y {
            for tile_col in 0..map_term_size.x {
                let term_pos = TermCoord::new(tile_row, tile_col);
                let cell = assets.get_ui_element_drawable("WORLD_MAP_BKG");

                self.module.draw_cell(cell, term_pos);
            }
        }
    }

    fn draw_courtyard(&mut self, camera: &Camera, assets: &Assets) {
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
                    self.module
                        .draw_cell(assets.get_ui_element_drawable("WORLD_MAP_BKG"), term_pos);
                }
            }
        }
    }

    fn draw_objs(
        &mut self,
        castle_id: &Option<GameId>,
        objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
        night: bool,
        assets: &Assets,
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

            let asset = assets.get_world_obj_asset_drawable(obj, owned, night);
            self.module.draw_asset(asset, term_coord);
        }
    }

    fn draw_world_objs(
        &mut self,
        castle_id: &Option<GameId>,
        world_objs: &HashMap<GameId, GameObjE>,
        camera: &Camera,
        night: bool,
        assets: &Assets,
    ) {
        for (id, obj) in world_objs.iter() {
            let pos = obj.get_pos();

            let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
                continue;
            };

            let owned = match obj {
                GameObjE::Castle(_) => *castle_id == Some(*id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == *castle_id,
                _ => false,
            };

            let asset = assets.get_world_obj_asset_drawable(obj, owned, night);
            self.module.draw_asset(asset, term_coord);
        }
    }

    fn draw_facilities(
        &mut self,
        facilities: &HashMap<u8, Facility>,
        camera: &Camera,
        night: bool,
        assets: &Assets,
    ) {
        for (_, facility) in facilities.iter() {
            let asset = assets.get_facility_asset_drawable(facility, night);

            let pos = facility.pos;
            let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
                continue;
            };

            self.module.draw_asset(asset, term_coord);
        }
    }
}
