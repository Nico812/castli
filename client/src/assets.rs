use std::env::current_exe;
use std::{collections::HashMap, fs::read_to_string};

use common::{courtyard::Facility, game_objs::GameObjE, map::Tile};
use crossterm::style::Color;
use serde::Deserialize;

use crate::renderer::renderer::{DrawableAsset, DrawableCell};

pub const BLOCK: char = '▀';

#[derive(Deserialize, Copy, Clone)]
pub struct WorldColor {
    day: Color,
    night: Color,
}

#[derive(Clone, Copy)]
pub enum DynamicColor {
    UiColor(Color),
    WorldColor(WorldColor),
}

impl DynamicColor {
    pub fn to_crossterm_color(&self, night: bool) -> Color {
        match self {
            Self::UiColor(color) => *color,
            Self::WorldColor(color) => {
                if night {
                    color.night
                } else {
                    color.day
                }
            }
        }
    }
}

#[derive(Deserialize, Clone)]
struct AssetCellUnresolved {
    ch: Option<char>,
    fg: Option<String>,
    bg: Option<String>,
}

#[derive(Deserialize)]
struct TileAssetUnresolved {
    up: String,
    down: String,
    std: AssetCellUnresolved,
    var: AssetCellUnresolved,
}

#[derive(Deserialize)]
struct ObjectAssetUnresolved {
    cells: Vec<Vec<AssetCellUnresolved>>,
}

#[derive(Clone, Copy)]
pub struct AssetCell {
    pub ch: char,
    pub fg: DynamicColor,
    pub bg: DynamicColor,
}

impl AssetCell {
    pub fn to_drawable_cell(&self, night: bool) -> DrawableCell {
        let fg = self.fg.to_crossterm_color(night);
        let bg = self.bg.to_crossterm_color(night);

        DrawableCell::new(self.ch, fg, bg)
    }
}

pub struct TileAsset {
    pub up: DynamicColor,
    pub down: DynamicColor,
    pub std: AssetCell,
    pub var: AssetCell,
}

pub struct ObjectAsset {
    pub cells: Vec<Vec<Option<AssetCell>>>,
}

pub struct Assets {
    colors_ui: HashMap<String, Color>,
    colors_world: HashMap<String, WorldColor>,
    ui_elements: HashMap<String, AssetCell>,
    tiles: HashMap<String, TileAsset>,
    objs: HashMap<String, ObjectAsset>,
}

impl Assets {
    pub fn load() -> Self {
        let mut exe_path = current_exe().expect("Failed to find executable path");
        exe_path.pop();
        exe_path.pop();
        exe_path.pop();
        let mut assets_dir_path = exe_path.into_os_string().into_string().unwrap();
        assets_dir_path.push_str("/client/assets/");

        let colors_ui: HashMap<String, Color> = toml::from_str(
            &read_to_string(assets_dir_path.clone() + "/colors_ui.toml")
                .expect("Failed to read colors_ui.toml"),
        )
        .expect("Failed to parse colors.toml");

        let colors_world: HashMap<String, WorldColor> = toml::from_str(
            &read_to_string(assets_dir_path.clone() + "/colors_world.toml")
                .expect("Failed to read colors_world.toml"),
        )
        .expect("Failed to parse colors.toml");

        let ui_elements: HashMap<String, AssetCell> =
            toml::from_str::<HashMap<String, AssetCellUnresolved>>(
                &read_to_string(assets_dir_path.clone() + "/ui_elements.toml")
                    .expect("Failed to read ui_elements.toml"),
            )
            .expect("Failed to parse ui_elements.toml")
            .into_iter()
            .map(|(name, asset_unresolved)| {
                (
                    name,
                    Self::resolve_cell(asset_unresolved, &colors_ui, &colors_world)
                        .expect("Trying to resolve empty ui element"),
                )
            })
            .collect();

        let tiles: HashMap<String, TileAsset> =
            toml::from_str::<HashMap<String, TileAssetUnresolved>>(
                &read_to_string(assets_dir_path.clone() + "/tiles.toml")
                    .expect("Failed to read tiles.toml"),
            )
            .expect("Failed to parse tiles.toml")
            .into_iter()
            .map(|(name, asset_unresolved)| {
                (
                    name,
                    Self::resolve_tile_asset(asset_unresolved, &colors_ui, &colors_world),
                )
            })
            .collect();

        let objs: HashMap<String, ObjectAsset> =
            toml::from_str::<HashMap<String, ObjectAssetUnresolved>>(
                &read_to_string(assets_dir_path + "/objects.toml")
                    .expect("Failed to read objects.toml"),
            )
            .expect("Failed to parse objects.toml")
            .into_iter()
            .map(|(name, asset_unresolved)| {
                (
                    name,
                    Self::resolve_object_asset(asset_unresolved, &colors_ui, &colors_world),
                )
            })
            .collect();

        Self {
            colors_ui,
            colors_world,
            ui_elements,
            tiles,
            objs,
        }
    }

