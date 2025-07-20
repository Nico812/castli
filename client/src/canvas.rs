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

    pub fn update_map(
        &mut self,
        tiles: &Vec<Vec<common::TileE>>,
        map_zoom: Option<(usize, usize)>,
    ) {
        match map_zoom {
            Some(quadrant) => self.central_module.set_map_zoomed(tiles, quadrant),
            None => self.central_module.set_map(),
        }
    }

    pub fn update_structures(
        &mut self,
        structures: &Vec<common::StructureE>,
        map_zoom: Option<(usize, usize)>,
    ) {
        match map_zoom {
            Some(quadrant) => self
                .central_module
                .set_strutures_zoomed(structures, quadrant),
            None => self.central_module.set_strutures(structures),
        }
    }

    pub fn print(&self) {
        let mut buffer: Vec<String> = vec![".".repeat(CANVAS_SIZE.1); CANVAS_SIZE.0];

        for (line, line_contents) in self.central_module.content.iter().enumerate() {
            buffer[line + CENTRAL_MODULE_POS.0].replace_range(
                CENTRAL_MODULE_POS.1..CENTRAL_MODULE_POS.1 + canvas_modules::CENTRAL_MODULE_SIZE,
                &line_contents.concat(),
            );
        }

        for line in buffer.iter() {
            print!("{}", line);
            print!("\r\n");
        }
    }
}
