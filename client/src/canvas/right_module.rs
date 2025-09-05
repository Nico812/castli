use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;

pub struct RightModule {
    // Inspect
}

impl RightModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_renderable_and_update(&self, frame_dt: u64) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); RIGHT_MODULE_COLS];
            RIGHT_MODULE_ROWS
        ];

        let dt_str = format!("Frame dt: {} ms", frame_dt);

        for (i, ch) in dt_str.chars().enumerate() {
            if Self::PADDING_LEFT + i < RIGHT_MODULE_COLS {
                content[1][Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BLUE);
            }
        }
        module_utility::add_frame("look", &mut content);
        content
    }
}
