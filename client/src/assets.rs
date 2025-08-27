//! # Game Asset Definitions
//!
//! This module serves as a central "sprite sheet" for the terminal application,
//! defining all static visual elements.
//!
//! The fundamental building block for all art is the `TermCell` struct, which
//! represents a single character cell with a specific glyph, foreground color, and
//! background color.

use crate::ansi::*;

#[derive(Copy, Clone, Debug)]
pub struct TermCell<'a> {
    pub ch: char,
    pub fg: &'a str,
    pub bg: &'a str,
}

impl<'a> TermCell<'a> {
    pub const fn new(ch: char, fg: &'a str, bg: &'a str) -> Self {
        Self { ch, fg, bg }
    }

    pub fn as_string(&self) -> &str {
        format!("{}{}{}", self.fg, self.bg, self.ch)
    }
}

// === ANSI GAME ELEMENTS ===
pub const ERR_FG: &str = FG_MAGENTA;
pub const ERR_BG: &str = BG_BRIGHT_MAGENTA;
pub const ERR_EL: TermCell<'static> = TermCell::new('?', FG_MAGENTA, BG_BRIGHT_MAGENTA);
pub const BKG_EL: TermCell<'static> = TermCell::new('_', FG_BRIGHT_BLACK, BG_BLACK);

pub const GRASS_FG: &str = FG_GREEN;
pub const GRASS_BG: &str = BG_GREEN;
pub const GRASS_EL_1: TermCell<'static> = TermCell::new('\"', FG_BRIGHT_GREEN, GRASS_BG);
pub const GRASS_EL_2: TermCell<'static> = TermCell::new(' ', FG_BRIGHT_GREEN, GRASS_BG);

pub const WATER_FG: &str = FG_BLUE;
pub const WATER_BG: &str = BG_BLUE;
pub const WATER_EL_1: TermCell<'static> = TermCell::new('~', FG_BRIGHT_BLUE, WATER_BG);
pub const WATER_EL_2: TermCell<'static> = TermCell::new(' ', FG_BRIGHT_BLUE, WATER_BG);

pub const CASTLE_FG: &str = FG_WHITE;
pub const CASTLE_BG: &str = BG_BLACK;

pub const CASTLE_ART: [[TermCell<'static>; 8]; 4] = [
    [
        TermCell::new('M', CASTLE_FG, CASTLE_BG), TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG), TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG), TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG), TermCell::new('M', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG), TermCell::new('▒', CASTLE_FG, CASTLE_BG),
        TermCell::new('▒', CASTLE_FG, CASTLE_BG), TermCell::new('░', CASTLE_FG, CASTLE_BG),
        TermCell::new('░', CASTLE_FG, CASTLE_BG), TermCell::new('░', CASTLE_FG, CASTLE_BG),
        TermCell::new('░', CASTLE_FG, CASTLE_BG), TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG), TermCell::new('▒', CASTLE_FG, CASTLE_BG),
        TermCell::new('▒', CASTLE_FG, CASTLE_BG), TermCell::new('X', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG), TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('░', CASTLE_FG, CASTLE_BG), TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG), TermCell::new('▒', CASTLE_FG, CASTLE_BG),
        TermCell::new('▒', CASTLE_FG, CASTLE_BG), TermCell::new('░', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG), TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('░', CASTLE_FG, CASTLE_BG), TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
];

pub const CASTLE_ART_WORLD: [[TermCell<'static>; 2]; 1] = [[
    TermCell::new('C', FG_YELLOW, BG_BLACK),
    TermCell::new('C', FG_YELLOW, BG_BLACK),
]];