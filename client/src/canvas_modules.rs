use rand::{self, Rng};

use crate::ansi::*;
use common::r#const;

pub const CENTRAL_MODULE_SIZE: usize = 64;

pub struct CentralModule {
    pub content: Vec<Vec<String>>,
}

impl CentralModule {
    pub fn new() -> Self {
        let content = vec![vec!["C".to_owned(); CENTRAL_MODULE_SIZE]; CENTRAL_MODULE_SIZE / 2];
        Self { content }
    }

    pub fn set_map(&mut self) {}

    pub fn set_strutures(&mut self, structures: &Vec<common::StructureE>) {}

    pub fn set_map_zoomed(&mut self, tiles: &Vec<Vec<common::TileE>>, quadrant: (usize, usize)) {
        let map_start_row = quadrant.0 * CENTRAL_MODULE_SIZE;
        let map_start_col = quadrant.1 * CENTRAL_MODULE_SIZE;
        let mut rng = rand::rng();

        let mut tiles_row;
        let mut tiles_col;
        for term_row in 0..CENTRAL_MODULE_SIZE / 2 {
            tiles_row = term_row * 2 + map_start_row;
            for term_col in 0..CENTRAL_MODULE_SIZE {
                tiles_col = term_col + map_start_col;
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
                    self.content[term_row][term_col] = char.to_string();
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
                    self.content[term_row][term_col] = top_color.to_string() + bottom_color + BLOCK;
                }
            }
            self.content[term_row][CENTRAL_MODULE_SIZE - 1] += RESET_COLOR;
        }
        self.content[CENTRAL_MODULE_SIZE / 2 - 1][CENTRAL_MODULE_SIZE - 1] += RESET_COLOR;
    }

    pub fn set_strutures_zoomed(
        &mut self,
        structures: &Vec<common::StructureE>,
        quadrant: (usize, usize),
    ) {
        for structure in structures.iter() {
            let term_pos = (structure.pos.0 / 2, structure.pos.1);
            if term_pos.0 < (quadrant.0 + 1) * CENTRAL_MODULE_SIZE
                && term_pos.0 >= (quadrant.0 * CENTRAL_MODULE_SIZE)
            {
                if term_pos.1 < (quadrant.1 + 1) * CENTRAL_MODULE_SIZE
                    && term_pos.1 >= (quadrant.1 * CENTRAL_MODULE_SIZE)
                {
                    if structure.struc_type == common::StructureTypeE::Castle {
                        for row in (term_pos.0 % CENTRAL_MODULE_SIZE)
                            ..(term_pos.0 % CENTRAL_MODULE_SIZE) + r#const::CASTLE_SIZE / 2
                        {
                            for col in (term_pos.1 % CENTRAL_MODULE_SIZE)
                                ..(term_pos.1 % CENTRAL_MODULE_SIZE) + r#const::CASTLE_SIZE
                            {
                                self.content[row][col] = "C".to_string();
                            }
                        }
                    }
                }
            }
        }
    }
}
