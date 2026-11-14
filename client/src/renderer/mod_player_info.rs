use crate::coord::TermCoord;
use crate::game_state::GameState;
use crate::logs::Logs;
use crate::renderer::ModPlayerInfoTab;
use crate::renderer::module::Module;
use crate::renderer::renderer::DrawableCell;
use crate::ui_state::UiState;
use common::Time;
use common::all_units;
use common::player::PlayerE;
use crossterm::style::Color;

pub struct ModPlayerInfo {
    module: Module,
}

impl ModPlayerInfo {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn render(
        &mut self,
        frame_dt: u64,
        game_state: &GameState,
        ui_state: &UiState,
    ) -> Vec<Vec<DrawableCell>> {
        match ui_state.tab {
            ModPlayerInfoTab::Castle => self.draw_castle_tab(game_state),
            ModPlayerInfoTab::Debug => {
                self.draw_debug_tab(frame_dt, &game_state.player, &game_state.time)
            }
            ModPlayerInfoTab::Logs => self.draw_logs_tab(&game_state.logs),
        };
        let title = "(y): me | (x): logs | (c): debug".to_string();
        self.module.set_name(title);
        self.module.get_cells().clone()
    }

    fn draw_debug_tab(&mut self, frame_dt: u64, player: &PlayerE, time: &Time) {
        let mut pos_to_draw = TermCoord::new(0, 0);
        let lobby_str = format!("Lobby {}", player.lobby);
        self.module.draw_text_in_pos(&lobby_str, pos_to_draw);

        pos_to_draw.y += 1;
        let id_str = format!("Castle ID: {:?}", player.castle_id);
        self.module.draw_text_in_pos(&id_str, pos_to_draw);

        pos_to_draw.y += 1;
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        self.module.draw_text_in_pos(&dt_str, pos_to_draw);

        pos_to_draw.y += 1;
        let tick_str = format!("Tick: {}", time.tick_cnt);
        self.module.draw_text_in_pos(&tick_str, pos_to_draw);
    }

    fn draw_castle_tab(&mut self, game_state: &GameState) {
        let mut pos_to_draw = TermCoord::new(0, 0);

        if let Some(ref castle) = game_state.castle {
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
                (pos_str.as_str(), 1),
                (time_str.as_str(), 1),
                (wood_str.as_str(), 1),
                (stone_str.as_str(), 2),
            ];

            for (i, unit_str) in unit_strings.iter().enumerate() {
                if i == 0 {
                    infos_to_print.push((unit_str.as_str(), 2))
                } else {
                    infos_to_print.push((unit_str.as_str(), 1))
                };
            }

            for (text, row) in infos_to_print {
                pos_to_draw.y += row;
                self.module.draw_text_in_pos(text, pos_to_draw);
            }
        } else {
            self.module
                .draw_text_in_pos("Welcome to Castli!", pos_to_draw);
            pos_to_draw.y += 2;
            self.module
                .draw_text_in_pos("Press \"l\" then arrowkeys to move", pos_to_draw);
            pos_to_draw.y += 1;
            self.module
                .draw_text_in_pos("Press enter to create your castle :)", pos_to_draw);
        };
    }

    pub fn draw_logs_tab(&mut self, logs: &Logs) {
        let drawable_size = self.module.drawable_size();
        let tab_width = 4;

        let mut all_lines: Vec<String> = Vec::new();

        for log in logs.content.iter().rev().take(drawable_size.y as usize) {
            if log.len() <= drawable_size.x as usize {
                all_lines.push(log.clone());
            } else {
                let chars: Vec<char> = log.chars().collect();
                let mut pos = 0;
                let mut is_first_line = true;

                while pos < chars.len() {
                    let end = if is_first_line {
                        (pos + drawable_size.x as usize).min(chars.len())
                    } else {
                        (pos + drawable_size.x as usize).min(chars.len())
                    };

                    let mut line = String::with_capacity(drawable_size.x as usize);
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

        for (line_row, line) in all_lines.iter().enumerate() {
            for (char_col, ch) in line.chars().enumerate() {
                if line_row < drawable_size.y as usize {
                    let pos =
                        TermCoord::new(drawable_size.y - line_row as isize - 1, char_col as isize);
                    self.module
                        .draw_cell(DrawableCell::new(ch, Color::White, Color::Black), pos);
                }
            }
        }
    }
}
