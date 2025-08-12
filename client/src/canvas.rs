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

/// Represents the main drawing area for the TUI.
///
/// It holds all the different UI modules and is responsible for positioning
/// them correctly and printing them to the screen.
pub struct Canvas {
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
    pub fn print(
        &self,
        game_objs: &HashMap<common::ID, common::GameObjE>,
        player_data: &common::PlayerDataE,
        map_zoom: Option<(usize, usize)>,
    ) {
        // PS: start by rendering the modules at the right
        let mut buffer: Vec<String> = vec!["_".repeat(CANVAS_COLS); CANVAS_ROWS];

        for (line, line_contents) in self.right_module.get_content().iter().enumerate() {
            buffer[line + RIGHT_MOD_POS.0].replace_range(
                RIGHT_MOD_POS.1..RIGHT_MOD_POS.1 + RIGHT_MODULE_COLS,
                line_contents,
            );
        }

        for (line, line_contents) in self
            .central_module
            .get_map(&game_objs, map_zoom)
            .iter()
            .enumerate()
        {
            let replacement = format!("{}{}", line_contents.concat(), ansi::RESET_COLOR!());

            buffer[line + CENTRAL_MOD_POS.0].replace_range(
                CENTRAL_MOD_POS.1..CENTRAL_MOD_POS.1 + CENTRAL_MODULE_COLS + 2,
                &replacement,
            );
        }
        for (line, line_contents) in self.left_module.get_content(&player_data).iter().enumerate() {
            buffer[line + LEFT_MOD_POS.0].replace_range(
                LEFT_MOD_POS.1..LEFT_MOD_POS.1 + LEFT_MODULE_COLS,
                line_contents,
            );
        }

        for (line, line_contents) in self.bottom_module.get_content().iter().enumerate() {
            buffer[line + BOTTOM_MOD_POS.0].replace_range(
                BOTTOM_MOD_POS.1..BOTTOM_MOD_POS.1 + BOTTOM_MODULE_COLS,
                line_contents,
            );
        }

        let buffer_len = buffer.len();
        let top_margin = "\r\n".repeat(self.canvas_pos.0);
        let left_term_margin = " ".repeat(self.canvas_pos.1);

        print!("{}", top_margin);
        for (iter, line) in buffer.iter().enumerate() {
            print!("{}{}", left_term_margin, line);
            if iter != buffer_len - 1 {
                print!("\r\n");
            };
        }
    }
}
