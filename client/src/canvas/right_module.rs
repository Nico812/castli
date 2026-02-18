use common::GameCoord;
use common::exports::game_object::GameObjE;

use super::{r#const::*, module_utility};
use crate::ansi::*;
use crate::assets::*;

enum RightModuleTab {
    Castle,
    Debug,
}

pub struct RightModule {
    current_tab: RightModuleTab,
}

impl RightModule {
    const PADDING_HORI: usize = 2;
    const PADDING_VERT: usize = 1;
    const CONTENT_ROWS: usize = RIGHT_MODULE_ROWS.saturating_sub(Self::PADDING_VERT * 2);
    const CONTENT_COLS: usize = RIGHT_MODULE_COLS.saturating_sub(Self::PADDING_HORI * 2);

    pub fn new() -> Self {
        let current_tab = RightModuleTab::Castle;
        Self { current_tab }
    }

    // TODO: refactor using the utility function defined at the end of module_utility.rs
    pub fn get_renderable_and_update(
        &self,
        frame_dt: u64,
        look_pos: Option<GameCoord>,
    ) -> Vec<Vec<TermCell>> {
        let mut content = vec![
            vec![TermCell::new(' ', FG_BLACK, BG_BLACK); Self::CONTENT_COLS];
            Self::CONTENT_ROWS
        ];

        match self.current_tab {
            RightModuleTab::Castle => Self::add_castle_tab(&mut content),
            RightModuleTab::Debug => Self::add_debug_tab(&mut content, frame_dt, look_pos),
        };
        module_utility::add_frame("inspect", &mut content);
        content
    }

    fn add_debug_tab(content: &mut Vec<Vec<TermCell>>, frame_dt: u64, look_pos: Option<GameCoord>) {
        // Show FPS
        let dt_str = format!("Frame dt: {} ms", frame_dt);
        module_utility::draw_text_in_row(
            content,
            &dt_str,
            1,
            Self::PADDING_HORI,
            Self::PADDING_HORI,
        );

        // Show looking coordinates
        if let Some(pos) = look_pos {
            let look_pos_str = format!("Looking at ({}, {})", pos.y, pos.x);
            module_utility::draw_text_in_row(
                content,
                &look_pos_str,
                3,
                Self::PADDING_HORI,
                Self::PADDING_HORI,
            );
        }
    }

    fn add_castle_tab(content: &mut Vec<Vec<TermCell>>) {}
}
