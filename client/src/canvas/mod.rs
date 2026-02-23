mod art;
mod cell;
mod coord;
mod frame;
mod layout;
mod map;
mod panel;

pub use panel::RightModuleTab;

use std::collections::{HashMap, VecDeque};
use std::fmt::Write as _;
use std::io::Write;
use std::time::Instant;

use crate::logger;
use common::exports::game_object::GameObjE;
use common::exports::player::PlayerE;
use common::exports::tile::TileE;
use common::{GameCoord, GameID};
use terminal_size::{Height, Width, terminal_size};

use art::{BKG_EL, CURSOR_DOWN, CURSOR_UP, ERR_EL};
use cell::{RESET_COLOR, TermCell};
use coord::TermCoord;
use layout::*;
use map::CentralModule;
use panel::RightModule;

pub struct Canvas {
    frames: [Vec<Vec<TermCell>>; 2],
    current: usize,
    write_buf: String,
    canvas_pos: (usize, usize),
    render_count: u32,
    central_module: CentralModule,
    right_module: RightModule,
}

impl Canvas {
    pub fn new() -> Self {
        let canvas_pos = if let Some((Width(w), Height(h))) = terminal_size() {
            if (h as usize) < CANVAS_ROWS || (w as usize) < CANVAS_COLS {
                eprintln!(
                    "Terminal too small (need {}x{}, got {}x{})",
                    CANVAS_COLS, CANVAS_ROWS, w, h
                );
                (0, 0)
            } else {
                (
                    ((h as usize) - CANVAS_ROWS) / 2,
                    ((w as usize) - CANVAS_COLS) / 2,
                )
            }
        } else {
            eprintln!("Could not detect terminal size");
            (0, 0)
        };

        logger::write(format_args!(
            "canvas: initialized at pos ({}, {}), size {}x{}",
            canvas_pos.0, canvas_pos.1, CANVAS_COLS, CANVAS_ROWS
        ));

        Self {
            frames: [
                vec![vec![ERR_EL; CANVAS_COLS]; CANVAS_ROWS],
                vec![vec![ERR_EL; CANVAS_COLS]; CANVAS_ROWS],
            ],
            current: 0,
            write_buf: String::with_capacity(CANVAS_ROWS * CANVAS_COLS * 16),
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
        let Self {
            frames,
            current,
            write_buf,
            canvas_pos,
            render_count,
            central_module,
            right_module,
        } = self;

        let render_start = Instant::now();

        let cur = *current;
        let prev = 1 - cur;

        for row in frames[cur].iter_mut() {
            for cell in row.iter_mut() {
                *cell = BKG_EL;
            }
        }

        right_module.render_into(&mut frames[cur], frame_dt, map_look, player_data, logs);
        central_module.render_into(&mut frames[cur], game_objs, map_zoom, *render_count);

        if let (Some(look_coord), Some(zoom_coord)) = (map_look, map_zoom)
            && let Some(term_coord) = TermCoord::from_game_coord(look_coord, zoom_coord)
        {
            let is_inside_fov = term_coord.y > CENTRAL_MOD_POS.0
                && term_coord.x > CENTRAL_MOD_POS.1
                && term_coord.y <= (CENTRAL_MOD_POS.0 + CENTRAL_MODULE_CONTENT_ROWS)
                && term_coord.x <= (CENTRAL_MOD_POS.1 + CENTRAL_MODULE_CONTENT_COLS);

            if is_inside_fov {
                let cursor = if look_coord.y % 2 == 0 {
                    CURSOR_UP
                } else {
                    CURSOR_DOWN
                };
                frames[cur][term_coord.y][term_coord.x - 1] = cursor;
            }
        }

        write_buf.clear();
        for (row, (new_row, prev_row)) in frames[cur].iter().zip(frames[prev].iter()).enumerate() {
            for (col, (new_cell, prev_cell)) in new_row.iter().zip(prev_row.iter()).enumerate() {
                if new_cell != prev_cell {
                    let _ = write!(
                        write_buf,
                        "\x1b[{};{}H",
                        canvas_pos.0 + row + 1,
                        canvas_pos.1 + col + 1,
                    );
                    new_cell.write_to(write_buf);
                }
            }
        }
        write_buf.push_str(RESET_COLOR);

        let mut stdout = std::io::stdout().lock();
        let _ = stdout.write_all(write_buf.as_bytes());
        let _ = stdout.flush();

        *current = prev;
        *render_count = render_count.wrapping_add(1);

        if *render_count % 300 == 0 {
            let render_us = render_start.elapsed().as_micros();
            logger::write(format_args!(
                "canvas: frame #{render_count} rendered in {render_us}us, buf={}B",
                write_buf.len()
            ));
        }
    }
}
