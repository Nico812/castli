use std::fmt::Pointer;

use common::{all_facilities, all_units, courtyard::FacilityType, game_objs::GameObjE};

use crate::{
    ansi::BLACK,
    assets::{SELECTION_TERMCELL, TermCell},
    game_state::GameState,
    renderer::{
        r#const::MOD_INTERACT_COLS,
        map_data::MapData,
        module_utility::{self, draw_text_in_row},
    },
    ui_state::{InteractTarget, UiMode, UiState},
};

pub struct ModInteract;

impl ModInteract {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_COLS: usize = MOD_INTERACT_COLS.saturating_sub(2);

    pub fn update(game_state: &GameState, ui_state: &UiState) -> Option<Vec<Vec<TermCell>>> {
        match ui_state.mode {
            UiMode::Interact(ref interact_target) => {
                let mut renderable = Vec::new();

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                match interact_target {
                    InteractTarget::GameObj(obj_id) => {
                        let obj = game_state.objs.get(&obj_id);

                        match obj {
                            Some(GameObjE::Castle(castle)) => {
                                Self::push_row_with_text(&mut renderable, &castle.name);
                                Self::push_row_with_text(&mut renderable, "a: attack");
                            }
                            Some(GameObjE::Structure(_)) => {}
                            Some(GameObjE::DeployedUnits(_)) => {}
                            None => {
                                Self::push_row_with_text(
                                    &mut renderable,
                                    "The object doesn't exist anymore",
                                );
                            }
                        }
                    }
                    InteractTarget::MapPos(pos) => {
                        let tile = game_state.get_tile(*pos);
                        Self::push_row_with_text(&mut renderable, &format!("{:?}", tile));
                        Self::push_row_with_text(&mut renderable, "a: send troops");
                    }
                    InteractTarget::Facility(facility_id) => {
                        let Some(facility) = game_state.get_facility(*facility_id) else {
                            return None;
                        };
                        Self::push_row_with_text(
                            &mut renderable,
                            &format!("{:?}", facility.r#type),
                        );
                        Self::push_row_with_text(&mut renderable, &format!("lv {}", facility.lv));
                    }
                    InteractTarget::CourtyardPos(_) => {
                        Self::push_row_with_text(&mut renderable, "n: build");
                    }
                }

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                module_utility::add_frame("interact", &mut renderable);

                Some(renderable)
            }
            UiMode::UnitSelection(ref selection) => {
                let Some(ref castle) = game_state.castle else {
                    return None;
                };
                let all_units = all_units!();
                let mut renderable = Vec::new();

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                for (i, unit) in all_units.iter().enumerate() {
                    let is_active = selection.active_input.0 == *unit;
                    let total = castle.units.quantities[i];

                    let display_quantities =
                        if is_active && let Some(ref input_str) = selection.active_input.1 {
                            format!("{}_/{}", input_str, total)
                        } else {
                            let selected = selection.selected_units.quantities[i];
                            format!("{}/{}", selected, total)
                        };

                    let text = format!("{:?}: {}", unit, display_quantities);

                    Self::push_row_with_text(&mut renderable, &text);
                    if is_active {
                        let marker_pos = Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1);
                        renderable.last_mut().unwrap()[marker_pos] = SELECTION_TERMCELL;
                    }
                }
                Self::push_empty_row(&mut renderable);
                Self::push_row_with_text(&mut renderable, "enter: select/set amount");
                Self::push_row_with_text(&mut renderable, "a: confirm");

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                module_utility::add_frame("units selection", &mut renderable);

                Some(renderable)
            }
            UiMode::FacilitySelection(ref selection) => {
                let all_facilities = all_facilities!();
                let mut renderable = Vec::new();

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                for facility_type in all_facilities.iter() {
                    let total = facility_type.max_count();
                    let owned = game_state
                        .facilities
                        .iter()
                        .filter(|facility| facility.1.r#type == *facility_type)
                        .count();

                    let is_active = selection.active == *facility_type;

                    let display_quantities = format!("{}/{}", owned, total);
                    let quantities_text = format!("{:?}: {}", facility_type, display_quantities);
                    let price_text = format!(
                        "Wood: {}, Stone: {}",
                        facility_type.base_cost().wood,
                        facility_type.base_cost().stone
                    );

                    Self::push_row_with_text(&mut renderable, &quantities_text);
                    if is_active {
                        let marker_pos = Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1);
                        renderable.last_mut().unwrap()[marker_pos] = SELECTION_TERMCELL;
                    }
                    Self::push_row_with_text(&mut renderable, &price_text);
                    Self::push_empty_row(&mut renderable);
                }
                Self::push_empty_row(&mut renderable);
                Self::push_row_with_text(&mut renderable, "enter: build");

                for _ in 0..Self::PADDING_VERT {
                    Self::push_empty_row(&mut renderable);
                }

                module_utility::add_frame("facility selection", &mut renderable);

                Some(renderable)
            }
            _ => None,
        }
    }

    fn push_empty_row(renderable: &mut Vec<Vec<TermCell>>) {
        renderable.push(vec![TermCell::new(' ', BLACK, BLACK); Self::CONTENT_COLS]);
    }

    fn push_row_with_text(renderable: &mut Vec<Vec<TermCell>>, text: &str) {
        renderable.push(vec![TermCell::new(' ', BLACK, BLACK); Self::CONTENT_COLS]);
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
