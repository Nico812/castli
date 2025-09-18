//! # Game Asset Definitions
//!
//! This module serves as a central "sprite sheet" for the terminal application,
//! defining all static visual elements.
//!
//! The fundamental building block for all art is the `TermCell` struct, which
//! represents a single character cell with a specific glyph, foreground color, and
//! background color.

#![allow(dead_code)]

use crate::ansi::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: &'static str,
    pub bg: &'static str,
}

impl<'a> TermCell {
    pub const fn new(ch: char, fg: &'static str, bg: &'static str) -> Self {
        Self { ch, fg, bg }
    }

    pub fn as_string(&self) -> String {
        format!("{}{}{}", self.fg, self.bg, self.ch)
    }
}

// --- ANSI Game Elements ---

pub const ERR_FG: &str = FG_MAGENTA;
pub const ERR_BG: &str = BG_BRIGHT_MAGENTA;
pub const ERR_EL: TermCell = TermCell::new('?', FG_MAGENTA, BG_BRIGHT_MAGENTA);
pub const BKG_FG: &str = FG_BLACK;
pub const BKG_BG: &str = BG_BLACK;
pub const BKG_EL: TermCell = TermCell::new('.', FG_RED, BG_BLACK);

pub const GRASS_FG: &str = FG_GREEN;
pub const GRASS_BG: &str = BG_GREEN;
pub const GRASS_EL_1: TermCell = TermCell::new(' ', FG_BRIGHT_GREEN, GRASS_BG);
pub const GRASS_EL_2: TermCell = TermCell::new('\"', FG_BRIGHT_GREEN, GRASS_BG);

pub const WATER_FG: &str = FG_BLUE;
pub const WATER_BG: &str = BG_BLUE;
pub const WATER_EL_1: TermCell = TermCell::new(' ', FG_BRIGHT_BLUE, WATER_BG);
pub const WATER_EL_2: TermCell = TermCell::new('~', FG_BRIGHT_BLUE, WATER_BG);

pub const CASTLE_FG: &str = FG_WHITE;
pub const CASTLE_BG: &str = BG_BLACK;

pub const CASTLE_ART: [[TermCell; 8]; 4] = [
    [
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    [
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
];

pub const CASTLE_ART_WORLD: [[TermCell; 2]; 1] = [[
    TermCell::new('C', FG_YELLOW, BG_BLACK),
    TermCell::new('C', FG_YELLOW, BG_BLACK),
]];

pub const UNIT_GROUP_ART: [[TermCell; 1]; 1] = [[TermCell::new('U', FG_RED, BG_BLACK)]];
