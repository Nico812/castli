use std::collections::VecDeque;

use crate::ansi::*;
use crate::assets::*;
use super::module_utiltiy;

pub struct BottomModule {
    // Game events
}

impl BottomModule {
    const PADDING_LEFT: usize = 2;
    const CONTENT_ROWS: usize = 11;
    const CONTENT_COLS: usize = 64;

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
        let renderable = chatbox.into();
        module_utility::add_frame("chat", renderable);
        renderable
    }
}
