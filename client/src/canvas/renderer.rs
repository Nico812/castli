use std::collections::{HashMap, VecDeque};

use common::exports::game_object::GameObjE;
use common::exports::player::PlayerE;
use common::exports::tile::TileE;
use common::{self, GameCoord, GameID};
use terminal_size::{Height, Width, terminal_size};

use crate::ansi;
use crate::assets;
use crate::assets::{CURSOR_DOWN, CURSOR_UP};
use crate::canvas::RightModuleTab;
use crate::canvas::r#const::*;
use crate::canvas::{central_module::CentralModule, right_module::RightModule};
use crate::coord::TermCoord;

pub struct Canvas {
    prev_frame: Vec<Vec<assets::TermCell>>,
    canvas_pos: (usize, usize),
    render_count: u32,
    central_module: CentralModule,
    right_module: RightModule,
}

impl Canvas {
    pub fn new() -> Self {
        let canvas_pos = if let Some((Width(w), Height(h))) = terminal_size() {
            if (h as usize) < CANVAS_ROWS || (w as usize) < CANVAS_COLS {
                println!("Terminal size is too small, consider changing your terminal text size.");
                (0, 0)
            } else {
                (
                    ((h as usize) - CANVAS_ROWS) / 2,
                    ((w as usize) - CANVAS_COLS) / 2,
                )
            }
        } else {
            println!("Could not detect the terminal size.");
            (0, 0)
        };

        Self {
            prev_frame: vec![vec![assets::ERR_EL; CANVAS_COLS]; CANVAS_ROWS],
            canvas_pos,
            render_count: 0,
            central_module: CentralModule::new(),
            right_module: RightModule::new(),
        }
    }

    pub fn init(&mut self, tiles: Vec<Vec<TileE>>) {
        self.central_module.init(tiles);
    }

    pub fn change_right_tab(&mut self, tab: RightModuleTab) {
        self.right_module.change_tab(tab);
    }

    pub fn render(
        &mut self,
        game_objs: &HashMap<GameID, GameObjE>,
        player_data: &PlayerE,
        map_zoom: Option<GameCoord>,
        map_look: Option<GameCoord>,
        frame_dt: u64,
        logs: &VecDeque<String>,
    ) {
        let mut new_frame: Vec<Vec<assets::TermCell>> =
            vec![vec![assets::BKG_EL; CANVAS_COLS]; CANVAS_ROWS];

        for (row, line_contents) in self
            .right_module
            .get_renderable_and_update(frame_dt, map_look, player_data, logs)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + RIGHT_MOD_POS.0][col + RIGHT_MOD_POS.1] = *cell;
            }
        }

        for (row, line_contents) in self
            .central_module
            .get_renderable_and_update(game_objs, map_zoom, self.render_count)
            .iter()
            .enumerate()
        {
            for (col, cell) in line_contents.iter().enumerate() {
                new_frame[row + CENTRAL_MOD_POS.0][col + CENTRAL_MOD_POS.1] = *cell;
            }
        }

        if let (Some(look_coord), Some(zoom_coord)) = (map_look, map_zoom)
            && let Some(term_coord) = TermCoord::from_game_coord(look_coord, zoom_coord)
        {
            let is_inside_fov = term_coord.y > CENTRAL_MOD_POS.0
                && term_coord.x > CENTRAL_MOD_POS.1
                && term_coord.y <= (CENTRAL_MOD_POS.0 + CENTRAL_MODULE_CONTENT_ROWS)
                && term_coord.x <= (CENTRAL_MOD_POS.1 + CENTRAL_MODULE_CONTENT_COLS);

            if is_inside_fov {
                if look_coord.y % 2 == 0 {
                    new_frame[term_coord.y][term_coord.x - 1] = CURSOR_UP;
                } else {
                    new_frame[term_coord.y][term_coord.x - 1] = CURSOR_DOWN;
                }
            }
        }

        for (row, (new_row, prev_row)) in new_frame.iter().zip(self.prev_frame.iter()).enumerate() {
            for (col, (new_cell, prev_cell)) in new_row.iter().zip(prev_row.iter()).enumerate() {
                if new_cell != prev_cell {
                    print!(
                        "\x1b[{};{}H{}",
                        self.canvas_pos.0 + row + 1,
                        self.canvas_pos.1 + col + 1,
                        new_cell.as_string()
                    );
                }
            }
        }
        print!("{}", ansi::RESET_COLOR);
        self.prev_frame = new_frame;
        self.render_count = self.render_count.wrapping_add(1);
    }
}
