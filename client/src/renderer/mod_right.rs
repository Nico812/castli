use std::collections::VecDeque;

use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;
use crate::game_state::GameState;
use crate::game_state::Logs;
use crate::renderer::ModRightTab;
use crate::renderer::module_utility::draw_text_in_row;
use crate::ui_state::UiState;
use common::exports::client::ClientE;
use common::exports::owned_castle::OwnedCastleE;
use common::exports::units::UnitType;

pub struct ModRight {}

impl ModRight {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_ROWS: usize = MOD_RIGHT_ROWS.saturating_sub(2);
    const CONTENT_COLS: usize = MOD_RIGHT_COLS.saturating_sub(2);

    pub fn update(frame_dt: u64, game_state: &GameState, ui_state: &UiState) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            Self::CONTENT_ROWS
        ];

        match ui_state.tab {
            ModRightTab::Castle => Self::add_castle_tab(&mut content, &game_state.castle),
            ModRightTab::Debug => Self::add_debug_tab(&mut content, frame_dt, &game_state.client),
            ModRightTab::Logs => Self::add_logs_tab(&mut content, &game_state.logs),
        };
        module_utility::add_frame("(y): me | (x): logs | (c): debug", &mut content);
        content
    }

    fn add_debug_tab(content: &mut [Vec<TermCell>], frame_dt: u64, client: &ClientE) {
        let lobby_str = format!("Lobby {}", client.lobby);
        module_utility::draw_text_in_row(
            content,
            &lobby_str,
            Self::PADDING_VERT,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );
        let id_str = format!("Castle ID: {:?}", client.castle_id);
        module_utility::draw_text_in_row(
            content,
            &id_str,
            Self::PADDING_VERT + 1,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );
        // Show FPS
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        module_utility::draw_text_in_row(
            content,
            &dt_str,
            Self::PADDING_VERT + 2,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );
    }

    fn add_castle_tab(content: &mut [Vec<TermCell>], castle: &Option<OwnedCastleE>) {
        let Some(castle) = castle else {
            draw_text_in_row(
                content,
                "Welcome to Castli!",
                Self::PADDING_VERT,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            draw_text_in_row(
                content,
                "Press \"l\" to move",
                Self::PADDING_VERT + 2,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            draw_text_in_row(
                content,
                "Press \"a\" to create your castle :)",
                Self::PADDING_VERT + 3,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            return;
        };

        let alive_str = if castle.alive { "Alive :)" } else { "Dead x|" };
        let pos_str = format!("{}", castle.pos);
        let peasants_str = format!("Peasants: {}", castle.peasants);
        let knights_str = format!(
            "Knights: {}",
            castle.units.quantities[UnitType::Knight.as_index()]
        );
        let mages_str = format!(
            "Mages: {}",
            castle.units.quantities[UnitType::Mage.as_index()]
        );
        let dragons_str = format!(
            "Dragons: {}",
            castle.units.quantities[UnitType::Dragon.as_index()]
        );

        let infos_to_print = [
            (&castle.name, Self::PADDING_VERT),
            (&alive_str.to_string(), Self::PADDING_VERT + 1),
            (&pos_str, Self::PADDING_VERT + 2),
            (&peasants_str, Self::PADDING_VERT + 5),
            (&knights_str, Self::PADDING_VERT + 6),
            (&mages_str, Self::PADDING_VERT + 7),
            (&dragons_str, Self::PADDING_VERT + 8),
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

    pub fn add_logs_tab(content: &mut Vec<Vec<TermCell>>, logs: &Logs) {
        let mut chatbox: VecDeque<Vec<TermCell>> = VecDeque::with_capacity(Self::CONTENT_ROWS);

        let available_width = Self::CONTENT_COLS - (Self::PADDING_HORI * 2);
        let available_height = Self::CONTENT_ROWS - (Self::PADDING_VERT * 2);
        let tab_width = 4;
        let continuation_start = Self::PADDING_HORI + tab_width;
        let continuation_width = Self::CONTENT_COLS - continuation_start - Self::PADDING_HORI;

        // First, expand all logs into individual lines (oldest first)
        let mut all_lines: Vec<String> = Vec::new();

        for log in logs.content.iter() {
            if log.len() <= available_width {
                all_lines.push(log.clone());
            } else {
                let chars: Vec<char> = log.chars().collect();
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
