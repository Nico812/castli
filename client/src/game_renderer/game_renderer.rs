use common::exports::tile::TileE;
use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::assets;
use crate::assets::CURSOR_DOWN;
use crate::assets::CURSOR_UP;
use crate::coord::TermCoord;
use crate::game_renderer::r#const::*;
use crate::game_renderer::map_data::MapData;
use crate::game_renderer::mod_inspect::ModInspect;
use crate::game_renderer::{mod_central::ModCentral, mod_right::ModRight};
use crate::tui::SharedState;

pub struct GameRenderer {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    map_data: MapData,
}

impl GameRenderer {
    pub const FOV_ROWS: usize = MOD_CENTRAL_ROWS - 2;
    pub const FOV_COLS: usize = MOD_CENTRAL_COLS - 2;
    pub const ZOOM_FACTOR: usize = 8;

    pub fn new(map_tiles: Vec<Vec<TileE>>) -> Self {
        let canvas_pos;
        match terminal_size() {
            Some((Width(w), Height(h))) => {
                if (h as usize) < CANVAS_ROWS || (w as usize) < CANVAS_COLS {
                    println!(
                        "Terminal size is too small, consider changing your terminal text size.."
                    );
                    canvas_pos = (0, 0);
                } else {
                    canvas_pos = (
                        ((h as usize) - CANVAS_ROWS) / 2,
                        ((w as usize) - CANVAS_COLS) / 2,
                    );
                }
            }
            None => {
                println!("Could not detect the terminal size.");
                canvas_pos = (0, 0);
            }
        }
        let prev_frame = vec![vec![assets::ERR.std; CANVAS_COLS]; CANVAS_ROWS];
        let map_data = MapData::new(map_tiles);

        Self {
            prev_frame,
            canvas_pos,
            render_count: 0,
            map_data,
        }
    }

    pub fn render(&mut self, state: &mut SharedState, frame_dt: u64) {
        self.map_data.update_wind(self.render_count, state.map_zoom);

        // Right module
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        for (row, line_contents) in ModRight::get_renderable(frame_dt, state).iter().enumerate() {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + MOD_RIGHT_POS.0][col + MOD_RIGHT_POS.1] = cell.clone();
            }
        }

        // Central module
        for (row, line_contents) in ModCentral::get_renderable(state, &self.map_data)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + MOD_CENTRAL_POS.0][col + MOD_CENTRAL_POS.1] = cell.clone();
            }
        }

        // Inspect module
        if let Some(renderable) = ModInspect::get_renderable(state, &self.map_data) {
            // TODO: Here pos_col should change based on look_coord
            let pos_row = MOD_CENTRAL_POS.0;
            let pos_col = MOD_CENTRAL_POS.1 + MOD_CENTRAL_COLS - MOD_INSPECT_COLS;

            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + pos_row][col + pos_col] = cell.clone();
                }
            }
        }

        // Adding the cursor
        if let (Some(look_coord), Some(zoom_coord)) = (state.map_look, state.map_zoom) {
            if let Some(term_coord) = TermCoord::from_game_coord(look_coord, zoom_coord) {
                // Checks if the cursor is inside the central module
                let is_inside_fov = term_coord.y > MOD_CENTRAL_POS.0
                    && term_coord.x > MOD_CENTRAL_POS.1
                    && term_coord.y <= (MOD_CENTRAL_POS.0 + Self::FOV_ROWS)
                    && term_coord.x <= (MOD_CENTRAL_POS.1 + Self::FOV_COLS);

                if is_inside_fov {
                    if look_coord.y % 2 == 0 {
                        new_frame[term_coord.y][term_coord.x - 1] = CURSOR_UP;
                    } else {
                        new_frame[term_coord.y][term_coord.x - 1] = CURSOR_DOWN;
                    }
                }
            }
        }

        // Only prints where the canvas has changed to avoid studdering
        for row in 0..CANVAS_ROWS {
            for col in 0..CANVAS_COLS {
                let new_cell = &new_frame[row][col];
                let last_cell = &self.prev_frame[row][col];
                if new_cell != last_cell {
                    // Move cursor and print changed cell
                    print!(
                        "\x1b[{};{}H{}",
                        self.canvas_pos.0 + row + 1,
                        self.canvas_pos.1 + col + 1,
                        new_cell.as_string()
                    );
                }
            }
        }
        print!("{}", ansi::RESET_COLOR);
        self.prev_frame = new_frame;
        self.render_count += 1;
    }
}
