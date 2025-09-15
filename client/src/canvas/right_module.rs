use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;

use common::GameObjE;

pub struct RightModule {
    // Inspect
}

impl RightModule {
    const PADDING_LEFT: usize = 2;
    const CONTENT_ROWS: usize = RIGHT_MODULE_ROWS - 2;
    const CONTENT_COLS: usize = RIGHT_MODULE_COLS - 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_renderable_and_update(
        &self,
        frame_dt: u64,
        sel_obj: Option<&GameObjE>,
    ) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            Self::CONTENT_ROWS
        ];

        // Show FPS
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        module_utility::draw_text(&mut content, &dt_str, 1, Self::PADDING_LEFT);

        // Show looked object
        let mut current_row = 3;
        if let Some(obj) = sel_obj {
            match obj {
                GameObjE::Castle(castle) => {
                    module_utility::draw_text(
                        &mut content,
                        "--- Castle ---",
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 2;

                    let name_str = format!("Name: {}", castle.name);
                    module_utility::draw_text(
                        &mut content,
                        &name_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 1;

                    let pos_str = format!("Position: ({}, {})", castle.pos.0, castle.pos.1);
                    module_utility::draw_text(
                        &mut content,
                        &pos_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                }
                GameObjE::Structure(structure) => {
                    module_utility::draw_text(
                        &mut content,
                        "--- Structure ---",
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 2;

                    let name_str = format!("Name: {}", structure.name);
                    module_utility::draw_text(
                        &mut content,
                        &name_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 1;

                    let type_str = format!("Type: {:?}", structure.r#type);
                    module_utility::draw_text(
                        &mut content,
                        &type_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 1;

                    let pos_str = format!("Position: ({}, {})", structure.pos.0, structure.pos.1);
                    module_utility::draw_text(
                        &mut content,
                        &pos_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                }
                GameObjE::UnitGroup(unit_group) => {
                    module_utility::draw_text(
                        &mut content,
                        "--- Unit Group ---",
                        current_row,
                        Self::PADDING_LEFT,
                    );
                    current_row += 2;

                    let owner_str = format!("Owner: {}", unit_group.owner);
                    module_utility::draw_text(
                        &mut content,
                        &owner_str,
                        current_row,
                        Self::PADDING_LEFT,
                    );
                }
            }
        }

        module_utility::add_frame("inspect", &mut content);
        content
    }
}
