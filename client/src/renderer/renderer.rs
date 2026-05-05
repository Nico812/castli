use std::io::Stdout;
use std::io::Write;

use common::map::Tile;
use crossterm::cursor;
use crossterm::queue;
use crossterm::style::PrintStyledContent;
use crossterm::terminal;

use crate::assets;
use crate::assets::CURSOR_DOWN;
use crate::assets::CURSOR_UP;
use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::renderer::r#const::*;
use crate::renderer::map_data::MapData;
use crate::renderer::mod_inspect::ModInspect;
use crate::renderer::mod_interact::ModInteract;
use crate::renderer::{mod_central::ModCentral, mod_right::ModRight};
use crate::ui_state::CameraLocation;
use crate::ui_state::UiMode;
use crate::ui_state::UiState;

pub struct Renderer {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    map_data: MapData,
    prev_is_night: bool,
}

impl Renderer {
    pub const FOV_ROWS: usize = MOD_CENTRAL_ROWS - 2;
    pub const FOV_COLS: usize = MOD_CENTRAL_COLS - 2;
    pub const ZOOM_FACTOR: usize = 8;

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
        self.map_data
            .update_wind(self.render_count, &ui_state.camera);

        // Right module
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        for (row, line_contents) in ModRight::update(frame_dt, game_state, ui_state)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + MOD_RIGHT_POS.0][col + MOD_RIGHT_POS.1] = *cell;
            }
        }

        // Central module
        for (row, line_contents) in ModCentral::update(game_state, ui_state, &self.map_data)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + MOD_CENTRAL_POS.0][col + MOD_CENTRAL_POS.1] = *cell;
            }
        }

        // Inspect module
        if let Some(renderable) = ModInspect::update(game_state, ui_state) {
            // TODO: Here pos_col should change based on look_coord
            let pos_row = MOD_CENTRAL_POS.0;
            let pos_col = MOD_CENTRAL_POS.1 + MOD_CENTRAL_COLS - MOD_INSPECT_COLS;

            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + pos_row][col + pos_col] = *cell;
                }
            }
        }

        //Interact module
        if let Some(renderable) = ModInteract::update(game_state, ui_state) {
            let pos_row = MOD_INTERACT_POS.0;
            let pos_col = MOD_INTERACT_POS.1;

            for (row, line_contents) in renderable.iter().enumerate() {
                for (col, cell) in line_contents.iter().enumerate() {
                    new_frame[row + pos_row][col + pos_col] = *cell;
                }
            }
        }

        // Adding the cursor
        if let UiMode::Inspect(ref inspect) = ui_state.mode
            && let Some(term_coord) =
                TermCoord::from_game_coord(inspect.coord, &ui_state.camera, false)
        {
            // Checks if the cursor is inside the central module
            let is_inside_fov = term_coord.y > MOD_CENTRAL_POS.0
                && term_coord.x > MOD_CENTRAL_POS.1
                && term_coord.y <= (MOD_CENTRAL_POS.0 + Self::FOV_ROWS)
                && term_coord.x <= (MOD_CENTRAL_POS.1 + Self::FOV_COLS);

            if is_inside_fov {
                let cursor_asset = match (ui_state.camera.location, inspect.coord.y) {
                    (CameraLocation::Map, y) | (CameraLocation::Courtyard, y) if y % 2 == 0 => {
                        CURSOR_UP
                    }
                    (CameraLocation::WorldMap, y) if y % (2 * Renderer::ZOOM_FACTOR) < 8 => {
                        CURSOR_UP
                    }
                    _ => CURSOR_DOWN,
                };
                new_frame[term_coord.y][term_coord.x - 1] = cursor_asset;
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
