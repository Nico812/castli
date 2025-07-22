use crate::ansi;
use crate::canvas_modules;

pub const CANVAS_SIZE: (usize, usize) = (50, 160);
pub const CENTRAL_MODULE_POS: (usize, usize) = (1, 2);

pub struct Canvas {
    central_module: canvas_modules::CentralModule,
}

impl Canvas {
    pub fn new() -> Self {
        let central_module = canvas_modules::CentralModule::new();
        Self { central_module }
    }

    pub fn init(&mut self, tiles: &Vec<Vec<common::TileE>>) {
        self.central_module.init(tiles);
    }

    pub fn print(&self, structures: &Vec<common::StructureE>, map_zoom: Option<(usize, usize)>) {
        let mut buffer: Vec<String> = vec![".".repeat(CANVAS_SIZE.1); CANVAS_SIZE.0];

        for (line, line_contents) in self
            .central_module
            .get_map(structures, map_zoom)
            .iter()
            .enumerate()
        {
            let replacement = format!("{}{}", line_contents.concat(), ansi::RESET_COLOR);

            buffer[line + CENTRAL_MODULE_POS.0].replace_range(
                CENTRAL_MODULE_POS.1..CENTRAL_MODULE_POS.1 + canvas_modules::CENTRAL_MODULE_SIZE,
                &replacement,
            );
        }

        for line in buffer.iter() {
            print!("{}", line);
            print!("\r\n");
        }
    }
}
