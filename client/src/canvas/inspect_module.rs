use crate::{
    ansi::{BG_BLACK, BG_GREEN_BRIGHT, BG_GREY, FG_BLACK, FG_GREEN_BRIGHT, FG_GREY},
    assets::TermCell,
    canvas::module_utility::{add_frame, draw_text_in_row},
    tui::SharedState,
};
use common::exports::{game_object::GameObjE, tile::TileE};

pub struct InspectModule {
    pub shown: bool,
}

impl InspectModule {
    const PADDING_HORI: usize = 2;
    const CONTENT_COLS: usize = 30;

    pub fn get_renderable(
        state: SharedState,
        map_tiles: Vec<Vec<TileE>>,
    ) -> Option<Vec<Vec<TermCell>>> {
        if state.map_look == None {
            return None;
        }
        let look_coord = state.map_look.unwrap();

        // TODO: implement this in the MapTiles struct (that you have to create)
        let looked_tile: TileE = map_tiles
            .get(look_coord.y)
            .and_then(|row| row.get(look_coord.x))
            .copied()
            .unwrap_or(TileE::Err);

        let looked_objs: Vec<&GameObjE> = state
            .game_objs
            .iter()
            .filter_map(|(_, game_obj)| {
                if game_obj.get_pos() == look_coord {
                    Some(game_obj)
                } else {
                    None
                }
            })
            .collect();

        let mut renderable;
        let objs_comp_opt = Self::create_objs_component(looked_objs);
        let mut tile_comp = Self::create_tile_component(looked_tile);

        if let Some(mut objs_comp) = objs_comp_opt {
            objs_comp.append(&mut tile_comp);
            renderable = objs_comp;
        } else {
            renderable = tile_comp;
        }

        add_frame(
            &format!("inspect: ({}, {})", look_coord.y, look_coord.x),
            &mut renderable,
        );

        Some(renderable)
    }

    fn create_objs_component(objs: Vec<&GameObjE>) -> Option<Vec<Vec<TermCell>>> {
        let mut castles_component: Vec<Vec<TermCell>> =
            vec![vec![TermCell::new(' ', FG_GREY, BG_GREY)]];
        let mut units_component: Vec<Vec<TermCell>> =
            vec![vec![TermCell::new(' ', FG_GREY, BG_GREY)]];
        let mut structures_component: Vec<Vec<TermCell>> =
            vec![vec![TermCell::new(' ', FG_GREY, BG_GREY)]];

        for obj in objs.iter() {
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
                }
                GameObjE::DeployedUnits(units) => {
                    Self::push_row_with_text(
                        &mut units_component,
                        format!("Owner: {}", units.owner_id),
                    );
                }
            }
        }
        if structures_component.len() > 1 {
            draw_text_in_row(
                &mut structures_component,
                &"Structures:".to_string(),
                0,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            castles_component.append(&mut structures_component);
        }
        if units_component.len() > 1 {
            draw_text_in_row(
                &mut units_component,
                &"Units:".to_string(),
                0,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            castles_component.append(&mut units_component);
        }
        if castles_component.len() > 1 {
            draw_text_in_row(
                &mut castles_component,
                &"Castles:".to_string(),
                0,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
            return Some(castles_component);
        } else {
            return None;
        }
    }

    fn create_tile_component(tile: TileE) -> Vec<Vec<TermCell>> {
        let mut tile_component: Vec<Vec<TermCell>> =
            vec![vec![TermCell::new(' ', FG_GREEN_BRIGHT, BG_GREEN_BRIGHT)]];
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
        renderable.push(vec![TermCell::new(' ', FG_BLACK, BG_BLACK)]);
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
