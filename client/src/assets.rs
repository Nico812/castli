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

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', FG_WHITE, BG_BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', FG_WHITE, BG_BLACK);
// pub const CURSOR_UP: TermCell = TermCell::new('\u{27B9}', FG_WHITE, BG_BLACK);
// pub const CURSOR_DOWN: TermCell = TermCell::new('\u{27B7}', FG_WHITE, BG_BLACK);

pub const ERR_FG: &str = FG_MAGENTA;
pub const ERR_BG: &str = BG_MAGENTA_BRIGHT;
pub const ERR_EL: TermCell = TermCell::new('?', FG_MAGENTA, ERR_BG);
pub const BKG_FG: &str = FG_BLACK;
pub const BKG_BG: &str = BG_BLACK;
pub const BKG_EL: TermCell = TermCell::new('.', FG_RED, BG_BLACK);

pub const GRASS_FG: &str = FG_GREEN;
pub const GRASS_BG: &str = BG_GREEN;
pub const GRASS_EL_1: TermCell = TermCell::new(' ', FG_GREEN_BRIGHT, GRASS_BG);
pub const GRASS_EL_2: TermCell = TermCell::new('\"', FG_GREEN_BRIGHT, GRASS_BG);

pub const WATER_FG: &str = FG_BLUE;
pub const WATER_BG: &str = BG_BLUE;
pub const WATER_EL_1: TermCell = TermCell::new(' ', FG_BLUE_BRIGHT, WATER_BG);
pub const WATER_EL_2: TermCell = TermCell::new('~', FG_BLUE_BRIGHT, WATER_BG);

pub const WOODS_FG: &str = FG_GREEN_DARK;
pub const WOODS_BG: &str = BG_GREEN_DARK;
pub const WOODS_EL_1: TermCell = TermCell::new(' ', FG_GREEN, WOODS_BG);
pub const WOODS_EL_2: TermCell = TermCell::new('#', FG_WHITE, WOODS_BG);

pub const MOUNTAIN_FG: &str = FG_GREEN;
pub const MOUNTAIN_BG: &str = BG_GREEN;
pub const MOUNTAIN_EL_1: TermCell = TermCell::new(' ', FG_RED_BRIGHT, MOUNTAIN_BG);
pub const MOUNTAIN_EL_2: TermCell = TermCell::new('#', FG_WHITE, MOUNTAIN_BG);

pub const HIGH_MOUNTAIN_FG: &str = FG_GREEN;
pub const HIGH_MOUNTAIN_BG: &str = BG_GREEN;
pub const HIGH_MOUNTAIN_EL_1: TermCell = TermCell::new('#', FG_RED_BRIGHT, HIGH_MOUNTAIN_BG);
pub const HIGH_MOUNTAIN_EL_2: TermCell = TermCell::new('#', FG_YELLOW_BRIGHT, HIGH_MOUNTAIN_BG);

pub const CASTLE_FG: &str = FG_WHITE;
pub const CASTLE_BG: &str = BG_BLACK;

pub const ERR_ART: &[&[TermCell]] = &[&[ERR_EL]];

// pub const CASTLE_ART: &[&[TermCell]] = &[
//     &[
//         TermCell::new('M', CASTLE_FG, CASTLE_BG),
//         TermCell::new('M', CASTLE_FG, CASTLE_BG),
//         TermCell::new('_', CASTLE_FG, CASTLE_BG),
//         TermCell::new('M', CASTLE_FG, CASTLE_BG),
//         TermCell::new('_', CASTLE_FG, CASTLE_BG),
//         TermCell::new('M', CASTLE_FG, CASTLE_BG),
//         TermCell::new('_', CASTLE_FG, CASTLE_BG),
//         TermCell::new('M', CASTLE_FG, CASTLE_BG),
//     ],
//     &[
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//     ],
//     &[
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new('_', CASTLE_FG, CASTLE_BG),
//         TermCell::new('_', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//     ],
//     &[
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//         TermCell::new(' ', CASTLE_FG, CASTLE_BG),
//         TermCell::new('|', CASTLE_FG, CASTLE_BG),
//     ],
// ];
//
// pub const CASTLE_ART_WORLD: &[&[TermCell]] = &[&[
//     TermCell::new('C', FG_YELLOW, BG_BLACK),
//     TermCell::new('C', FG_YELLOW, BG_BLACK),
// ]];

pub const CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', FG_WHITE_BRIGHT, BG_BLACK)]];

pub const CASTLE_ART_WORLD: &[&[TermCell]] = &[&[TermCell::new('@', FG_WHITE_BRIGHT, BG_BLACK)]];

pub const DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', FG_RED, BG_BLACK)]];

pub const ERR_ART_SIZE: (usize, usize) = (ERR_ART.len(), ERR_ART[0].len());

pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());

pub const CASTLE_ART_WORLD_SIZE: (usize, usize) =
    (CASTLE_ART_WORLD.len(), CASTLE_ART_WORLD[0].len());

pub const DEPLOYED_UNITS_ART_SIZE: (usize, usize) =
    (DEPLOYED_UNITS_ART.len(), DEPLOYED_UNITS_ART[0].len());
