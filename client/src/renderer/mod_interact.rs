use common::{GroupElement, courtyard::FacilityType, game_objs::GameObjE};

use crate::{
    assets::Assets,
    coord::TermCoord,
    game_state::GameState,
    renderer::{module::Module, renderer::DrawableCell},
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
        assets: &Assets,
    ) -> Option<Vec<Vec<DrawableCell>>> {
        match ui_state.mode {
            UiMode::Interact(ref interact_target) => {
                match interact_target {
                    InteractTarget::GameObj(obj_id) => {
                        let obj = game_state.get_obj(*obj_id);

                        match obj {
                            Some(GameObjE::Castle(castle)) => {
                                self.module.push_row_with_text(&castle.name);
                                self.module.push_row_with_text("a: attack");
                            }
                            Some(GameObjE::Structure(structure)) => {
                                self.module
                                    .push_row_with_text(&structure.r#type.to_string());
                                self.module.push_row_with_text("a: attack");
                            }
                            Some(GameObjE::DeployedUnits(units)) => {
                                self.module
                                    .push_row_with_text(&format!("Owner id: {}", units.owner_id));
                            }
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
                        let desc_textbox_cols = 25;
                        let Some(facility) = game_state.get_facility(*facility_id) else {
                            return None;
                        };
                        self.module
                            .push_row_with_text(&format!("{:?}", facility.r#type));
                        self.module.draw_text_in_box(
                            facility.r#type.desc(),
                            TermCoord::new(2, 0),
                            desc_textbox_cols,
                            true,
                        );
                        let mut curr_pos =
                            TermCoord::new(0, desc_textbox_cols + self.module.get_padding().x);

                        self.module.draw_text_in_pos(
                            &format!("lv {}/{}", facility.lv, facility.r#type.max_lv()),
                            curr_pos,
                            true,
                        );
                        curr_pos.y += 2;

                        for production in facility.production() {
                            self.module.draw_text_in_pos("Production", curr_pos, true);
                            curr_pos.y += 1;
                            self.module
                                .draw_text_in_pos(&production.to_string(), curr_pos, true);
                            curr_pos.y += 2;
                        }

                        self.module.draw_text_in_pos("n: upgrade", curr_pos, true);
                        for string in facility.upgrade_cost().as_string_vector_nonzero() {
                            curr_pos.y += 1;
                            self.module.draw_text_in_pos(&string, curr_pos, true);
                        }
                    }
                    InteractTarget::CourtyardPos(_) => {
                        self.module.push_row_with_text("n: build");
                    }
                }

                self.module.set_name("interact".to_string());
                Some(self.module.get_renderable_and_reset().clone())
            }
            UiMode::UnitSelection(ref selection) => {
                let Some(ref castle) = game_state.castle else {
                    return None;
                };

                for (ty, amt) in castle.units.iter() {
                    let ty_index = ty.as_index();
                    let is_active = selection.active_input.0 == ty;

                    let display_quantities =
                        if is_active && let Some(ref input_str) = selection.active_input.1 {
                            format!("{}_/{}", input_str, amt)
                        } else {
                            let selected = selection.selected_units.quant[ty_index];
                            format!("{}/{}", selected, amt)
                        };

                    let text = format!("{:?}: {}", ty, display_quantities);

                    self.module.push_row_with_text(&text);

                    if is_active {
                        let selection_icon = assets.get_ui_element_drawable("SELECTION");
                        self.module
                            .draw_cell_last_row(selection_icon, self.module.drawable_size().x - 1);
                    }
                }
                self.module.push_empty_row();
                self.module.push_row_with_text("enter: select/set amount");
                self.module.push_row_with_text("a: confirm");

                self.module.set_name("unit selection".to_string());
                Some(self.module.get_renderable_and_reset().clone())
            }
            UiMode::FacilitySelection(ref selection) => {
                for ty in FacilityType::all().iter() {
                    let total = FacilityType::max_count()[ty.as_index()];
                    let owned = game_state
                        .get_facilities()
                        .iter()
                        .filter(|facility| facility.1.r#type == *ty)
                        .count();

                    let is_active = selection.active == *ty;

                    let display_quantities = format!("{}/{}", owned, total);
                    let quantities_text = format!("{:?}: {}", ty, display_quantities);
                    let price_text = ty.base_cost().to_string();

                    self.module.push_row_with_text(&quantities_text);
                    if is_active {
                        self.module.draw_cell_last_row(
                            assets.get_ui_element_drawable("SELECTION"),
                            self.module.drawable_size().x - 1,
                        );
                    }
                    self.module.push_row_with_text(&price_text);
                    self.module.push_empty_row();
                }
                self.module.push_empty_row();
                self.module.push_row_with_text("enter: build");

                self.module.set_name("facility selection".to_string());
                Some(self.module.get_renderable_and_reset().clone())
            }
            _ => None,
        }
    }
}
