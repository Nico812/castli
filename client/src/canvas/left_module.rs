use crate::ansi::*;
use crate::assets::*;
use super::{r#const::*, module_utiltiy};

pub struct LeftModule {
    // Player data
}

impl LeftModule {
    const PADDING_LEFT: usize = 1;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_renderable_and_update(&self, player: &common::PlayerE) -> Vec<Vec<TermCell>> {
        let blank_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); LEFT_MODULE_COLS];
        let mut content = vec![blank_row.clone(); LEFT_MODULE_ROWS];

        let name = &player.name;
        for (i, ch) in name.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[3][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
            }
        }
        let pos_str = format!("({}, {})", player.pos.0, player.pos.1);
        for (i, ch) in pos_str.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[5][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
            }
        }
        module_utility::add_frame("player", content);
        content
    }
}
