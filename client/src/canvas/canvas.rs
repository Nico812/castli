//! # TUI Canvas
//!
//! This module defines the `Canvas`, which is responsible for composing and
//! rendering the different UI modules (central map, side panels, etc.) into
//! a single view in the terminal.

use common::GameCoord;
use common::exports::game_object::GameObjE;
use common::exports::player::PlayerE;
use common::exports::tile::TileE;
use std::collections::{HashMap, VecDeque};
use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::assets;
use crate::assets::CURSOR_DOWN;
use crate::assets::CURSOR_UP;
use crate::canvas::r#const::*;
use crate::canvas::{central_module::CentralModule, right_module::RightModule};
use crate::coord::TermCoord;
use common::{self, GameID};

/// Represents the main drawing area for the TUI.
///
/// It holds all the different UI modules and is responsible for positioning
/// them correctly and printing them to the screen.
pub struct Canvas {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    // Modules
    central_module: CentralModule,
    right_module: RightModule,
}

impl Canvas {
    pub fn new() -> Self {
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
        let prev_frame = vec![vec![assets::ERR_EL; CANVAS_COLS]; CANVAS_ROWS];
        let central_module = CentralModule::new();
        let right_module = RightModule::new();
        Self {
            prev_frame,
            canvas_pos,
            render_count: 0,
            central_module,
            right_module,
        }
    }

    pub fn init(&mut self, tiles: Vec<Vec<TileE>>) {
        self.central_module.init(tiles);
    }

    /// Prints the entire canvas to the terminal.
    ///
    /// It gets the content from each module, assembles it into a buffer,
    /// and then prints the buffer to stdout.
    pub fn render(
        &mut self,
        game_objs: &HashMap<GameID, GameObjE>,
        player_data: &PlayerE,
        map_zoom: Option<GameCoord>,
        map_look: Option<GameCoord>,
        frame_dt: u64,
        logs: &mut VecDeque<String>,
        sel_obj: Option<(GameCoord, Option<GameID>)>,
    ) {
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        // TODO: refactor modules logic

        let mut selected_obj = None;
        let mut selected_pos = None;
        if let Some((pos, id)) = sel_obj {
            selected_pos = Some(pos);
            if let Some(selected_id) = id {
                selected_obj = Some(&game_objs[&selected_id]);
            }
        }

        for (row, line_contents) in self
            .right_module
            .get_renderable_and_update(frame_dt, selected_pos)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + RIGHT_MOD_POS.0][col + RIGHT_MOD_POS.1] = cell.clone();
            }
        }

        for (row, line_contents) in self
            .central_module
            .get_renderable_and_update(game_objs, map_zoom, self.render_count)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + CENTRAL_MOD_POS.0][col + CENTRAL_MOD_POS.1] = cell.clone();
            }
        }

        // Adding the cursor
        if let (Some(look_coord), Some(zoom_coord)) = (map_look, map_zoom) {
            if let Some(term_coord) = TermCoord::from_game_coord(look_coord, zoom_coord) {
                // Checks if the cursor is inside the central module
                let is_inside_fov = term_coord.y > CENTRAL_MOD_POS.0
                    && term_coord.x > CENTRAL_MOD_POS.1
                    && term_coord.y <= (CENTRAL_MOD_POS.0 + CENTRAL_MODULE_CONTENT_ROWS)
                    && term_coord.x <= (CENTRAL_MOD_POS.1 + CENTRAL_MODULE_CONTENT_COLS);

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
