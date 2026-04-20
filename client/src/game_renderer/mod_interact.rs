use common::{GameCoord, exports::game_object::GameObjE};

use crate::{
    ansi::{BG_BLACK, FG_BLACK},
    assets::TermCell,
    game_renderer::{
        r#const::MOD_INTERACT_COLS, map_data::MapData, module_utility::draw_text_in_row,
    },
    shared_state::SharedState,
};

pub struct ModInteract {}

impl ModInteract {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_COLS: usize = MOD_INTERACT_COLS.saturating_sub(2);

    pub fn update(state: &mut SharedState, map_data: &MapData) -> Option<Vec<Vec<TermCell>>> {
        let interact_target = state.interact_target.as_mut()?;
        let coord = interact_target.pos;
        let tile = map_data.get_tile(coord);
        let obj = interact_target
            .obj_id
            .and_then(|obj_id| state.game_objs.get(&obj_id));

        let mut renderable = Vec::new();

        for _ in 0..Self::PADDING_VERT {
            Self::push_empty_row(&mut renderable);
        }

        match obj {
            Some(GameObjE::Castle(castle)) => {
                Self::push_row_with_text(&mut renderable, &castle.name);
                Self::push_row_with_text(&mut renderable, &"a: attack".to_string());
            }
            Some(GameObjE::Structure(_)) => {}
            Some(GameObjE::DeployedUnits(_)) => {}
            _ => {}
        }

        Self::push_empty_row(&mut renderable);
        Self::push_row_with_text(&mut renderable, &format!("{:?}", tile));
        Self::push_row_with_text(&mut renderable, &"s: send troops".to_string());

        Some(renderable)
    }

    fn push_empty_row(renderable: &mut Vec<Vec<TermCell>>) {
        renderable.push(vec![
            TermCell::new(' ', FG_BLACK, BG_BLACK);
            Self::CONTENT_COLS
        ]);
    }

    fn push_row_with_text(renderable: &mut Vec<Vec<TermCell>>, text: &String) {
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
