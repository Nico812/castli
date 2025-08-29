use crate::canvas::r#const::*;
use crate::ansi::*;
use crate::assets::*;

pub struct LeftModule {
    // Player data
}

impl LeftModule {
    const PADDING_LEFT: usize = 1;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self, player_data: &common::PlayerDataE) -> Vec<Vec<TermCell>> {
        let blank_row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); LEFT_MODULE_COLS];
        let mut content = vec![blank_row.clone(); LEFT_MODULE_ROWS];

        let name = &player_data.name;
        for (i, ch) in name.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[3][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BRIGHT_YELLOW);
            }
        }
        let pos_str = format!("({}, {})", player_data.pos.0, player_data.pos.1);
        for (i, ch) in pos_str.chars().enumerate() {
            if Self::PADDING_LEFT + i < LEFT_MODULE_COLS {
                content[5][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BRIGHT_YELLOW);
            }
        }
        content
    }
}