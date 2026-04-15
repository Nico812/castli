use crate::{
    ansi::{BG_BLACK, BG_GREEN_BRIGHT, BG_GREY, FG_BLACK, FG_GREEN_BRIGHT, FG_GREY},
    assets::TermCell,
    game_renderer::{
        r#const::MOD_INSPECT_COLS,
        map_data::MapData,
        module_utility::{add_frame, draw_text_in_row},
    },
    tui::SharedState,
};
use common::{
    GameID,
    exports::{game_object::GameObjE, tile::TileE},
};

pub struct ModInspect {}

impl ModInspect {
    const PADDING_HORI: usize = 2;
    const CONTENT_COLS: usize = MOD_INSPECT_COLS - 2;

    pub fn get_renderable(state: &SharedState, map_data: &MapData) -> Option<Vec<Vec<TermCell>>> {
        if state.map_look == None {
            return None;
        }
        let look_coord = state.map_look.unwrap();
        let looked_tile = map_data.get_tile(look_coord);

        let mut looked_objs: Vec<(&GameID, &GameObjE)> = state
            .game_objs
            .iter()
            .filter_map(|(game_id, game_obj)| {
                if game_obj.get_pos() == look_coord {
                    Some((game_id, game_obj))
                } else {
                    None
                }
            })
            .collect();

        let mut renderable = Vec::new();

        if looked_objs.len() > 0 {
            looked_objs.sort_by(|a, b| a.0.cmp(b.0));
            let mut objs_comp = Self::create_objs_component(looked_objs);
            renderable.append(&mut objs_comp);
        }

        let mut tile_comp = Self::create_tile_component(looked_tile);
        renderable.append(&mut tile_comp);

        add_frame(&format!("inspect: {}", look_coord), &mut renderable);

        Some(renderable)
    }

    fn create_objs_component(objs: Vec<(&GameID, &GameObjE)>) -> Vec<Vec<TermCell>> {
        let mut castles_component: Vec<Vec<TermCell>> = Vec::new();
        Self::push_row_with_text(&mut castles_component, "Castles:".to_string());

        let mut units_component: Vec<Vec<TermCell>> = Vec::new();
        Self::push_row_with_text(&mut units_component, "Units:".to_string());

        let mut structures_component: Vec<Vec<TermCell>> = Vec::new();
        Self::push_row_with_text(&mut structures_component, "Structures:".to_string());

        for (id, obj) in objs.iter() {
            match obj {
                GameObjE::Castle(castle) => {
                    Self::push_row_with_text(
                        &mut castles_component,
                        format!("Name: {}", castle.name),
                    );
                    Self::push_row_with_text(
                        &mut castles_component,
                        format!("Alive: {}", castle.is_alive),
                    );
                    Self::push_row_with_text(&mut castles_component, format!("ID: {}", id));
                }
                GameObjE::Structure(structure) => {
                    Self::push_row_with_text(
                        &mut structures_component,
                        format!("Name: {}", structure.name),
                    );
                    Self::push_row_with_text(
                        &mut structures_component,
                        format!("Type: {:?}", structure.r#type),
                    );
                    Self::push_row_with_text(&mut structures_component, format!("ID: {}", id));
                }
                GameObjE::DeployedUnits(units) => {
                    Self::push_row_with_text(
                        &mut units_component,
                        format!("Owner: {}", units.owner_id),
                    );
                    Self::push_row_with_text(&mut units_component, format!("ID: {}", id));
                }
            }
        }
        let mut renderable = Vec::new();

        if structures_component.len() > 1 {
            renderable.append(&mut structures_component);
        }
        if units_component.len() > 1 {
            renderable.append(&mut units_component);
        }
        if castles_component.len() > 1 {
            renderable.append(&mut castles_component);
        }

        renderable
    }

    fn create_tile_component(tile: TileE) -> Vec<Vec<TermCell>> {
        let mut tile_component: Vec<Vec<TermCell>> =
            vec![vec![
                TermCell::new(' ', FG_GREEN_BRIGHT, BG_GREEN_BRIGHT);
                Self::CONTENT_COLS
            ]];
        draw_text_in_row(
            &mut tile_component,
            &format!("Tile: {:?}", tile),
            0,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );

        tile_component
    }

    fn push_row_with_text(renderable: &mut Vec<Vec<TermCell>>, text: String) {
        renderable.push(vec![
            TermCell::new(' ', FG_BLACK, BG_BLACK);
            Self::CONTENT_COLS
        ]);
        let row_to_write = renderable.len() - 1;
        draw_text_in_row(
            renderable,
            &text,
            row_to_write,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );
    }
}
