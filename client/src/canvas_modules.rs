//! # TUI Canvas Modules
//!
//! This module defines the individual components or "modules" that make up the `Canvas`.
//! Each module is responsible for generating a specific part of the UI, such as the
//! map view, player information panel, or event log.

use rand::{self, Rng};

use crate::ansi::*;
use common::r#const::{self, MAP_COLS, MAP_ROWS};

pub const CENTRAL_MODULE_ROWS: usize = r#const::MAP_ROWS / 16;
pub const CENTRAL_MODULE_COLS: usize = r#const::MAP_COLS / 8;
pub const LEFT_MODULE_ROWS: usize = r#const::MAP_ROWS / 16;
pub const LEFT_MODULE_COLS: usize = 18;
pub const RIGHT_MODULE_ROWS: usize = r#const::MAP_ROWS / 16;
pub const RIGHT_MODULE_COLS: usize = 13;
pub const BOTTOM_MODULE_ROWS: usize = 7;
pub const BOTTOM_MODULE_COLS: usize = r#const::MAP_COLS / 8;

pub struct CentralModule {
    // Stores the tiles for the rest of the game, since they should be immutable
    map_tiles: Vec<Vec<common::TileE>>,
    world_map_tiles: Vec<Vec<common::TileE>>,
    map_tiles_formatted: Vec<Vec<String>>,
    world_map_tiles_formatted: Vec<Vec<String>>,
}

pub struct LeftModule {
    // Player data
    player_name: String,
}

pub struct RightModule {
    // Inspect
}

pub struct BottomModule {
    // Game events
}

fn add_frame(content: &mut Vec<Vec<String>>, with_markers: bool) {
    let content_rows = content.len();
    let content_cols = content[0].len();

    let top_right_corner = "+".to_owned();
    let bottom_left_corner = top_right_corner.clone();
    let bottom_right_corner = top_right_corner.clone();
    let top_left_corner = match with_markers {
        true => "0".to_owned(),
        false => top_right_corner.clone(),
    };

    let mut bottom_border = vec!["-".to_owned(); content_cols];
    let right_border = vec![concat!(RESET_COLOR!(), "|").to_owned(); content_rows];
    let mut top_border = bottom_border.clone();
    let mut left_border = right_border.clone();
    if with_markers {
        for col_marker in 1..content_cols / 8 {
            top_border[col_marker * 8] = col_marker.to_string();
        }
        for row_marker in 1..content_rows / 4 {
            left_border[row_marker * 4] = row_marker.to_string();
        }
    }

    for row in 0..content_rows {
        content[row].insert(0, left_border[row].clone());
        content[row].push(right_border[row].clone());
    }
    let mut top_row = vec![top_left_corner];
    top_row.append(&mut top_border);
    top_row.push(top_right_corner);
    let mut bottom_row = vec![bottom_left_corner];
    bottom_row.append(&mut bottom_border);
    bottom_row.push(bottom_right_corner);

    content.insert(0, top_row);
    content.push(bottom_row);
}

