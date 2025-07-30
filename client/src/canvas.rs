use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::canvas_modules;

pub struct Canvas {
    canvas_size: (usize, usize),
    central_module_pos: (usize, usize),
    left_module_pos: (usize, usize),
    right_module_pos: (usize, usize),
    bottom_module_pos: (usize, usize),
    central_module: canvas_modules::CentralModule,
    left_module: canvas_modules::LeftModule,
    right_module: canvas_modules::RightModule,
    bottom_module: canvas_modules::BottomModule,
}

impl Canvas {
    pub fn new() -> Self {
        let canvas_size;
        match terminal_size() {
            Some((Width(w), Height(h))) => canvas_size = (h as usize, w as usize),
            None => panic!(),
        }
        let central_module_pos = (
            2,
            (canvas_size.1 - canvas_modules::CENTRAL_MODULE_COLS - 2) / 2,
        );
        let left_module_pos = (2, 2);
        let right_module_pos = (
            2,
            central_module_pos.1 + canvas_modules::CENTRAL_MODULE_COLS + 4,
        );
        let bottom_module_pos = (
            central_module_pos.0 + canvas_modules::CENTRAL_MODULE_ROWS + 4,
            central_module_pos.1,
        );
        let central_module = canvas_modules::CentralModule::new();
        let left_module = canvas_modules::LeftModule::new();
        let right_module = canvas_modules::RightModule::new();
        let bottom_module = canvas_modules::BottomModule::new();
        Self {
            canvas_size,
            central_module_pos,
            left_module_pos,
            right_module_pos,
            bottom_module_pos,
            central_module,
            left_module,
            right_module,
            bottom_module,
        }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.central_module.init(tiles);
    }

    pub fn print(&self, structures: &Vec<common::StructureE>, map_zoom: Option<(usize, usize)>) {
        // PS: start by rendering the modules at the right
        // Positions debug
        print!("\n--- DEBUG POSIZIONI MODULI ---\n\r");
        print!(
            "CentralModule:   top-left=({}, {}), bottom-right=({}, {})\n\r",
            self.central_module_pos.0,
            self.central_module_pos.1,
            self.central_module_pos.0 + canvas_modules::CENTRAL_MODULE_ROWS,
            self.central_module_pos.1 + canvas_modules::CENTRAL_MODULE_COLS
        );
        print!(
            "LeftModule:      top-left=({}, {}), bottom-right=({}, {})\n\r",
            self.left_module_pos.0,
            self.left_module_pos.1,
            self.left_module_pos.0 + canvas_modules::LEFT_MODULE_ROWS,
            self.left_module_pos.1 + canvas_modules::LEFT_MODULE_COLS
        );
        print!(
            "RightModule:     top-left=({}, {}), bottom-right=({}, {})\n\r",
            self.right_module_pos.0,
            self.right_module_pos.1,
            self.right_module_pos.0 + canvas_modules::RIGHT_MODULE_ROWS,
            self.right_module_pos.1 + canvas_modules::RIGHT_MODULE_COLS
        );
        print!(
            "BottomModule:    top-left=({}, {}), bottom-right=({}, {})\n\r",
            self.bottom_module_pos.0,
            self.bottom_module_pos.1,
            self.bottom_module_pos.0 + canvas_modules::BOTTOM_MODULE_ROWS,
            self.bottom_module_pos.1 + canvas_modules::BOTTOM_MODULE_COLS
        );
        print!(
            "Canvas size:     rows={}, cols={}\n\r",
            self.canvas_size.0, self.canvas_size.1
        );
        print!("--- FINE DEBUG POSIZIONI ---\n\r");

        let mut buffer: Vec<String> = vec!["_".repeat(self.canvas_size.1); self.canvas_size.0];

        for (line, line_contents) in self.right_module.get_content().iter().enumerate() {
            buffer[line + self.right_module_pos.0].replace_range(
                self.right_module_pos.1
                    ..self.right_module_pos.1 + canvas_modules::RIGHT_MODULE_COLS,
                line_contents,
            );
        }

        for (line, line_contents) in self
            .central_module
            .get_map(structures, map_zoom)
            .iter()
            .enumerate()
        {
            let replacement = format!("{}{}", line_contents.concat(), ansi::RESET_COLOR!());

            buffer[line + self.central_module_pos.0].replace_range(
                self.central_module_pos.1
                    ..self.central_module_pos.1 + canvas_modules::CENTRAL_MODULE_COLS + 2,
                &replacement,
            );
        }
        for (line, line_contents) in self.left_module.get_content().iter().enumerate() {
            buffer[line + self.left_module_pos.0].replace_range(
                self.left_module_pos.1..self.left_module_pos.1 + canvas_modules::LEFT_MODULE_COLS,
                line_contents,
            );
        }

        for (line, line_contents) in self.bottom_module.get_content().iter().enumerate() {
            buffer[line + self.bottom_module_pos.0].replace_range(
                self.bottom_module_pos.1
                    ..self.bottom_module_pos.1 + canvas_modules::BOTTOM_MODULE_COLS,
                line_contents,
            );
        }

        let buffer_len = buffer.len();
        for (iter, line) in buffer.iter().enumerate() {
            print!("{}", line);
            if iter != buffer_len - 1 {
                print!("\r\n");
            };
        }
    }
}
