//! # TUI Canvas
//!
//! This module defines the `Canvas`, which is responsible for composing and
//! rendering the different UI modules (central map, side panels, etc.) into
//! a single view in the terminal.

use std::collections::{HashMap, VecDeque};
use terminal_size::{terminal_size, Height, Width};

use crate::ansi;
use crate::assets;
use crate::r#const::*;
use common;

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
    left_module: LeftModule,
    right_module: RightModule,
    bottom_module: BottomModule,
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
        let central_module = canvas_modules::CentralModule::new();
        let left_module = canvas_modules::LeftModule::new();
        let right_module = canvas_modules::RightModule::new();
        let bottom_module = canvas_modules::BottomModule::new();
        Self {
            prev_frame,
            canvas_pos,
            render_count: 0,
            central_module,
            left_module,
            right_module,
            bottom_module,
        }
    }

    pub fn init(&mut self, tiles: Vec<Vec<common::TileE>>) {
        self.central_module.init(tiles);
    }

    /// Prints the entire canvas to the terminal.
    ///
    /// It gets the content from each module, assembles it into a buffer,
    /// and then prints the buffer to stdout.
    pub fn render(
        &mut self,
        game_objs: &HashMap<common::GameID, common::GameObjE>,
        player_data: &common::PlayerDataE,
        map_zoom: Option<(usize, usize)>,
        frame_dt: u64,
        logs: &mut VecDeque<String>,
    ) {
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        // TODO: refactor modules logic
        for (row, line_contents) in self.right_module.get_renderable_and_update(frame_dt).iter().enumerate() {
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

        for (row, line_contents) in self.left_module.get_coget_renderable_and_updatentent(player_data).iter().enumerate() {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + LEFT_MOD_POS.0][col + LEFT_MOD_POS.1] = cell.clone();
            }
        }

        for (row, line_contents) in self.bottom_module.get_get_renderable_and_updatecontent(&mut logs).iter().enumerate() {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + BOTTOM_MOD_POS.0][col + BOTTOM_MOD_POS.1] = cell.clone();
            }
        }

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

    pub fn update_and_print_cursor(&self, map_look: Option<(usize, usize)>) {
        if let Some((row, col)) = map_look {
            // Terminal coord are 1-indexed
            print!(
                "\r\x1b[{};{}H",
                crate::r#const::CENTRAL_MOD_POS.0 + row + self.canvas_pos.0 + 1,
                crate::r#const::CENTRAL_MOD_POS.1 + col + self.canvas_pos.1 + 1
            );
        } else {
            print!("\r\x1b[0;0H");
        }
    }
}
