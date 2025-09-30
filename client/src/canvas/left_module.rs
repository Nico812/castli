use common::exports::player::PlayerE;
use common::exports::units::UnitE;

use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;
use crate::canvas::module_utility::string_into_content;

pub struct LeftModule {
    // Player data
}

impl LeftModule {
    const PADDING_LEFT: usize = 1;
    const PADDING_RIGHT: usize = 1;
    const CONTENT_ROWS: usize = LEFT_MODULE_ROWS - 2;
    const CONTENT_COLS: usize = LEFT_MODULE_COLS - 2;

    pub fn new() -> Self {
        Self {}
    }

    // TODO: make the module only compute the parts like "Peasants:" only once.
    pub fn get_renderable_and_update(&self, player: &PlayerE) -> Vec<Vec<TermCell>> {
        let blank_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
        let mut content = vec![blank_row.clone(); Self::CONTENT_ROWS];

        let pos_str = format!("({}, {})", player.pos.y, player.pos.x);
        let peasants_str = format!("Peasants: {}", player.peasants);
        let knights_str = format!(
            "Knights: {}",
            player.units.quantities[UnitE::Knight.as_index()]
        );
        let mages_str = format!("Mages: {}", player.units.quantities[UnitE::Mage.as_index()]);
        let dragons_str = format!(
            "Dragons: {}",
            player.units.quantities[UnitE::Dragon.as_index()]
        );

        let infos_to_print = [
            (&player.name, 3),
            (&pos_str, 5),
            (&peasants_str, 6),
            (&mages_str, 7),
            (&dragons_str, 8),
        ];

        for info_to_print in infos_to_print {
            string_into_content(
                &mut content,
                info_to_print.0,
                info_to_print.1,
                Self::PADDING_LEFT,
                Self::PADDING_RIGHT,
            );
        }

        module_utility::add_frame("player", &mut content);
        content
    }
}
