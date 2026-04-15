use std::collections::VecDeque;

use common::GameCoord;
use common::exports::player::PlayerE;
use common::exports::units::UnitE;

use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;
use crate::game_renderer::ModRightTab;
use crate::game_renderer::module_utility::draw_text_in_row;
use crate::tui::SharedState;

pub struct ModRight {}

impl ModRight {
    const PADDING_HORI: usize = 1;
    const PADDING_VERT: usize = 2;
    const CONTENT_ROWS: usize = MOD_RIGHT_ROWS.saturating_sub(2);
    const CONTENT_COLS: usize = MOD_RIGHT_COLS.saturating_sub(2);

    pub fn get_renderable(frame_dt: u64, state: &mut SharedState) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            Self::CONTENT_ROWS
        ];

        match state.mod_right_tab {
            ModRightTab::Castle => Self::add_castle_tab(&mut content, &state.player_data),
            ModRightTab::Debug => Self::add_debug_tab(&mut content, frame_dt, state.map_look),
            ModRightTab::Logs => Self::add_logs_tab(&mut content, &mut state.chat),
        };
        module_utility::add_frame("(1): me | (2): logs | (3): debug", &mut content);
        content
    }

    fn add_debug_tab(content: &mut Vec<Vec<TermCell>>, frame_dt: u64, look_pos: Option<GameCoord>) {
        // Show FPS
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        module_utility::draw_text_in_row(
            content,
            &dt_str,
            1,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );

        // Show looking coordinates
        if let Some(pos) = look_pos {
            let look_pos_str = format!("Looking at {}", pos);
            module_utility::draw_text_in_row(
                content,
                &look_pos_str,
                3,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
        }
    }

    fn add_castle_tab(content: &mut Vec<Vec<TermCell>>, player: &PlayerE) {
        let pos_str = format!("{}", player.pos);
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

        let infos_to_print = [
            (&player.name, 3),
            (&pos_str, 5),
            (&peasants_str, 6),
            (&knights_str, 8),
            (&mages_str, 9),
            (&dragons_str, 10),
        ];

        for info_to_print in infos_to_print {
            draw_text_in_row(
                content,
                info_to_print.0,
                info_to_print.1,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
        }
    }

    pub fn add_logs_tab(content: &mut Vec<Vec<TermCell>>, logs: &mut VecDeque<String>) {
        let mut chatbox: VecDeque<Vec<TermCell>> = VecDeque::with_capacity(Self::CONTENT_ROWS);

        let available_width = Self::CONTENT_COLS - (Self::PADDING_HORI * 2);
        let available_height = Self::CONTENT_ROWS - (Self::PADDING_VERT * 2);
        let tab_width = 4;
        let continuation_start = Self::PADDING_HORI + tab_width;
        let continuation_width = Self::CONTENT_COLS - continuation_start - Self::PADDING_HORI;

        // First, expand all logs into individual lines (oldest first)
        let mut all_lines: Vec<String> = Vec::new();

        for log in logs.iter() {
            if log.len() <= available_width {
                all_lines.push(log.clone());
            } else {
                let mut chars: Vec<char> = log.chars().collect();
                let mut pos = 0;
                let mut is_first_line = true;

                while pos < chars.len() {
                    let end = if is_first_line {
                        (pos + available_width).min(chars.len())
                    } else {
                        (pos + continuation_width).min(chars.len())
                    };

                    let mut line = String::with_capacity(Self::CONTENT_COLS);
                    if !is_first_line {
                        line.push_str(&" ".repeat(tab_width));
                    }
                    line.extend(&chars[pos..end]);
                    all_lines.push(line);

                    pos = end;
                    is_first_line = false;
                }
            }
        }

        // Take only the most recent lines that fit (drop oldest from the beginning)
        let start_index = if all_lines.len() > available_height {
            all_lines.len() - available_height
        } else {
            0
        };

        let recent_lines = &all_lines[start_index..];

        // Start with top padding
        for _ in 0..Self::PADDING_VERT {
            let empty_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            chatbox.push_back(empty_row);
        }

        // Add the content rows (oldest to newest from top to bottom)
        for line in recent_lines {
            let mut row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];

            for (i, ch) in line.chars().enumerate() {
                let col_pos = Self::PADDING_HORI + i;
                if col_pos < Self::CONTENT_COLS - Self::PADDING_HORI {
                    row[col_pos] = TermCell::new(ch, FG_WHITE, BG_BLACK);
                }
            }
            chatbox.push_back(row);
        }

        // Fill remaining space with bottom padding
        while chatbox.len() < Self::CONTENT_ROWS {
            let empty_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            chatbox.push_back(empty_row);
        }

        *content = chatbox.into();
    }
}
