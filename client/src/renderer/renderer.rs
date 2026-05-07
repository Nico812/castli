use std::io::Stdout;
use std::io::Write;

use common::map::Tile;
use crossterm::cursor;
use crossterm::queue;
use crossterm::style::PrintStyledContent;
use crossterm::terminal;

use crate::assets;
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::r#const::*;
use crate::renderer::map_data::MapData;
use crate::renderer::mod_inspect::ModInspect;
use crate::renderer::mod_interact::ModInteract;
use crate::renderer::module::Module;
use crate::renderer::{mod_central::ModCentral, mod_right::ModRight};
use crate::ui_state::UiState;

pub struct Renderer {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    map_data: MapData,
    prev_is_night: bool,
}

impl Renderer {
    pub fn new(map_tiles: Vec<Vec<Tile>>) -> Result<Self, ()> {
        let canvas_pos = if let Ok((w, h)) = terminal::size()
            && (h as usize) >= CANVAS_ROWS
            && (w as usize) >= CANVAS_COLS
        {
            (
                ((h as usize) - CANVAS_ROWS) / 2,
                ((w as usize) - CANVAS_COLS) / 2,
            )
        } else {
            return Err(());
        };

        let prev_frame = vec![vec![assets::TermCell::ERR; CANVAS_COLS]; CANVAS_ROWS];
        let map_data = MapData::new(map_tiles);

        Ok(Self {
            prev_is_night: false,
            prev_frame,
            canvas_pos,
            render_count: 0,
            map_data,
        })
    }

    pub fn render(
        &mut self,
        stdout: &mut Stdout,
        game_state: &mut GameState,
        ui_state: &UiState,
        frame_dt: u64,
    ) {
        // Right module
        let mut mod_right = ModRight::new(Module::new(
            TermCoord::new(MOD_RIGHT_ROWS, MOD_RIGHT_COLS),
            TermCoord::new(1, 2),
        ));
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        for (row, line_contents) in mod_right
            .render(frame_dt, game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + MOD_RIGHT_POS.0)
                    .map(|frame_row| frame_row.get_mut(col + MOD_RIGHT_POS.1))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        // Central module
        let mut mod_central = ModCentral::new(Module::new(
            TermCoord::new(MOD_CENTRAL_ROWS, MOD_CENTRAL_COLS),
            TermCoord::new(
                (MOD_CENTRAL_ROWS - FOV_ROWS) / 2 - FRAME_WIDTH,
                (MOD_CENTRAL_COLS - FOV_COLS) / 2 - FRAME_WIDTH,
            ),
        ));
        for (row, line_contents) in mod_central
            .render(game_state, ui_state, &self.map_data)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + MOD_CENTRAL_POS.0)
                    .map(|frame_row| frame_row.get_mut(col + MOD_CENTRAL_POS.1))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        // Inspect module
        let mut mod_inspect = ModInspect::new(Module::new(
            TermCoord::new(0, MOD_INSPECT_COLS),
            TermCoord::new(1, 2),
        ));
        if let Some(renderable) = mod_inspect.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + MOD_INSPECT_POS.0][col + MOD_INSPECT_POS.1] = *cell;
                }
            }
        }

        //Interact module
        let mut mod_interact = ModInteract::new(Module::new(
            TermCoord::new(0, MOD_INTERACT_COLS),
            TermCoord::new(1, 2),
        ));
        if let Some(renderable) = mod_interact.render(game_state, ui_state) {
            let pos_row = MOD_INTERACT_POS.0;
            let pos_col = MOD_INTERACT_POS.1;

            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + pos_row][col + pos_col] = *cell;
                }
            }
        }

        // Only prints where the canvas has changed to avoid studdering
        for row in 0..CANVAS_ROWS {
            for col in 0..CANVAS_COLS {
                let new_cell = &new_frame[row][col];
                let last_cell = &self.prev_frame[row][col];
                if (new_cell != last_cell) || (self.prev_is_night != game_state.time.night) {
                    // Move cursor and print changed cell
                    let x = (self.canvas_pos.1 + col + 1) as u16;
                    let y = (self.canvas_pos.0 + row + 1) as u16;
                    let _ = queue!(
                        stdout,
                        cursor::MoveTo(x, y),
                        PrintStyledContent(new_cell.printable())
                    );
                }
            }
        }

        self.prev_is_night = game_state.time.night;
        self.prev_frame = new_frame;
        self.render_count += 1;

        // All the commands are executed and tha changes printed now
        let _ = stdout.flush();
    }
}