impl CentralModule {
    pub fn new() -> Self {
        let map_tiles = vec![vec![common::TileE::Grass; r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles =
            vec![vec![common::TileE::Grass; r#const::MAP_COLS / 8]; r#const::MAP_ROWS / 8];
        let map_tiles_formatted =
            vec![vec![ERR_VARIANT.to_owned(); r#const::MAP_COLS]; r#const::MAP_ROWS];
        let world_map_tiles_formatted =
            vec![vec![ERR_VARIANT.to_owned(); r#const::MAP_COLS / 8]; r#const::MAP_ROWS / 8];

        Self {
            map_tiles,
            world_map_tiles,
            map_tiles_formatted,
            world_map_tiles_formatted,
        }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.set_tiles(tiles);
        self.format_tiles();
    }

    pub fn get_map(
        &self,
        structures: &Vec<common::StructureE>,
        map_zoom: Option<(usize, usize)>,
    ) -> Vec<Vec<String>> {
        match map_zoom {
            Some(quadrant) => {
                let mut content = self.add_structures_to_map(structures, quadrant);
                add_frame(&mut content, true);
                content
            }
            None => {
                let mut content = self.add_structures_to_world_map(structures);
                add_frame(&mut content, false);
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

    fn format_tiles(&mut self) {
        fn format_tiles_core(tiles: &Vec<Vec<common::TileE>>) -> Vec<Vec<String>> {
            let mut rng = rand::rng();
            let mut tiles_formatted =
                vec![vec![ERR_VARIANT.to_owned(); tiles[0].len()]; tiles.len() / 2];
            let mut tiles_row;
            let mut tiles_col;
            for term_row in 0..tiles.len() / 2 {
                tiles_row = term_row * 2;
                for term_col in 0..tiles[tiles_row].len() {
                    tiles_col = term_col;
                    if tiles[tiles_row][tiles_col] == tiles[tiles_row + 1][tiles_col] {
                        let char;
                        match tiles[tiles_row][tiles_col] {
                            common::TileE::Grass => {
                                if rng.random_bool(0.2) {
                                    char = GRASS_VARIANTS.0;
                                } else {
                                    char = GRASS_VARIANTS.1;
                                }
                            }
                            common::TileE::Water => {
                                if rng.random_bool(0.2) {
                                    char = WATER_VARIANTS.0;
                                } else {
                                    char = WATER_VARIANTS.1;
                                }
                            }
                            _ => {
                                char = ERR_VARIANT;
                            }
                        }
                        tiles_formatted[term_row][term_col] = char.to_string();
                    } else {
                        let top_color;
                        let bottom_color;
                        match tiles[tiles_row][tiles_col] {
                            common::TileE::Grass => {
                                top_color = GRASS_COLOR.0;
                            }
                            common::TileE::Water => {
                                top_color = WATER_COLOR.0;
                            }
                            _ => {
                                top_color = ERR_COLOR.0;
                            }
                        }
                        match tiles[tiles_row + 1][tiles_col] {
                            common::TileE::Grass => {
                                bottom_color = GRASS_COLOR.1;
                            }
                            common::TileE::Water => {
                                bottom_color = WATER_COLOR.1;
                            }
                            _ => {
                                bottom_color = ERR_COLOR.1;
                            }
                        }
                        tiles_formatted[term_row][term_col] =
                            top_color.to_string() + bottom_color + BLOCK;
                    }
                }
            }
            tiles_formatted
        }
        self.map_tiles_formatted = format_tiles_core(&self.map_tiles);
        self.world_map_tiles_formatted = format_tiles_core(&self.world_map_tiles);
    }

    fn add_structures_to_world_map(
        &self,
        structures: &Vec<common::StructureE>,
    ) -> Vec<Vec<String>> {
        let mut output = self.world_map_tiles_formatted.clone();

        for structure in structures.iter() {
            let term_pos = (structure.pos.0 / 16, structure.pos.1 / 8);
            match structure.struc_type {
                common::StructureTypeE::Castle => {
                    for (row, ansi_row) in CASTLE_ART_WORLD.iter().enumerate() {
                        for (col, ansi_char) in ansi_row.iter().enumerate() {
                            output[term_pos.0 + row][term_pos.1 + col] = ansi_char.to_string();
                        }
                    }
                }
                _ => {}
            }
        }
        output
    }

    fn add_structures_to_map(
        &self,
        structures: &Vec<common::StructureE>,
        quadrant: (usize, usize),
    ) -> Vec<Vec<String>> {
        let mut output: Vec<Vec<String>> = self.map_tiles_formatted
            [quadrant.0 * CENTRAL_MODULE_ROWS..(quadrant.0 + 1) * CENTRAL_MODULE_ROWS]
            .iter()
            .map(|row| {
                row[quadrant.1 * CENTRAL_MODULE_COLS..(quadrant.1 + 1) * CENTRAL_MODULE_COLS]
                    .to_vec()
            })
            .collect();

        for structure in structures.iter() {
            let str_term_pos = (structure.pos.0 / 2, structure.pos.1);
            if str_term_pos.0 < (quadrant.0 + 1) * CENTRAL_MODULE_ROWS
                && str_term_pos.0 >= (quadrant.0 * CENTRAL_MODULE_ROWS)
            {
                if str_term_pos.1 < (quadrant.1 + 1) * CENTRAL_MODULE_COLS
                    && str_term_pos.1 >= (quadrant.1 * CENTRAL_MODULE_COLS)
                {
                    match structure.struc_type {
                        common::StructureTypeE::Castle => {
                            for ansi_art_row in 0..r#const::CASTLE_SIZE / 2 {
                                let output_row =
                                    str_term_pos.0 % CENTRAL_MODULE_ROWS + ansi_art_row;
                                for ansi_art_col in 0..r#const::CASTLE_SIZE {
                                    let output_col =
                                        str_term_pos.1 % CENTRAL_MODULE_COLS + ansi_art_col;
                                    output[output_row][output_col] =
                                        CASTLE_ART[ansi_art_row][ansi_art_col].to_owned();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        output
    }
}

impl LeftModule {
    const PADDING_LEFT: usize = 1;

    pub fn new() -> Self {
        let player_name = "ni".to_owned();
        Self { player_name }
    }

    pub fn get_content(&self) -> Vec<String> {
        let mut content: Vec<String> = vec![" ".repeat(LEFT_MODULE_COLS); LEFT_MODULE_ROWS];

        let name_line = " ".repeat(Self::PADDING_LEFT).to_owned()
            + &self.player_name.clone()
            + &" "
                .repeat(LEFT_MODULE_COLS - Self::PADDING_LEFT - &self.player_name.len())
                .to_owned();

        content[3] = name_line;
        content
    }
}

impl RightModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self) -> Vec<String> {
        let mut content: Vec<String> = vec![" ".repeat(RIGHT_MODULE_COLS); RIGHT_MODULE_ROWS];
        content
    }
}

impl BottomModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self) -> Vec<String> {
        let mut content: Vec<String> = vec![" ".repeat(BOTTOM_MODULE_COLS); BOTTOM_MODULE_ROWS];
        content
    }
}
