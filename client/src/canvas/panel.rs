use std::collections::VecDeque;

use common::GameCoord;
use common::exports::player::PlayerE;
use common::exports::units::UnitE;

use super::cell::*;
use super::frame;
use super::layout::*;

#[derive(Copy, Clone)]
pub enum RightModuleTab {
    Castle,
    Debug,
    Logs,
}

pub struct RightModule {
    current_tab: RightModuleTab,
}

impl RightModule {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_ROWS: usize = RIGHT_MODULE_ROWS.saturating_sub(Self::PADDING_VERT * 2);
    const CONTENT_COLS: usize = RIGHT_MODULE_COLS.saturating_sub(Self::PADDING_HORI * 2);

    pub fn new() -> Self {
        Self {
            current_tab: RightModuleTab::Castle,
        }
    }

    pub fn render_into(
        &self,
        frame: &mut [Vec<TermCell>],
        frame_dt: u64,
        look_pos: Option<GameCoord>,
        player: &PlayerE,
        logs: &VecDeque<String>,
    ) {
        let content_top = RIGHT_MOD_POS.0 + 1;
        let content_left = RIGHT_MOD_POS.1 + 1;

        for row in 0..Self::CONTENT_ROWS {
            for col in 0..Self::CONTENT_COLS {
                frame[content_top + row][content_left + col] =
                    TermCell::new(' ', FG_BLACK, BG_BLACK);
            }
        }

        match self.current_tab {
            RightModuleTab::Castle => {
                Self::render_castle_tab(frame, content_top, content_left, player)
            }
            RightModuleTab::Debug => {
                Self::render_debug_tab(frame, content_top, content_left, frame_dt, look_pos)
            }
            RightModuleTab::Logs => Self::render_logs_tab(frame, content_top, content_left, logs),
        }

        frame::render_frame_into(
            frame,
            "inspect",
            RIGHT_MOD_POS,
            Self::CONTENT_ROWS,
            Self::CONTENT_COLS,
        );
    }

    pub fn change_tab(&mut self, new_tab: RightModuleTab) {
        self.current_tab = new_tab;
    }

    fn render_debug_tab(
        frame: &mut [Vec<TermCell>],
        top: usize,
        left: usize,
        frame_dt: u64,
        look_pos: Option<GameCoord>,
    ) {
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        frame::draw_text_row(
            frame,
            &dt_str,
            top + 1,
            left + Self::PADDING_HORI,
            Self::CONTENT_COLS - Self::PADDING_HORI * 2,
        );

        if let Some(pos) = look_pos {
            let look_str = format!("Looking at ({}, {})", pos.y, pos.x);
            frame::draw_text_row(
                frame,
                &look_str,
                top + 3,
                left + Self::PADDING_HORI,
                Self::CONTENT_COLS - Self::PADDING_HORI * 2,
            );
        }
    }

    fn render_castle_tab(frame: &mut [Vec<TermCell>], top: usize, left: usize, player: &PlayerE) {
        let pos_str = format!("({}, {})", player.pos.y, player.pos.x);
        let peasants_str = format!("Peasants: {}", player.peasants);
        let knights_str = format!(
            "Knights: {}",
            player.units.quantities[UnitE::Knight.as_index()]
        );
        let mages_str = format!("Mages: {}", player.units.quantities[UnitE::Mage.as_index()]);
        let dragons_str = format!(
            "Dragons: {}",
            player.units.quantities[UnitE::Dragon.as_index()]
        );

        let col = left + Self::PADDING_HORI;
        let max_w = Self::CONTENT_COLS - Self::PADDING_HORI * 2;

        let entries: [(&str, usize); 6] = [
            (&player.name, 3),
            (&pos_str, 5),
            (&peasants_str, 6),
            (&knights_str, 8),
            (&mages_str, 9),
            (&dragons_str, 10),
        ];

        for (text, row_offset) in entries {
            frame::draw_text_row(frame, text, top + row_offset, col, max_w);
        }
    }

    fn render_logs_tab(
        frame: &mut [Vec<TermCell>],
        top: usize,
        left: usize,
        logs: &VecDeque<String>,
    ) {
        let visible_count = logs.len().min(Self::CONTENT_ROWS);
        let start = logs.len().saturating_sub(Self::CONTENT_ROWS);
        let max_chars = Self::CONTENT_COLS - Self::PADDING_HORI;

        for (i, log) in logs.iter().skip(start).enumerate() {
            let frame_row = top + Self::CONTENT_ROWS - visible_count + i;
            for (j, ch) in log.chars().enumerate() {
                if j >= max_chars {
                    break;
                }
                frame[frame_row][left + Self::PADDING_HORI + j] =
                    TermCell::new(ch, FG_WHITE, BG_BLACK);
            }
        }
    }
}
