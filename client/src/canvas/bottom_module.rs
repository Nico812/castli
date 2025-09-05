use std::collections::VecDeque;

use super::{
    r#const::{BOTTOM_MODULE_COLS, BOTTOM_MODULE_ROWS},
    module_utility,
};
use crate::ansi::*;
use crate::assets::*;

pub struct BottomModule {
    // Game events
}

impl BottomModule {
    const PADDING_LEFT: usize = 2;
    const CONTENT_ROWS: usize = BOTTOM_MODULE_ROWS - 2;
    const CONTENT_COLS: usize = BOTTOM_MODULE_COLS - 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_renderable_and_update(&self, logs: &mut VecDeque<String>) -> Vec<Vec<TermCell>> {
        let mut chatbox: VecDeque<Vec<TermCell>> = VecDeque::with_capacity(Self::CONTENT_ROWS);

        for _ in 0..Self::CONTENT_ROWS {
            chatbox.push_back(vec![
                TermCell::new(' ', FG_BLACK, BG_BLACK);
                Self::CONTENT_COLS
            ]);
        }

        for log in logs {
            chatbox.pop_front();
            let mut row = vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            for (i, ch) in log.chars().enumerate() {
                if i < Self::CONTENT_COLS - Self::PADDING_LEFT {
                    row[Self::PADDING_LEFT + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
                }
            }
            chatbox.push_back(row);
        }
        let mut renderable = chatbox.into();
        module_utility::add_frame("chat", &mut renderable);
        renderable
    }
}
