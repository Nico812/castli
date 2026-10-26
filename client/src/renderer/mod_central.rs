use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use common::r#const::{COURTYARD_COLS, COURTYARD_ROWS, MAP_COLS, MAP_ROWS};
use common::courtyard::Facility;
use common::game_objs::GameObjE;
use common::map::Tile;
use common::{GameCoord, GameId};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::assets::*;
use crate::camera::{Camera, CameraLocation};
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::module::Module;
use crate::renderer::renderer::DrawableCell;
use crate::ui_state::{UiMode, UiState};

pub struct ModCentral {
    module: Module,
    courtyard_tiles: Vec<Vec<Tile>>,
    variants_map: Vec<Vec<bool>>,
    variants_courtyard: Vec<Vec<bool>>,
}

impl ModCentral {
    pub fn new(module: Module) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);

        let assets_dir_path = Assets::get_assets_path();

        let mut courtyard_tiles = vec![vec![Tile::Grass; COURTYARD_COLS]; COURTYARD_ROWS];
        let courtyard_tiles_str = read_to_string(assets_dir_path.join("courtyard.txt"))
            .expect("Failed to read courtyard asset");
        for (i, line) in courtyard_tiles_str.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == 'W' {
                    courtyard_tiles[i][j] = Tile::Water;
                }
            }
        }

        let mut variants_map = vec![vec![false; MAP_COLS]; MAP_ROWS.div_ceil(2)];
        let mut variants_courtyard = vec![vec![false; COURTYARD_COLS]; COURTYARD_ROWS.div_ceil(2)];
        for cell in variants_map.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }
        for cell in variants_courtyard.iter_mut().flat_map(|row| row.iter_mut()) {
            *cell = rng.random_bool(0.1);
        }

        Self {
            module,
            courtyard_tiles,
            variants_map,
            variants_courtyard,
        }
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
                Self::draw_tiles(
                    &mut self.module,
                    &game_state.map,
                    &self.variants_map,
                    &ui_state.camera,
                    &assets,
                    &game_state,
                );
                self.draw_objs(&ui_state.camera, &assets, game_state);
                title
            }
            CameraLocation::WorldMap => {
                let title = "Castli | world map".to_string();
                self.draw_world_map(&ui_state.camera, &assets);

                self.draw_world_objs(&ui_state.camera, &assets, &game_state);
                title
            }
            CameraLocation::Courtyard => {
                let title = format!("Castli | courtyard {}", camera_coord);
                Self::draw_tiles(
                    &mut self.module,
                    &self.courtyard_tiles,
                    &self.variants_courtyard,
                    &ui_state.camera,
                    &assets,
                    game_state,
                );
                self.draw_facilities(&ui_state.camera, &assets, &game_state);
                title
            }
        };

        // Adding the cursor
        if let UiMode::Inspect(ref inspect) = ui_state.mode {
            let cursor_pos = TermCoord::from_game_coord(inspect.coord, &ui_state.camera);
            self.module
                .draw_cell(assets.get_ui_element_drawable("CURSOR_L"), cursor_pos);
            self.module.draw_cell(
                assets.get_ui_element_drawable("CURSOR_R"),
                cursor_pos + TermCoord::new(0, 1),
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
        let y_factor = MAP_ROWS.div_ceil((fov_size.y * 2).max(1) as usize);
        let x_factor = MAP_COLS.div_ceil((fov_size.x).max(1) as usize);
        x_factor.max(y_factor).max(1)
    }

    fn draw_tiles(
        module: &mut Module,
        tiles: &Vec<Vec<Tile>>,
        variants: &Vec<Vec<bool>>,
        camera: &Camera,
        assets: &Assets,
        game_state: &GameState,
    ) {
        let drawable_size = module.drawable_size();
        let camera_pos = camera.get_pos();
        let night = game_state.time.night;

        for tile_row in camera_pos.y..camera_pos.y + drawable_size.y as usize * 2 {
            if tile_row & 1 == 1 {
                continue;
            }
            for tile_col in camera_pos.x..camera_pos.x + drawable_size.x as usize {
                let term_pos =
                    TermCoord::from_game_coord(GameCoord::new(tile_row, tile_col), camera);

                let tile_top = tiles.get(tile_row).and_then(|row| row.get(tile_col));
                let tile_bot = tiles.get(tile_row + 1).and_then(|row| row.get(tile_col));

                if let (Some(tile_top_), Some(tile_bot_)) = (tile_top, tile_bot) {
                    let top_tile_asset = assets.get_tile_asset(tile_top_);
                    let bot_tile_asset = assets.get_tile_asset(tile_bot_);

                    let cell;
                    if tile_top_ == tile_bot_ {
                        let variant = variants
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
                    module.draw_cell(cell, term_pos);
                }
            }
        }
    }

    fn draw_world_map(&mut self, camera: &Camera, assets: &Assets) {
        let map_term_size = TermCoord::from_game_coord(GameCoord::new(MAP_ROWS, MAP_COLS), camera);

        for tile_row in 0..map_term_size.y {
            for tile_col in 0..map_term_size.x {
                let term_pos = TermCoord::new(tile_row, tile_col);
                let cell = assets.get_ui_element_drawable("WORLD_MAP_BKG");

                self.module.draw_cell(cell, term_pos);
            }
        }
    }

    fn draw_objs(&mut self, camera: &Camera, assets: &Assets, game_state: &GameState) {
        let drawable_size = self.module.drawable_size();
        let camera_pos = camera.get_pos();
        let castle_id = game_state.player.castle_id;
        let night = game_state.time.night;
        let hitbox_map = &game_state.hitbox_map;

        let mut seen_objs = HashSet::new();
        for row in camera_pos.y..camera_pos.y + drawable_size.y as usize * 2 {
            for col in camera_pos.x..camera_pos.x + drawable_size.x as usize {
                if let Some(obj_id) = hitbox_map[row][col] {
                    seen_objs.insert(obj_id);
                }
            }
        }

        for obj_id in seen_objs {
            let Some(obj) = game_state.get_obj(obj_id) else {
                continue;
            };

            let term_coord = TermCoord::from_game_coord(obj.get_pos(), camera);

            let owned = match obj {
                GameObjE::Castle(_) => castle_id == Some(obj_id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == castle_id,
                _ => false,
            };

            let asset = assets.get_world_obj_asset_drawable(obj, owned, night);
            self.module.draw_asset(asset, term_coord);
        }

        // for (id, obj) in objs.iter() {
        //     let pos = obj.get_pos();
        //     let camera_pos = camera.get_pos();
        //     if pos.y < camera_pos.y
        //         || pos.x < camera_pos.x
        //         || pos.y >= camera_pos.y + drawable_size.y * 2
        //         || pos.x >= camera_pos.x + drawable_size.x
        //     {
        //         continue;
        //     };
        //     let Some(term_coord) = TermCoord::from_game_coord(pos, camera) else {
        //         continue;
        //     };

        //     let owned = match obj {
        //         GameObjE::Castle(_) => *castle_id == Some(*id),
        //         GameObjE::DeployedUnits(units) => Some(units.owner_id) == *castle_id,
        //         _ => false,
        //     };

        //     let asset = assets.get_world_obj_asset_drawable(obj, owned, night);
        //     self.module.draw_asset(asset, term_coord);
        // }
    }

    fn draw_world_objs(&mut self, camera: &Camera, assets: &Assets, game_state: &GameState) {
        let night = game_state.time.night;
        let world_objs = game_state.get_objs();
        let castle_id = game_state.player.castle_id;

        for (id, obj) in world_objs.iter() {
            let pos = obj.get_pos();

            let term_coord = TermCoord::from_game_coord(pos, camera);

            let owned = match obj {
                GameObjE::Castle(_) => castle_id == Some(*id),
                GameObjE::DeployedUnits(units) => Some(units.owner_id) == castle_id,
                _ => false,
            };

            let asset = assets.get_world_obj_asset_drawable(obj, owned, night);
            self.module.draw_asset(asset, term_coord);
        }
    }

    fn draw_facilities(&mut self, camera: &Camera, assets: &Assets, game_state: &GameState) {
        let night = game_state.time.night;
        let facilities = game_state.get_facilities();

        for (_, facility) in facilities.iter() {
            let asset = assets.get_facility_asset_drawable(facility, night);

            let pos = facility.pos;
            let term_coord = TermCoord::from_game_coord(pos, camera);

            self.module.draw_asset(asset, term_coord);
        }
    }
}
