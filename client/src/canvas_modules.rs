//! # TUI Canvas Modules
//!
//! This module defines the individual components or "modules" that make up the `Canvas`.
//! Each module is responsible for generating a specific part of the UI, such as the
//! map view, player information panel, or event log.

use rand::SeedableRng;
use rand::{self, Rng, rngs};
use std::collections::HashMap;

use crate::ansi::*;
use crate::assets::*;
use crate::r#const::*;
use common::r#const::{self, MAP_COLS, MAP_ROWS};

pub struct CentralModule {
    // Stores the tiles for the rest of the game, since they should be immutable
    map_tiles: Vec<Vec<common::TileE>>,
    world_map_tiles: Vec<Vec<common::TileE>>,
    wind_map: Vec<Vec<bool>>,
    rng: rngs::SmallRng,
}

pub struct LeftModule {
    // Player data
}

pub struct RightModule {
    // Inspect
}

pub struct BottomModule {
    // Game events
}

impl CentralModule {
    pub fn new() -> Self {
        let map_tiles = vec![vec![common::TileE::Grass; r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles =
            vec![vec![common::TileE::Grass; r#const::MAP_COLS / 8]; r#const::MAP_ROWS / 8];

        // Wind
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let mut wind_map = vec![vec![false; r#const::MAP_COLS]; r#const::MAP_ROWS];
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

    pub fn get_content(
        &mut self,
        game_objs: &HashMap<common::GameID, common::GameObjE>,
        map_zoom: Option<(usize, usize)>,
        render_count: u32,
    ) -> Vec<Vec<TermCell>> {
        match map_zoom {
            Some(quadrant) => {
                let cut_map = self.get_map_cut(quadrant);
                let cut_wind_map = self.get_wind_map_cut(quadrant);
                let mut content = Self::tiles_to_cells(&cut_map, &cut_wind_map);
                Self::add_objs_to_map(&mut content, game_objs, quadrant);
                //add_frame(&mut content, true);

                self.update_wind(render_count, quadrant);
                content
            }
            None => {
                let cut_wind_map = self.get_wind_map_cut((7, 7));
                let mut content = Self::tiles_to_cells(&self.world_map_tiles, &cut_wind_map);
                Self::add_objs_to_world_map(&mut content, game_objs);
                //add_frame(&mut content, false);
                content
            }
        }
    }

    fn set_tiles(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        fn compact_8x8_tiles(
            tiles: &Vec<Vec<common::TileE>>,
            pos: (usize, usize),
        ) -> common::TileE {
            let mut grass_counter = 0;
            let mut water_counter = 0;

            for row in pos.0..(pos.0 + 8).min(MAP_ROWS) {
                for col in pos.1..(pos.1 + 8).min(MAP_COLS) {
                    match tiles[row][col] {
                        common::TileE::Grass => grass_counter += 1,
                        common::TileE::Water => water_counter += 1,
                        _ => {}
                    }
                }
            }
            if grass_counter >= water_counter {
                common::TileE::Grass
            } else {
                common::TileE::Water
            }
        }
        let mut compacted = vec![vec![common::TileE::Grass; MAP_COLS / 8]; MAP_ROWS / 8];
        for row in 0..MAP_ROWS / 8 {
            for col in 0..MAP_COLS / 8 {
                compacted[row][col] = compact_8x8_tiles(tiles, (row * 8, col * 8));
            }
        }
        self.world_map_tiles = compacted;
        self.map_tiles = tiles.clone();
    }

    fn tiles_to_cells<'a>(
        tiles: &Vec<Vec<common::TileE>>,
        wind_map: &Vec<Vec<bool>>,
    ) -> Vec<Vec<TermCell>> {
        let mut cells = vec![vec![ERR_EL; tiles[0].len()]; tiles.len() / 2];
        let mut tiles_row;
        let mut tiles_col;
        for term_row in 0..tiles.len() / 2 {
            tiles_row = term_row * 2;
            for term_col in 0..tiles[tiles_row].len() {
                tiles_col = term_col;
                if tiles[tiles_row][tiles_col] == tiles[tiles_row + 1][tiles_col] {
                    let cell;
                    match tiles[tiles_row][tiles_col] {
                        common::TileE::Grass => {
                            if wind_map[tiles_row][tiles_col] {
                                cell = GRASS_EL_2;
                            } else {
                                cell = GRASS_EL_1;
                            }
                        }
                        common::TileE::Water => {
                            if wind_map[tiles_row][tiles_col] {
                                cell = WATER_EL_2;
                            } else {
                                cell = WATER_EL_1;
                            }
                        }
                        _ => {
                            cell = ERR_EL;
                        }
                    }
                    cells[term_row][term_col] = cell;
                } else {
                    let fg_color;
                    let bg_color;
                    match tiles[tiles_row][tiles_col] {
                        common::TileE::Grass => {
                            fg_color = GRASS_FG;
                        }
                        common::TileE::Water => {
                            fg_color = WATER_FG;
                        }
                        _ => {
                            fg_color = ERR_FG;
                        }
                    }
                    match tiles[tiles_row + 1][tiles_col] {
                        common::TileE::Grass => {
                            bg_color = GRASS_BG;
                        }
                        common::TileE::Water => {
                            bg_color = WATER_BG;
                        }
                        _ => {
                            bg_color = ERR_BG;
                        }
                    }
                    cells[term_row][term_col] = TermCell::new(BLOCK, fg_color, bg_color);
                }
            }
        }
        cells
    }

    fn add_objs_to_world_map(
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

    fn get_map_cut(&self, quadrant: (usize, usize)) -> Vec<Vec<common::TileE>> {
        self.map_tiles
            [quadrant.0 * CENTRAL_MODULE_ROWS * 2..(quadrant.0 + 1) * CENTRAL_MODULE_ROWS * 2]
            .iter()
            .map(|row| {
                row[quadrant.1 * CENTRAL_MODULE_COLS..(quadrant.1 + 1) * CENTRAL_MODULE_COLS]
                    .to_vec()
            })
            .collect()
    }

    fn get_wind_map_cut(&self, quadrant: (usize, usize)) -> Vec<Vec<bool>> {
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

impl LeftModule {
    const PADDING_LEFT: usize = 1;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self, player_data: &common::PlayerDataE) -> Vec<Vec<TermCell>> {
        let blank_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); LEFT_MODULE_COLS];
        let mut content = vec![blank_row.clone(); LEFT_MODULE_ROWS];

        let name = &player_data.name;
        for (i, ch) in name.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[3][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BRIGHT_YELLOW);
            }
        }
        let pos_str = format!("({}, {})", player_data.pos.0, player_data.pos.1);
        for (i, ch) in pos_str.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[5][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BRIGHT_YELLOW);
            }
        }
        content
    }
}

impl RightModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self, frame_dt: u64) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); RIGHT_MODULE_COLS];
            RIGHT_MODULE_ROWS
        ];

        let dt_str = format!("Frame dt: {} ms", frame_dt);

        for (i, ch) in dt_str.chars().enumerate() {
            if Self::PADDING_LEFT + i < RIGHT_MODULE_COLS {
                content[1][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BLUE);
            }
        }
        content
    }
}

impl BottomModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); BOTTOM_MODULE_COLS];
            BOTTOM_MODULE_ROWS
        ];
        content
    }
}
