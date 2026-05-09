use std::io::Stdout;
use std::io::Write;

use common::map::Tile;
use crossterm::cursor;
use crossterm::queue;
use crossterm::style::PrintStyledContent;
use crossterm::terminal;

use crate::assets;
use crate::config::{UiConfig, config};
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::map_data::MapData;
use crate::renderer::mod_inspect::ModInspect;
use crate::renderer::mod_interact::ModInteract;
use crate::renderer::module::Module;
use crate::renderer::{mod_central::ModCentral, mod_player_info::ModPlayerInfo};
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
        let ui = &config().ui;
        let canvas_pos = if let Ok((w, h)) = terminal::size()
            && (h as usize) >= ui.canvas_rows
            && (w as usize) >= ui.canvas_cols
        {
            (
                ((h as usize) - ui.canvas_rows) / 2,
                ((w as usize) - ui.canvas_cols) / 2,
            )
        } else {
            return Err(());
        };

        let prev_frame = vec![vec![assets::TermCell::ERR; ui.canvas_cols]; ui.canvas_rows];
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
        let ui = &config().ui;

        // Right module
        let mut mod_player_info = ModPlayerInfo::new(Module::new(
            TermCoord::new(ui.mod_player_info_rows(), ui.mod_player_info_cols()),
            TermCoord::new(1, 2),
        ));
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; ui.canvas_cols]; ui.canvas_rows];

        let player_info_pos = ui.mod_player_info_pos();
        for (row, line_contents) in mod_player_info
            .render(frame_dt, game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + player_info_pos.0)
                    .map(|frame_row| frame_row.get_mut(col + player_info_pos.1))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        // Central module
        let mut mod_central = ModCentral::new(Module::new(
            TermCoord::new(ui.mod_central_rows, ui.mod_central_cols()),
            TermCoord::new(
                (ui.mod_central_rows - ui.fov_rows()) / 2 - ui.frame_width,
                (ui.mod_central_cols() - ui.fov_cols()) / 2 - ui.frame_width,
            ),
        ));
        for (row, line_contents) in mod_central
            .render(game_state, ui_state, &self.map_data)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + UiConfig::MOD_CENTRAL_POS.0)
                    .map(|frame_row| frame_row.get_mut(col + UiConfig::MOD_CENTRAL_POS.1))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        // Inspect module
        let mut mod_inspect = ModInspect::new(Module::new(
            TermCoord::new(0, ui.mod_inspect_cols),
            TermCoord::new(1, 2),
        ));
        let inspect_pos = ui.mod_inspect_pos();
        if let Some(renderable) = mod_inspect.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + inspect_pos.0][col + inspect_pos.1] = *cell;
                }
            }
        }

        //Interact module
        let mut mod_interact = ModInteract::new(Module::new(
            TermCoord::new(0, ui.mod_interact_cols()),
            TermCoord::new(1, 2),
        ));
        if let Some(renderable) = mod_interact.render(game_state, ui_state) {
            let interact_pos = ui.mod_interact_pos();
            let pos_row = interact_pos.0;
            let pos_col = interact_pos.1;

            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + pos_row][col + pos_col] = *cell;
                }
            }
        }

        // Only prints where the canvas has changed to avoid studdering
        for row in 0..ui.canvas_rows {
            for col in 0..ui.canvas_cols {
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
