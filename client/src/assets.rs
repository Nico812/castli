// In this game the renderables are all Vec<Vec<TermCell>>.
//
// Each asset has two version. One is used as a standard, the other as the cell where wind is present.
// This will proably change in the future.

#![allow(dead_code)]

use common::exports::tile::TileE;
use crossterm::style::{Color, StyledContent, Stylize};

use crate::ansi::*;

#[derive(Copy, Clone, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: DynamicColor,
    pub bg: DynamicColor,
}

impl TermCell {
    pub const fn new(ch: char, fg: DynamicColor, bg: DynamicColor) -> Self {
        Self { ch, fg, bg }
    }

    pub fn printable(&self, night: bool) -> StyledContent<char> {
        self.ch.with(self.fg.get(night)).on(self.bg.get(night))
    }
}

pub struct TileAsset {
    pub fg: DynamicColor,
    pub bg: DynamicColor,
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

// Misc
pub const BLOCK: char = '▀';

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', WHITE, BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', WHITE, BLACK);

pub const SELECTION_TERMCELL: TermCell = TermCell::new('<', BLACK, WHITE);

pub const BKG_FG: DynamicColor = BLACK;
pub const BKG_BG: DynamicColor = BLACK;
pub const BKG_EL: TermCell = TermCell::new('.', RED, BLACK);

// Tiles
pub const GRASS: TileAsset = TileAsset {
    fg: GREEN_1,
    bg: GREEN_1,
    std: TermCell::new(' ', GREEN_0, GREEN_1),
    wind: TermCell::new('\"', GREEN_0, GREEN_1),
};

pub const WATER: TileAsset = TileAsset {
    fg: BLUE_1,
    bg: BLUE_0,
    std: TermCell::new(' ', BLUE_0, BLUE_1),
    wind: TermCell::new('~', BLUE_0, BLUE_1),
};

pub const WOODS: TileAsset = TileAsset {
    fg: GREEN_2,
    bg: GREEN_2,
    std: TermCell::new(' ', GREEN_1, GREEN_2),
    wind: TermCell::new('"', GREEN_3, GREEN_2),
};

pub const MOUNTAIN: TileAsset = TileAsset {
    fg: GREY_0,
    bg: GREY_2,
    std: TermCell::new('^', GREY_2, GREY_1),
    wind: TermCell::new('^', WHITE, GREY_1),
};

pub const HIGH_MOUNTAIN: TileAsset = TileAsset {
    fg: WHITE,
    bg: WHITE,
    std: TermCell::new(' ', WHITE, WHITE),
    wind: TermCell::new('^', BLUE_0, WHITE),
};

pub const ERR: TileAsset = TileAsset {
    fg: MAGENTA,
    bg: MAGENTA,
    std: TermCell::new('?', WHITE, MAGENTA),
    wind: TermCell::new('!', WHITE, MAGENTA),
};

// Game elements

pub const MY_CASTLE_FG: DynamicColor = WHITE;
pub const MY_CASTLE_BG: DynamicColor = BLACK;
pub const MY_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', GREEN_1, BLACK)]];
pub const CASTLE_FG: DynamicColor = BLACK;
pub const CASTLE_BG: DynamicColor = WHITE;
pub const CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', WHITE, BLACK)]];
pub const DEAD_CASTLE_FG: DynamicColor = BLACK;
pub const DEAD_CASTLE_BG: DynamicColor = WHITE;
pub const DEAD_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('X', RED, BLACK)]];
pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());

pub const MY_DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', GREEN_1, BLACK)]];
pub const DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', WHITE, BLACK)]];
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
//     TermCell::new('C', YELLOW, BLACK),
//     TermCell::new('C', YELLOW, BLACK),
// ]];
