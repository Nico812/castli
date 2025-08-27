//! # TUI Canvas
//!
//! This module defines the `Canvas`, which is responsible for composing and
//! rendering the different UI modules (central map, side panels, etc.) into
//! a single view in the terminal.

use std::collections::HashMap;
use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::canvas_modules;
use crate::r#const::*;
use common;

/// Represents the main drawing area for the TUI.
///
/// It holds all the different UI modules and is responsible for positioning
/// them correctly and printing them to the screen.
pub struct Canvas {
    last_frame: Vec<Vec<TermCell>>,
    canvas_pos: (usize, usize),
    central_module: canvas_modules::CentralModule,
    left_module: canvas_modules::LeftModule,
    right_module: canvas_modules::RightModule,
    bottom_module: canvas_modules::BottomModule,
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

        let central_module = canvas_modules::CentralModule::new();
        let left_module = canvas_modules::LeftModule::new();
        let right_module = canvas_modules::RightModule::new();
        let bottom_module = canvas_modules::BottomModule::new();
        Self {
            canvas_pos,
            central_module,
            left_module,
            right_module,
            bottom_module,
        }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.central_module.init(tiles);
    }

    /// Prints the entire canvas to the terminal.
    ///
    /// It gets the content from each module, assembles it into a buffer,
    /// and then prints the buffer to stdout.
    pub fn render(
        &self,
        game_objs: &HashMap<common::GameID, common::GameObjE>,
        player_data: &common::PlayerDataE,
        map_zoom: Option<(usize, usize)>,
    ) {
        let mut new_frame: Vec<Vec<TermCell>> = vec![vec![BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        // TODO: refactor modules logic
        for (row, line_contents) in self.right_module.get_content().iter().enumerate() {
            for (col, cell) in line_contents.chars().enumerate() {
                new_frame[row + RIGHT_MOD_POS.0][col + RIGHT_MOD_POS.1] = cell;
            }
        }

        for (row, line_contents) in self.central_module.get_content().iter().enumerate() {
            for (col, cell) in line_contents.chars().enumerate() {
                new_frame[row + CENTRAL_MOD_POS.0][col + CENTRAL_MOD_POS.1] = cell;
            }
        }

        for (row, line_contents) in self.left_module.get_content().iter().enumerate() {
            for (col, cell) in line_contents.chars().enumerate() {
                new_frame[row + LEFT_MOD_POS.0][col + LEFT_MOD_POS.1] = cell;
            }
        }

        for (row, line_contents) in self.bottom_module.get_content().iter().enumerate() {
            for (col, cell) in line_contents.chars().enumerate() {
                new_frame[row + BOTTOM_MOD_POS.0][col + BOTTOM_MOD_POS.1] = cell;
            }
        }
        
        // is the terminal by default iterable even if theres nothjing printed?
        //let top_margin = "\r\n".repeat(self.canvas_pos.0);
        //let left_term_margin = " ".repeat(self.canvas_pos.1);

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

        self.prev_frame = new_frame;
    }

    pub fn update_and_print_cursor(&self, map_look: Option<(usize, usize)>) {
        if let Some((row, col)) = map_look {
            // Terminal coord are 1-indexed
            // + 2 to account for that and the module frame
            print!(
                "\r\x1b[{};{}H",
                crate::r#const::CENTRAL_MOD_POS.0 + row + self.canvas_pos.0 + 2,
                crate::r#const::CENTRAL_MOD_POS.1 + col + self.canvas_pos.1 + 2
            );
        } else {
            print!("\r\x1b[0;0H");
        }
    }
}