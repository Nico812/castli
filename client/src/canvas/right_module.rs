use std::collections::VecDeque;

use common::GameCoord;
use common::exports::player::PlayerE;
use common::exports::units::UnitE;

use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;
use crate::canvas::RightModuleTab;
use crate::canvas::module_utility::draw_text_in_row;
use crate::tui::SharedState;

pub struct RightModule {
    current_tab: RightModuleTab,
}

impl RightModule {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_ROWS: usize = RIGHT_MODULE_ROWS.saturating_sub(2);
    const CONTENT_COLS: usize = RIGHT_MODULE_COLS.saturating_sub(2);

    pub fn new() -> Self {
        let current_tab = RightModuleTab::Castle;
        Self { current_tab }
    }

    pub fn get_renderable_and_update(
        &self,
        frame_dt: u64,
        state: &mut SharedState,
    ) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            Self::CONTENT_ROWS
        ];

        match self.current_tab {
            RightModuleTab::Castle => Self::add_castle_tab(&mut content, &state.player_data),
            RightModuleTab::Debug => Self::add_debug_tab(&mut content, frame_dt, state.map_look),
            RightModuleTab::Logs => Self::add_logs_tab(&mut content, &mut state.chat),
        };
        module_utility::add_frame("inspect", &mut content);
        content
    }

    pub fn update_tab(&mut self, new_tab: RightModuleTab) {
        self.current_tab = new_tab;
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
            let look_pos_str = format!("Looking at ({}, {})", pos.y, pos.x);
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

        let recent_logs: Vec<&String> = logs.iter().take(Self::CONTENT_ROWS).collect();

        // Aggiungi i log dal più vecchio al più recente (per visualizzazione dall'alto verso il basso)
        for log in recent_logs.iter().rev() {
            let mut row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];

            for (i, ch) in log.chars().enumerate() {
                if i < Self::CONTENT_COLS - Self::PADDING_HORI {
                    row[Self::PADDING_HORI + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
                }
            }
            chatbox.push_back(row);
        }

        // Riempie con righe vuote all'inizio se non ci sono abbastanza log
        while chatbox.len() < Self::CONTENT_ROWS {
            let empty_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            chatbox.push_front(empty_row);
        }

        *content = chatbox.into();
    }
}
