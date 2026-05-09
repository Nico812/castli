use crossterm::style::Color;

use crate::{
    ansi::{BLACK, DAY_GREEN_0},
    assets::TermCell,
};

pub const FRAME_BK_COLOR: Color = BLACK;
pub const MOD_BK_COLOR: Color = BLACK;
pub const COURTYARD_BK_CELL: TermCell = TermCell::new('.', DAY_GREEN_0, BLACK);
