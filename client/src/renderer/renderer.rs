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
use crate::renderer::{mod_central::ModCentral, mod_player_info::ModPlayerInfo};
use crate::ui_state::UiState;

pub struct Renderer {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    map_data: MapData,
    prev_is_night: bool,

    mod_central: ModCentral,
    mod_player_info: ModPlayerInfo,
    mod_inspect: ModInspect,
    mod_interact: ModInteract,
}

impl Renderer {
    pub fn new(map_tiles: Vec<Vec<Tile>>) -> Result<Self, ()> {
        let canvas_pos = if let Ok((w, h)) = terminal::size()
            && (h as usize) >= CANVAS_ROWS
            && (w as usize) >= CANVAS_COLS
        {
            (
                ((h as usize) - CANVAS_ROWS) / 2 - 1,
                ((w as usize) - CANVAS_COLS) / 2 - 1,
            )
        } else {
            return Err(());
        };

        let prev_frame = vec![vec![assets::TermCell::ERR; CANVAS_COLS]; CANVAS_ROWS];

        let mod_central = ModCentral::new(Module::new(
            TermCoord::new(MOD_CENTRAL_ROWS, MOD_CENTRAL_COLS),
            TermCoord::new(1, 2),
        ));
        let map_data = MapData::new(map_tiles, mod_central.zoom_factor());

        let mod_player_info = ModPlayerInfo::new(Module::new(
            TermCoord::new(MOD_PLAYER_INFO_ROWS, MOD_PLAYER_INFO_COLS),
            TermCoord::new(1, 2),
        ));

        let mod_inspect = ModInspect::new(Module::new(
            TermCoord::new(0, MOD_INSPECT_COLS),
            TermCoord::new(1, 2),
        ));

        let mod_interact = ModInteract::new(Module::new(
            TermCoord::new(0, MOD_INTERACT_COLS),
            TermCoord::new(1, 2),
        ));

        Ok(Self {
            prev_is_night: false,
            prev_frame,
            canvas_pos,
            render_count: 0,
            map_data,
            mod_central,
            mod_player_info,
            mod_inspect,
            mod_interact,
        })
    }

    pub fn render(
        &mut self,
        stdout: &mut Stdout,
        game_state: &mut GameState,
        ui_state: &UiState,
        frame_dt: u64,
    ) {
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        for (row, line_contents) in self
            .mod_central
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

        for (row, line_contents) in self
            .mod_player_info
            .render(frame_dt, game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + MOD_PLAYER_INFO_POS.0)
                    .map(|frame_row| frame_row.get_mut(col + MOD_PLAYER_INFO_POS.1))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        if let Some(renderable) = self.mod_inspect.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame
                        .get_mut(row + MOD_INSPECT_POS.0)
                        .map(|frame_row| frame_row.get_mut(col + MOD_INSPECT_POS.1))
                        .flatten()
                        .map(|frame_cell| *frame_cell = *cell);
                }
            }
        }

        if let Some(renderable) = self.mod_interact.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame
                        .get_mut(row + MOD_INTERACT_POS.0)
                        .map(|frame_row| frame_row.get_mut(col + MOD_INTERACT_POS.1))
                        .flatten()
                        .map(|frame_cell| *frame_cell = *cell);
                }
            }
        }

        for row in 0..CANVAS_ROWS {
            for col in 0..CANVAS_COLS {
                let new_cell = &new_frame[row][col];
                let last_cell = &self.prev_frame[row][col];
                if (new_cell != last_cell) || (self.prev_is_night != game_state.time.night) {
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

        let _ = stdout.flush();
    }

    pub fn fov_size(&self) -> TermCoord {
        self.mod_central.fov_size()
    }

    pub fn zoom_factor(&self) -> usize {
        self.mod_central.zoom_factor()
    }
}
