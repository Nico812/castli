use crate::{
    ansi::{BG_BLACK, BG_WHITE, FG_BLACK},
    assets::{self, SELECTION_TERMCELL, TermCell, TileAsset},
    game_renderer::{
        r#const::MOD_INSPECT_COLS,
        map_data::MapData,
        module_utility::{add_frame, draw_text_in_row},
    },
    shared_state::{SharedState, UIState},
};
use common::{
    GameID,
    exports::{game_object::GameObjE, tile::TileE},
};

pub struct ModInspect {}

impl ModInspect {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_COLS: usize = MOD_INSPECT_COLS - 2;

    pub fn update(state: &mut SharedState, map_data: &MapData) -> Option<Vec<Vec<TermCell>>> {
        if let UIState::Inspect(ref inspect) = state.ui_state {
            let looked_tile = map_data.get_tile(inspect.coord);

            let mut renderable = Vec::new();

            for _ in 0..Self::PADDING_VERT {
                Self::push_empty_row(&mut renderable);
            }

            let looked_objs =
                SharedState::get_looked_objs(inspect.coord, &state.map_zoom, &state.game_objs);
            let selected_id = inspect.selection;

            if !looked_objs.is_empty() {
                let mut objs_comp = Self::create_objs_component(selected_id, looked_objs);
                renderable.append(&mut objs_comp);
            }

            let mut tile_comp = Self::create_tile_component(looked_tile);
            renderable.append(&mut tile_comp);

            for _ in 0..Self::PADDING_VERT {
                Self::push_empty_row(&mut renderable);
            }

            add_frame(&format!("inspect: {}", inspect.coord), &mut renderable);
            Some(renderable)
        } else {
            None
        }
    }

    fn create_objs_component(
        selected_id: Option<GameID>,
        objs: Vec<(GameID, &GameObjE)>,
    ) -> Vec<Vec<TermCell>> {
        let mut castles_component = Vec::new();
        let mut units_component = Vec::new();
        let mut structures_component = Vec::new();

        for (id, obj) in objs.iter() {
            let selected = selected_id.is_some_and(|id_| id_ == *id);

            match obj {
                GameObjE::Castle(castle) => {
                    let mut alive_str = "Alive".to_string();
                    if !castle.is_alive {
                        alive_str = "Dead".to_string();
                    }

                    Self::push_row_with_text(
                        &mut castles_component,
                        &format!(" : {}", castle.name),
                    );
                    castles_component.last_mut().unwrap()[Self::PADDING_HORI] =
                        assets::CASTLE_ART[0][0];
                    if selected {
                        castles_component.last_mut().unwrap()
                            [Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1)] =
                            SELECTION_TERMCELL;
                    }

                    Self::push_row_with_text(
                        &mut castles_component,
                        &format!("   {}, ID({})", alive_str, id),
                    );
                }
                GameObjE::Structure(structure) => {
                    Self::push_row_with_text(
                        &mut structures_component,
                        &format!("T: {:?}", structure.r#type),
                    );
                    if selected {
                        structures_component.last_mut().unwrap()
                            [Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1)] =
                            SELECTION_TERMCELL;
                    }
                    Self::push_row_with_text(&mut structures_component, &format!("ID: {}", id));
                }
                GameObjE::DeployedUnits(units) => {
                    Self::push_row_with_text(
                        &mut units_component,
                        &format!(" : OwnerID({}), ID({})", units.owner_id, id),
                    );
                    units_component.last_mut().unwrap()[Self::PADDING_HORI] =
                        assets::DEPLOYED_UNITS_ART[0][0];
                    if selected {
                        units_component.last_mut().unwrap()
                            [Self::CONTENT_COLS.saturating_sub(Self::PADDING_HORI + 1)] =
                            SELECTION_TERMCELL;
                    }
                }
            }
        }

        let mut renderable = Vec::new();

        if !castles_component.is_empty() {
            renderable.append(&mut castles_component);
        }
        if !structures_component.is_empty() {
            renderable.append(&mut structures_component);
        }
        if !units_component.is_empty() {
            renderable.append(&mut units_component);
        }

        renderable
    }

    fn create_tile_component(tile: TileE) -> Vec<Vec<TermCell>> {
        let mut tile_component = Vec::new();
        Self::push_row_with_text(&mut tile_component, &format!(" : {:?}", tile));
        tile_component.last_mut().unwrap()[Self::PADDING_HORI] = TileAsset::get_asset(tile).std;
        tile_component
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
