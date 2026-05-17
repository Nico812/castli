use std::io::Stdout;
use std::io::Write;

use crossterm::cursor;
use crossterm::queue;
use crossterm::style::PrintStyledContent;
use crossterm::terminal;
use crossterm::terminal::Clear;

use crate::assets::BKG_EL;
use crate::assets::TermCell;
use crate::r#const::MOD_INSPECT_COLS;
use crate::r#const::MOD_INTERACT_COLS;
use crate::r#const::MOD_PLAYER_INFO_ROWS;
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::mod_inspect::ModInspect;
use crate::renderer::mod_interact::ModInteract;
use crate::renderer::module::Module;
use crate::renderer::{mod_central::ModCentral, mod_player_info::ModPlayerInfo};
use crate::ui_state::UiState;

pub struct Renderer {
    prev_frame: Vec<Vec<TermCell>>,
    render_count: u32,
    prev_is_night: bool,
    canvas_size: TermCoord,
    mod_central: ModCentral,
    mod_player_info: ModPlayerInfo,
    mod_inspect: ModInspect,
    mod_interact: ModInteract,
}

impl Renderer {
    pub const PADDING: TermCoord = TermCoord { y: 0, x: 0 };

    // TODO: use resize_modules, dont duplicate code.
    pub fn new() -> Result<Self, ()> {
        let terminal_size = if let Ok((w, h)) = terminal::size() {
            TermCoord::new(h as usize, w as usize)
        } else {
            return Err(());
        };

        let canvas_size = terminal_size - Self::PADDING * 2;

        let mod_padding = TermCoord::new(1, 2);
        let mod_player_info_size = TermCoord::new(MOD_PLAYER_INFO_ROWS, canvas_size.x);
        let mod_central_size = canvas_size - TermCoord::new(mod_player_info_size.y + 1, 0);
        let mod_inspect_size = TermCoord::new(0, MOD_INSPECT_COLS);
        let mod_interact_size = TermCoord::new(0, MOD_INTERACT_COLS);

        let mod_central = ModCentral::new(Module::new(mod_central_size, mod_padding));
        let mod_player_info = ModPlayerInfo::new(Module::new(mod_player_info_size, mod_padding));
        let mod_inspect = ModInspect::new(Module::new(mod_inspect_size, mod_padding));
        let mod_interact = ModInteract::new(Module::new(mod_interact_size, mod_padding));

        let prev_frame = vec![vec![TermCell::ERR; canvas_size.x]; canvas_size.y];

        Ok(Self {
            prev_is_night: false,
            prev_frame,
            render_count: 0,
            canvas_size,
            mod_central,
            mod_player_info,
            mod_inspect,
            mod_interact,
        })
    }

    fn resize_modules(&mut self) {
        let mod_padding = TermCoord::new(1, 2);
        let mod_player_info_size = TermCoord::new(MOD_PLAYER_INFO_ROWS, self.canvas_size.x);
        let mod_central_size = self.canvas_size - TermCoord::new(mod_player_info_size.y + 1, 0);
        let mod_inspect_size = TermCoord::new(0, MOD_INSPECT_COLS);
        let mod_interact_size = TermCoord::new(0, MOD_INTERACT_COLS);

        self.mod_central = ModCentral::new(Module::new(mod_central_size, mod_padding));
        self.mod_player_info = ModPlayerInfo::new(Module::new(mod_player_info_size, mod_padding));
        self.mod_inspect = ModInspect::new(Module::new(mod_inspect_size, mod_padding));
        self.mod_interact = ModInteract::new(Module::new(mod_interact_size, mod_padding));

        self.prev_frame = vec![vec![TermCell::ERR; self.canvas_size.x]; self.canvas_size.y];
    }

    pub fn render(
        &mut self,
        stdout: &mut Stdout,
        game_state: &mut GameState,
        ui_state: &mut UiState,
        frame_dt: u64,
    ) {
        if let Some(new_term_size) = ui_state.term_size_change {
            self.canvas_size = new_term_size - Self::PADDING * 2;
            self.resize_modules();
            ui_state.camera.zoom_factor = self.zoom_factor();
            let _ = queue!(stdout, Clear(terminal::ClearType::Purge));
            ui_state.term_size_change = None;
        };

        let mut new_frame: Vec<Vec<TermCell>> =
            vec![vec![BKG_EL; self.canvas_size.x]; self.canvas_size.y];

        for (row, line_contents) in self
            .mod_central
            .render(game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row)
                    .map(|frame_row| frame_row.get_mut(col))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        let mod_player_info_pos = TermCoord::new(self.canvas_size.y - MOD_PLAYER_INFO_ROWS, 0);
        for (row, line_contents) in self
            .mod_player_info
            .render(frame_dt, game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame
                    .get_mut(row + mod_player_info_pos.y)
                    .map(|frame_row| frame_row.get_mut(col + mod_player_info_pos.x))
                    .flatten()
                    .map(|frame_cell| *frame_cell = *cell);
            }
        }

        let mod_inspect_pos =
            TermCoord::new(3, self.canvas_size.x.saturating_sub(MOD_INSPECT_COLS + 5));
        if let Some(renderable) = self.mod_inspect.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame
                        .get_mut(row + mod_inspect_pos.y)
                        .map(|frame_row| frame_row.get_mut(col + mod_inspect_pos.x))
                        .flatten()
                        .map(|frame_cell| *frame_cell = *cell);
                }
            }
        }

        let mod_interact_pos = TermCoord::new(
            3,
            (self.canvas_size.x.saturating_sub(MOD_INTERACT_COLS)) / 2,
        );
        if let Some(renderable) = self.mod_interact.render(game_state, ui_state) {
            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame
                        .get_mut(row + mod_interact_pos.y)
                        .map(|frame_row| frame_row.get_mut(col + mod_interact_pos.x))
                        .flatten()
                        .map(|frame_cell| *frame_cell = *cell);
                }
            }
        }

        for row in 0..self.canvas_size.y {
            for col in 0..self.canvas_size.x {
                let new_cell = &new_frame[row][col];
                let last_cell = &self.prev_frame[row][col];
                if (new_cell != last_cell) || (self.prev_is_night != game_state.time.night) {
                    let x = (Self::PADDING.x + col) as u16;
                    let y = (Self::PADDING.y + row) as u16;
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
