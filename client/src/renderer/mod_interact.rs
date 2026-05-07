use common::{all_facilities, all_units, game_objs::GameObjE};

use crate::{
    ansi::BLACK,
    assets::{SELECTION_TERMCELL, TermCell},
    game_state::GameState,
    renderer::{r#const::MOD_INTERACT_COLS, module::Module},
    ui_state::{InteractTarget, UiMode, UiState},
};

pub struct ModInteract {
    module: Module,
}

impl ModInteract {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn render(
        &mut self,
        game_state: &GameState,
        ui_state: &UiState,
    ) -> Option<Vec<Vec<TermCell>>> {
        match ui_state.mode {
            UiMode::Interact(ref interact_target) => {
                match interact_target {
                    InteractTarget::GameObj(obj_id) => {
                        let obj = game_state.objs.get(&obj_id);

                        match obj {
                            Some(GameObjE::Castle(castle)) => {
                                self.module.push_row_with_text(&castle.name);
                                self.module.push_row_with_text("a: attack");
                            }
                            Some(GameObjE::Structure(_)) => {}
                            Some(GameObjE::DeployedUnits(_)) => {}
                            None => {
                                self.module
                                    .push_row_with_text("The object doesn't exist anymore");
                            }
                        }
                    }
                    InteractTarget::MapPos(pos) => {
                        let tile = game_state.get_tile(*pos);
                        self.module.push_row_with_text(&format!("{:?}", tile));
                        self.module.push_row_with_text("a: send troops");
                    }
                    InteractTarget::Facility(facility_id) => {
                        let Some(facility) = game_state.get_facility(*facility_id) else {
                            return None;
                        };
                        self.module
                            .push_row_with_text(&format!("{:?}", facility.r#type));
                        self.module
                            .push_row_with_text(&format!("lv {}", facility.lv));
                    }
                    InteractTarget::CourtyardPos(_) => {
                        self.module.push_row_with_text("n: build");
                    }
                }

                self.module.set_name("interact".to_string());
                Some(self.module.get_cells().clone())
            }
            UiMode::UnitSelection(ref selection) => {
                let Some(ref castle) = game_state.castle else {
                    return None;
                };
                let all_units = all_units!();

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

                    self.module.push_row_with_text(&text);

                    if is_active {
                        let selection_icon = SELECTION_TERMCELL;
                        self.module
                            .draw_cell_last_row(selection_icon, self.module.drawable_size().x - 1);
                    }
                }
                self.module.push_empty_row();
                self.module.push_row_with_text("enter: select/set amount");
                self.module.push_row_with_text("a: confirm");

                self.module.set_name("unit selection".to_string());
                Some(self.module.get_cells().clone())
            }
            UiMode::FacilitySelection(ref selection) => {
                let all_facilities = all_facilities!();

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

                    self.module.push_row_with_text(&quantities_text);
                    if is_active {
                        self.module.draw_cell_last_row(
                            SELECTION_TERMCELL,
                            self.module.drawable_size().x - 1,
                        );
                    }
                    self.module.push_row_with_text(&price_text);
                    self.module.push_empty_row();
                }
                self.module.push_empty_row();
                self.module.push_row_with_text("enter: build");

                self.module.set_name("facility selection".to_string());
                Some(self.module.get_cells().clone())
            }
            _ => None,
        }
    }
}
