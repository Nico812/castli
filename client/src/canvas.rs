use rand::{self, Rng};

use common::r#const;

pub const CANVAS_CHAR_COLS: usize = 100;
pub const CANVAS_CHAR_ROWS: usize = 40;
pub const MAP_LEFT_MARGIN: usize = 10;
pub const MAP_TOP_MARGIN: usize = 5;

pub const BLOCK: &str = "▀";
pub const DEF_COLOR: &str = "\x1b[0m";

pub const ERR_COLOR: (&str, &str) = ("\x1b[35m", "\x1b[45m");
pub const ERR_VARIANT: &str = "?";

pub const GRASS_COLOR: (&str, &str) = ("\x1b[32m", "\x1b[42m");
pub const GRASS_VARIANTS: (&str, &str) = ("\x1b[33m\x1b[42m\"", "\x1b[42m ");

pub const WATER_COLOR: (&str, &str) = ("\x1b[34m", "\x1b[44m");
pub const WATER_VARIANTS: (&str, &str) = ("\x1b[35m\x1b[44m~", "\x1b[44m ");

pub struct Canvas {
    canvas: Vec<Vec<String>>,
}

impl Canvas {
    pub fn new() -> Self {
        let canvas = vec![vec!["X".to_owned(); CANVAS_CHAR_COLS]; CANVAS_CHAR_ROWS];
        Self { canvas }
    }

    pub fn print(&self) {
        for line in self.canvas.iter() {
            print!("{}", line.concat() + "\r\n");
        }
    }

    pub fn add_map(&mut self) {}

    pub fn add_map_zoomed(&mut self, tiles: &Vec<Vec<common::TileE>>, quadrant: (usize, usize)) {
        let map_start_row = quadrant.0 * 64;
        let map_start_col = quadrant.1 * 64;
        let mut rng = rand::rng();

        let mut tiles_row;
        let mut tiles_col;
        for term_row in 0..32 {
            tiles_row = term_row * 2 + map_start_row;
            for term_col in 0..64 {
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
                    self.canvas[term_row + MAP_TOP_MARGIN][term_col + MAP_LEFT_MARGIN] =
                        char.to_string();
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
                    self.canvas[term_row + MAP_TOP_MARGIN][term_col + MAP_LEFT_MARGIN] =
                        top_color.to_string() + bottom_color + BLOCK;
                }
            }
            self.canvas[term_row + MAP_TOP_MARGIN][63 + MAP_LEFT_MARGIN] += DEF_COLOR;
        }
        self.canvas[31 + MAP_TOP_MARGIN][63 + MAP_LEFT_MARGIN] += DEF_COLOR;
    }

    pub fn add_strutures_zoomed(&mut self, structures: &Vec<common::StructureE>) {
        for structure in structures.iter() {
            if structure.struc_type == common::StructureTypeE::Castle {
                let pos = structure.pos;
                for row in pos.0..pos.0 + r#const::CASTLE_SIZE {
                    for col in pos.1..pos.1 + r#const::CASTLE_SIZE {
                        self.canvas[row][col] = "C".to_string();
                    }
                }
            }
        }
    }
}
