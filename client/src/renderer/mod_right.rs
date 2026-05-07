use std::collections::VecDeque;

use crate::ansi::*;
use crate::assets::*;
use crate::game_state::GameState;
use crate::game_state::Logs;
use crate::renderer::ModRightTab;
use crate::renderer::module::Module;
use crate::ui_state::UiState;
use common::Time;
use common::all_units;
use common::player::PlayerE;

pub struct ModRight {
    module: Module,
}

impl ModRight {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn render(
        &mut self,
        frame_dt: u64,
        game_state: &GameState,
        ui_state: &UiState,
    ) -> Vec<Vec<TermCell>> {
        match ui_state.tab {
            ModRightTab::Castle => self.draw_castle_tab(&game_state),
            ModRightTab::Debug => {
                self.draw_debug_tab(frame_dt, &game_state.player, &game_state.time)
            }
            ModRightTab::Logs => self.draw_logs_tab(&game_state.logs),
        };
        let title = "(y): me | (x): logs | (c): debug".to_string();
        self.module.set_name(title);
        self.module.get_cells().clone()
    }

    fn draw_debug_tab(&mut self, frame_dt: u64, player: &PlayerE, time: &Time) {
        let lobby_str = format!("Lobby {}", player.lobby);
        self.module.draw_text_in_row(&lobby_str, 0);
        let id_str = format!("Castle ID: {:?}", player.castle_id);
        self.module.draw_text_in_row(&id_str, 1);
        // Show FPS
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        self.module.draw_text_in_row(&dt_str, 2);
        let tick_str = format!("Tick: {}", time.tick_cnt);
        self.module.draw_text_in_row(&tick_str, 3);
    }

    fn draw_castle_tab(&mut self, game_state: &GameState) {
        let Some(ref castle) = game_state.castle else {
            self.module.draw_text_in_row("Welcome to Castli!", 0);
            self.module
                .draw_text_in_row("Press \"l\" then arrowkeys to move", 2);
            self.module
                .draw_text_in_row("Press enter to create your castle :)", 3);
            return;
        };

        let alive_str = if castle.alive { "Alive :)" } else { "Dead x|" };
        let pos_str = format!("{}", castle.pos);
        let time_str = format!("Time: {}", game_state.time.h);
        let wood_str = format!("Wood: {}", castle.resources.wood);
        let stone_str = format!("Stone: {}", castle.resources.stone);

        let mut unit_strings = Vec::new();
        for unit_type in all_units!() {
            let count = castle.units.quantities[unit_type.as_index()];
            let s = format!("{:?}: {}", unit_type, count);
            unit_strings.push(s);
        }

        let mut infos_to_print = vec![
            (castle.name.as_str(), 0),
            (alive_str, 1),
            (pos_str.as_str(), 2),
            (time_str.as_str(), 3),
            (wood_str.as_str(), 5),
            (stone_str.as_str(), 6),
        ];

        for (i, unit_str) in unit_strings.iter().enumerate() {
            infos_to_print.push((unit_str.as_str(), 8 + i));
        }

        for (text, row) in infos_to_print {
            self.module.draw_text_in_row(text, row);
        }
    }

    pub fn draw_logs_tab(&mut self, logs: &Logs) {
        let drawable_size = self.module.drawable_size();
        let tab_width = 4;

        // First, expand all logs into individual lines (oldest first)
        let mut all_lines: Vec<String> = Vec::new();

        for log in logs.content.iter() {
            if log.len() <= drawable_size.x {
                all_lines.push(log.clone());
            } else {
                let chars: Vec<char> = log.chars().collect();
                let mut pos = 0;
                let mut is_first_line = true;

                while pos < chars.len() {
                    let end = if is_first_line {
                        (pos + drawable_size.x).min(chars.len())
                    } else {
                        (pos + drawable_size.x).min(chars.len())
                    };

                    let mut line = String::with_capacity(drawable_size.x);
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
        let start_index = if all_lines.len() > drawable_size.y {
            all_lines.len() - drawable_size.y
        } else {
            0
        };

        let recent_lines = &all_lines[start_index..];

        // Add the content rows (oldest to newest from top to bottom)
        for line in recent_lines {
            self.module.push_empty_row();

            for (i, ch) in line.chars().enumerate() {
                let col_pos = i;
                if col_pos < drawable_size.x {
                    self.module
                        .draw_cell_last_row(TermCell::new(ch, WHITE, BLACK), col_pos);
                }
            }
        }
    }
}
