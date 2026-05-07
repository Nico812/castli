use crate::{
    assets::{GameObjAsset, SELECTION_TERMCELL, TermCell, TileAsset},
    game_state::GameState,
    renderer::module::Module,
    tui::Tui,
    ui_state::{CameraLocation, UiMode, UiState},
};
use common::{GameId, courtyard::Facility, game_objs::GameObjE, map::Tile};

pub struct ModInspect {
    module: Module,
}

impl ModInspect {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn render(
        &mut self,
        game_state: &GameState,
        ui_state: &UiState,
    ) -> Option<Vec<Vec<TermCell>>> {
        if let UiMode::Inspect(ref inspect) = ui_state.mode {
            match ui_state.camera.location {
                CameraLocation::Map | CameraLocation::WorldMap => {
                    let is_world_map = ui_state.camera.location == CameraLocation::WorldMap;
                    let night = game_state.time.night;
                    let looked_tile = game_state.get_tile(inspect.coord);

                    let mut looked_objs =
                        Tui::get_looked_objs(inspect.coord, &game_state.objs, is_world_map);
                    let selected_id = inspect.selection;

                    self.draw_objs_component(
                        &game_state.player.castle_id,
                        selected_id,
                        &mut looked_objs,
                    );

                    self.draw_tile_component(looked_tile, night);
                }
                CameraLocation::Courtyard => {
                    if let Some(looked_facility) =
                        Tui::get_looked_facility(inspect.coord, &game_state.facilities)
                    {
                        self.draw_facility_component(looked_facility.1);
                    };
                }
            }
            self.module.set_name(format!("inspect | {}", inspect.coord));
            Some(self.module.get_cells().clone())
        } else {
            None
        }
    }

    fn draw_objs_component(
        &mut self,
        owned_castle: &Option<GameId>,
        selected_id: Option<GameId>,
        objs: &mut Vec<(GameId, &GameObjE)>,
    ) {
        if objs.is_empty() {
            return;
        }
        fn sort_priority(obj: &(GameId, &GameObjE)) -> u8 {
            match obj.1 {
                GameObjE::Castle(_) => 0,
                GameObjE::Structure(_) => 1,
                GameObjE::DeployedUnits(_) => 2,
            }
        }
        objs.sort_by_key(|obj| sort_priority(obj));

        for (id, obj) in objs.iter() {
            let selected = selected_id.is_some_and(|id_| id_ == *id);
            let selected_icon = SELECTION_TERMCELL;
            match obj {
                GameObjE::Castle(castle) => {
                    let owned = *owned_castle == Some(*id);
                    let icon = GameObjAsset::get_asset(obj, owned)[0][0];
                    let name_string = format!(" : {}", castle.name).to_string();

                    self.module.push_row_with_text(&name_string);
                    self.module.draw_cell_last_row(icon, 0);
                    if selected {
                        self.module
                            .draw_cell_last_row(selected_icon, self.module.drawable_size().x - 1)
                    };

                    let info_string = if castle.alive {
                        format!("  alive, id {}", id).to_string()
                    } else {
                        format!("  dead, id: {}", id).to_string()
                    };

                    self.module.push_row_with_text(&info_string);
                }
                GameObjE::Structure(structure) => {
                    let owned = *owned_castle == Some(*id);
                    let icon = GameObjAsset::get_asset(obj, owned)[0][0];
                    let name_string = format!(" : {}", structure.name).to_string();

                    self.module.push_row_with_text(&name_string);
                    self.module.draw_cell_last_row(icon, 0);
                    if selected {
                        self.module
                            .draw_cell_last_row(selected_icon, self.module.drawable_size().x - 1)
                    };
                }
                GameObjE::DeployedUnits(units) => {
                    let owned = *owned_castle == Some(*id);
                    let icon = GameObjAsset::get_asset(obj, owned)[0][0];
                    let name_string = "{Units}".to_string();

                    self.module.push_row_with_text(&name_string);
                    self.module.draw_cell_last_row(icon, 0);
                    if selected {
                        self.module
                            .draw_cell_last_row(selected_icon, self.module.drawable_size().x - 1)
                    };

                    let info_string = format!("  owner {}, id {}", units.owner_id, id).to_string();

                    self.module.push_row_with_text(&info_string);
                }
            }
        }
    }

    fn draw_tile_component(&mut self, tile: Tile, night: bool) {
        let icon = TileAsset::get_asset(tile, night).std;
        self.module.push_row_with_text(&format!(" : {:?}", tile));
        self.module.draw_cell_last_row(icon, 0);
    }

    fn draw_facility_component(&mut self, facility: &Facility) {
        self.module
            .push_row_with_text(&format!("{:?}", facility.r#type));
        self.module
            .push_row_with_text(&format!("lv {}", facility.lv));
    }
}