    pub fn get_ui_element_drawable(&self, name: &str) -> DrawableCell {
        self.ui_elements
            .get(name)
            .expect(&format!(
                "Unable to fetch ui element with name \"{}\"",
                name
            ))
            .to_drawable_cell(false)
    }

    pub fn get_facility_asset_drawable(&self, facility: &Facility, night: bool) -> DrawableAsset {
        let object_key = format!("{:?}", facility.r#type);

        self.objs
            .get(&object_key)
            .expect("Unable to fetch facility asset")
            .cells
            .iter()
            .map(|asset_row| {
                asset_row
                    .iter()
                    .map(|opt_cell| opt_cell.map(|cell| cell.to_drawable_cell(night)))
                    .collect()
            })
            .collect()
    }

    pub fn get_tile_asset(&self, tile: &Tile) -> &TileAsset {
        let tile_key = format!("{:?}", tile);

        self.tiles
            .get(&tile_key)
            .expect("Unable to fetch tile asset")
    }

    pub fn get_world_obj_asset_drawable(
        &self,
        world_obj: &GameObjE,
        owned: bool,
        night: bool,
    ) -> DrawableAsset {
        let object_key = match world_obj {
            GameObjE::Castle(castle) => {
                if !castle.alive {
                    "DeadCastle"
                } else if owned {
                    "MyCastle"
                } else {
                    "Castle"
                }
            }
            GameObjE::Structure(_) => "Err",
            GameObjE::DeployedUnits(_) => {
                if owned {
                    "MyDeployedUnits"
                } else {
                    "DeployedUnits"
                }
            }
        };

        self.objs
            .get(object_key)
            .expect("Unable to fetch world_obj asset")
            .cells
            .iter()
            .map(|asset_row| {
                asset_row
                    .iter()
                    .map(|opt_cell| opt_cell.map(|cell| cell.to_drawable_cell(night)))
                    .collect()
            })
            .collect()
    }

    fn resolve_color_ref(
        color_ref: &str,
        colors_ui: &HashMap<String, Color>,
        colors_world: &HashMap<String, WorldColor>,
    ) -> DynamicColor {
        if let Some(color_name) = color_ref.strip_prefix("WORLD.") {
            let color = colors_world
                .get(color_name)
                .expect("Failed to fetch color from table");
            DynamicColor::WorldColor(*color)
        } else {
            let color_name = color_ref
                .strip_prefix("UI.")
                .expect("Trying to fetch color that doesnt start with UI./WORLD.");
            let color = colors_ui
                .get(color_name)
                .expect("Failed to fetch color from table");
            DynamicColor::UiColor(*color)
        }
    }

    fn resolve_cell(
        cell: AssetCellUnresolved,
        colors_ui: &HashMap<String, Color>,
        colors_world: &HashMap<String, WorldColor>,
    ) -> Option<AssetCell> {
        if let (Some(ch), Some(fg), Some(bg)) = (cell.ch, cell.fg, cell.bg) {
            Some(AssetCell {
                ch: ch,
                fg: Self::resolve_color_ref(&fg, colors_ui, colors_world),
                bg: Self::resolve_color_ref(&bg, colors_ui, colors_world),
            })
        } else {
            None
        }
    }

    fn resolve_tile_asset(
        tile_asset: TileAssetUnresolved,
        colors_ui: &HashMap<String, Color>,
        colors_world: &HashMap<String, WorldColor>,
    ) -> TileAsset {
        TileAsset {
            up: Self::resolve_color_ref(&tile_asset.up, colors_ui, colors_world),
            down: Self::resolve_color_ref(&tile_asset.down, colors_ui, colors_world),
            std: Self::resolve_cell(tile_asset.std, colors_ui, colors_world)
                .expect("Trying to resolve empty tile"),
            var: Self::resolve_cell(tile_asset.var, colors_ui, colors_world)
                .expect("Trying to resolve empty tile"),
        }
    }

    fn resolve_object_asset(
        obj_asset: ObjectAssetUnresolved,
        colors_ui: &HashMap<String, Color>,
        colors_world: &HashMap<String, WorldColor>,
    ) -> ObjectAsset {
        let cells = obj_asset
            .cells
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|unresolved| Self::resolve_cell(unresolved, colors_ui, colors_world))
                    .collect()
            })
            .collect();

        ObjectAsset { cells }
    }
}
