// In this game the renderables are all Vec<Vec<TermCell>>.
//
// Each asset has two version. One is used as a standard, the other as the cell where wind is present.
// This will proably change in the future.

#![allow(dead_code)]

use common::exports::tile::TileE;

use crate::ansi::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: &'static str,
    pub bg: &'static str,
}

impl TermCell {
    pub const fn new(ch: char, fg: &'static str, bg: &'static str) -> Self {
        Self { ch, fg, bg }
    }

    pub fn as_string(&self) -> String {
        format!("{}{}{}", self.fg, self.bg, self.ch)
    }
}

// Misc

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', FG_WHITE, BG_BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', FG_WHITE, BG_BLACK);

pub const SELECTION_TERMCELL: TermCell = TermCell::new('<', FG_BLACK, BG_WHITE);

pub const BKG_FG: &str = FG_BLACK;
pub const BKG_BG: &str = BG_BLACK;
pub const BKG_EL: TermCell = TermCell::new('.', FG_RED, BG_BLACK);

// Tiles

pub struct TileAsset {
    pub fg: &'static str,
    pub bg: &'static str,
    pub std: TermCell,
    pub wind: TermCell,
}

impl TileAsset {
    pub fn get_asset(tile: TileE) -> Self {
        match tile {
            TileE::Grass => GRASS,
            TileE::Water => WATER,
            TileE::Woods => WOODS,
            TileE::Mountain => MOUNTAIN,
            TileE::HighMountain => HIGH_MOUNTAIN,
            TileE::Err => ERR,
        }
    }
}

pub const GRASS: TileAsset = TileAsset {
    fg: FG_GREEN,
    bg: BG_GREEN,
    std: TermCell::new(' ', FG_GREEN_BRIGHT, BG_GREEN),
    wind: TermCell::new('\"', FG_GREEN_BRIGHT, BG_GREEN),
};

pub const WATER: TileAsset = TileAsset {
    fg: FG_BLUE,
    bg: BG_BLUE_BRIGHT,
    std: TermCell::new(' ', FG_BLUE_BRIGHT, BG_BLUE),
    wind: TermCell::new('~', FG_BLUE_BRIGHT, BG_BLUE),
};

pub const WOODS: TileAsset = TileAsset {
    fg: FG_GREEN_DARK,
    bg: BG_GREEN_DARK,
    std: TermCell::new(' ', FG_GREEN, BG_GREEN_DARK),
    wind: TermCell::new('"', FG_GREEN_DARKER, BG_GREEN_DARK),
};

pub const MOUNTAIN: TileAsset = TileAsset {
    fg: FG_GREY_BRIGHT,
    bg: BG_GREY,
    std: TermCell::new('^', FG_GREY, BG_GREY_GREENISH),
    wind: TermCell::new('^', FG_WHITE, BG_GREY_GREENISH),
};

pub const HIGH_MOUNTAIN: TileAsset = TileAsset {
    fg: FG_WHITE,
    bg: BG_WHITE,
    std: TermCell::new(' ', FG_WHITE, BG_WHITE),
    wind: TermCell::new('^', FG_BLUE_BRIGHT, BG_WHITE),
};

pub const ERR: TileAsset = TileAsset {
    fg: FG_MAGENTA,
    bg: BG_MAGENTA,
    std: TermCell::new('?', FG_WHITE, BG_MAGENTA),
    wind: TermCell::new('!', FG_WHITE, BG_MAGENTA),
};

// Game elements

pub const MY_CASTLE_FG: &str = FG_WHITE;
pub const MY_CASTLE_BG: &str = BG_BLACK;
pub const MY_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', FG_GREEN, BG_BLACK)]];
pub const CASTLE_FG: &str = FG_BLACK;
pub const CASTLE_BG: &str = BG_WHITE;
pub const CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', FG_WHITE, BG_BLACK)]];
pub const DEAD_CASTLE_FG: &str = FG_BLACK;
pub const DEAD_CASTLE_BG: &str = BG_WHITE;
pub const DEAD_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('X', FG_RED, BG_BLACK)]];
pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());

pub const MY_DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', FG_GREEN, BG_BLACK)]];
pub const DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', FG_WHITE, BG_BLACK)]];
pub const DEPLOYED_UNITS_ART_SIZE: (usize, usize) =
    (DEPLOYED_UNITS_ART.len(), DEPLOYED_UNITS_ART[0].len());

pub const ERR_ART: &[&[TermCell]] = &[&[ERR.std]];
pub const ERR_ART_SIZE: (usize, usize) = (ERR_ART.len(), ERR_ART[0].len());

// Old assets
//
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
