use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::canvas_modules;

pub struct Canvas {
    canvas_size: (usize, usize),
    central_module_pos: (usize, usize),
    central_module: canvas_modules::CentralModule,
}

impl Canvas {
    pub fn new() -> Self {
        let canvas_size;
        match terminal_size() {
            Some((Width(w), Height(h))) => canvas_size = (h as usize, w as usize),
            None => panic!(),
        }
        println!("{}{}", canvas_size.0, canvas_size.1);
        let central_module_pos = (
            (canvas_size.0 - canvas_modules::CENTRAL_MODULE_SIZE / 2) / 2,
            (canvas_size.1 - canvas_modules::CENTRAL_MODULE_SIZE) / 2,
        );
        let central_module = canvas_modules::CentralModule::new();
        Self {
            canvas_size,
            central_module_pos,
            central_module,
        }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.central_module.init(tiles);
    }

    pub fn print(&self, structures: &Vec<common::StructureE>, map_zoom: Option<(usize, usize)>) {
        let mut buffer: Vec<String> = vec![".".repeat(self.canvas_size.1); self.canvas_size.0];

        for (line, line_contents) in self
            .central_module
            .get_map(structures, map_zoom)
            .iter()
            .enumerate()
        {
            let replacement = format!("{}{}", line_contents.concat(), ansi::RESET_COLOR);

            buffer[line + self.central_module_pos.0].replace_range(
                self.central_module_pos.1
                    ..self.central_module_pos.1 + canvas_modules::CENTRAL_MODULE_SIZE,
                &replacement,
            );
        }

        for line in buffer.iter() {
            print!("{}", line);
            print!("\r\n");
        }
    }
}
