use common::{
    all_units,
    exports::{game_object::GameObjE, units::UnitType},
};

use crate::{
    ansi::{BG_BLACK, FG_BLACK},
    assets::{SELECTION_TERMCELL, TermCell},
    game_state::GameState,
    renderer::{
        r#const::MOD_INTERACT_COLS,
        map_data::MapData,
        module_utility::{self, draw_text_in_row},
    },
    ui_state::{UiMode, UiState},
};

pub struct ModInteract {}

impl ModInteract {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_COLS: usize = MOD_INTERACT_COLS.saturating_sub(2);

    pub fn update(
        game_state: &GameState,
        ui_state: &UiState,
        map_data: &MapData,
    ) -> Option<Vec<Vec<TermCell>>> {
        match ui_state.mode {
            UiMode::Interact(ref interact) => {
                let tile = map_data.get_tile(interact.coord);
                let obj = interact
                    .obj_id
                    .and_then(|obj_id| game_state.objs.get(&obj_id));
                let mut renderable = Vec::new();

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                match obj {
                    Some(GameObjE::Castle(castle)) => {
                        Self::push_row_with_text(&mut renderable, &castle.name);
                        Self::push_row_with_text(&mut renderable, "a: attack");
                        Self::push_empty_row(&mut renderable);
                    }
                    Some(GameObjE::Structure(_)) => {}
                    Some(GameObjE::DeployedUnits(_)) => {}
                    None => {
                        Self::push_row_with_text(&mut renderable, &format!("{:?}", tile));
                        Self::push_row_with_text(&mut renderable, "a: send troops");
                    }
                }

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                module_utility::add_frame("interact", &mut renderable);

                Some(renderable)
            }
            UiMode::UnitSelection(ref selection) => {
                let all_units = all_units!();
                let mut renderable = Vec::new();

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                for (i, unit) in all_units.iter().enumerate() {
                    if selection.active_input.0 == *unit {
                        Self::push_row_with_text(
                            &mut renderable,
                            &format!(
                                "{:?}: {}/{}",
                                unit,
                                selection.active_input.1,
                                game_state.player.units.quantities[i]
                            ),
                        );

                        renderable.last_mut().unwrap()
                            [Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1)] =
                            SELECTION_TERMCELL;
                    } else {
                        Self::push_row_with_text(
                            &mut renderable,
                            &format!(
                                "{:?}: {}/{}",
                                unit,
                                selection.selected_units.quantities[i],
                                game_state.player.units.quantities[i]
                            ),
                        );
                    }
                }

                Self::push_row_with_text(&mut renderable, "a: confirm");

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                module_utility::add_frame("units selection", &mut renderable);

                Some(renderable)
            }
            _ => None,
        }
    }

    fn push_empty_row(renderable: &mut Vec<Vec<TermCell>>) {
        renderable.push(vec![
            TermCell::new(' ', FG_BLACK, BG_BLACK);
            Self::CONTENT_COLS
        ]);
    }

    fn push_row_with_text(renderable: &mut Vec<Vec<TermCell>>, text: &str) {
        renderable.push(vec![
            TermCell::new(' ', FG_BLACK, BG_BLACK);
            Self::CONTENT_COLS
        ]);
        let row_to_write = renderable.len() - 1;
        draw_text_in_row(
            renderable,
            text,
            row_to_write,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );
    }
}
