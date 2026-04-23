// In this game the renderables are all Vec<Vec<TermCell>>.
//
// Each asset has two version. One is used as a standard, the other as the cell where wind is present.
// This will proably change in the future.

#![allow(dead_code)]

use common::exports::tile::TileE;
use crossterm::style::{Color, StyledContent, Stylize};

use crate::ansi::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl TermCell {
    pub const fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }

    pub fn printable(&self) -> StyledContent<char> {
        self.ch.with(self.fg).on(self.bg)
    }
}

// Misc
pub const BLOCK: char = '▀';

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', WHITE, BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', WHITE, BLACK);

pub const SELECTION_TERMCELL: TermCell = TermCell::new('<', BLACK, WHITE);

pub const BKG_FG: Color = BLACK;
pub const BKG_BG: Color = BLACK;
pub const BKG_EL: TermCell = TermCell::new('.', RED, BLACK);

// Tiles
pub struct TileAsset {
    pub fg: Color,
    pub bg: Color,
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
    fg: GREEN,
    bg: GREEN,
    std: TermCell::new(' ', GREEN_BRIGHT, GREEN),
    wind: TermCell::new('\"', GREEN_BRIGHT, GREEN),
};

pub const WATER: TileAsset = TileAsset {
    fg: BLUE,
    bg: BLUE_BRIGHT,
    std: TermCell::new(' ', BLUE_BRIGHT, BLUE),
    wind: TermCell::new('~', BLUE_BRIGHT, BLUE),
};

pub const WOODS: TileAsset = TileAsset {
    fg: GREEN_DARK,
    bg: GREEN_DARK,
    std: TermCell::new(' ', GREEN, GREEN_DARK),
    wind: TermCell::new('"', GREEN_DARKER, GREEN_DARK),
};

pub const MOUNTAIN: TileAsset = TileAsset {
    fg: GREY_BRIGHT,
    bg: GREY,
    std: TermCell::new('^', GREY, GREY_GREENISH),
    wind: TermCell::new('^', WHITE, GREY_GREENISH),
};

pub const HIGH_MOUNTAIN: TileAsset = TileAsset {
    fg: WHITE,
    bg: WHITE,
    std: TermCell::new(' ', WHITE, WHITE),
    wind: TermCell::new('^', BLUE_BRIGHT, WHITE),
};

pub const ERR: TileAsset = TileAsset {
    fg: MAGENTA,
    bg: MAGENTA,
    std: TermCell::new('?', WHITE, MAGENTA),
    wind: TermCell::new('!', WHITE, MAGENTA),
};

// Game elements

pub const MY_CASTLE_FG: Color = WHITE;
pub const MY_CASTLE_BG: Color = BLACK;
pub const MY_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', GREEN, BLACK)]];
pub const CASTLE_FG: Color = BLACK;
pub const CASTLE_BG: Color = WHITE;
pub const CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', WHITE, BLACK)]];
pub const DEAD_CASTLE_FG: Color = BLACK;
pub const DEAD_CASTLE_BG: Color = WHITE;
pub const DEAD_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('X', RED, BLACK)]];
pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());

pub const MY_DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', GREEN, BLACK)]];
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
