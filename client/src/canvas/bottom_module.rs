use crate::canvas::r#const::*;
use crate::ansi::*;
use crate::assets::*;

pub struct BottomModule {
    // Game events
}


impl BottomModule {
    const PADDING_LEFT: usize = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_content(&self) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); BOTTOM_MODULE_COLS];
            BOTTOM_MODULE_ROWS
        ];
        content
    }
}