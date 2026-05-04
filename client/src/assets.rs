// In this game the renderables are all Vec<Vec<TermCell>>.
//
// Each asset has two version. One is used as a standard, the other as the cell where wind is present.
// This will proably change in the future.

use common::{
    courtyard::{Facility, FacilityType},
    map::Tile,
};
use crossterm::style::{Color, StyledContent, Stylize};

use crate::ansi::*;

#[derive(Copy, Clone, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl TermCell {
    pub const ERR: Self = Self {
        fg: MAGENTA,
        bg: BLACK,
        ch: '?',
    };

    pub const fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }

    pub fn printable(&self) -> StyledContent<char> {
        self.ch.with(self.fg).on(self.bg)
    }
}

pub struct TileAsset {
    pub fg: Color,
    pub bg: Color,
    pub std: TermCell,
    pub wind: TermCell,
}

impl TileAsset {
    pub const ERR: TileAsset = TileAsset {
        fg: MAGENTA,
        bg: MAGENTA,
        std: TermCell::ERR,
        wind: TermCell::new('!', WHITE, MAGENTA),
    };

    pub fn get_asset(tile: Tile, night: bool) -> Self {
        match tile {
            Tile::Grass => {
                if night {
                    NIGHT_GRASS
                } else {
                    DAY_GRASS
                }
            }
            Tile::Water => {
                if night {
                    NIGHT_WATER
                } else {
                    DAY_WATER
                }
            }
            Tile::Woods => {
                if night {
                    NIGHT_WOODS
                } else {
                    DAY_WOODS
                }
            }
            Tile::Mountain => {
                if night {
                    NIGHT_MOUNTAIN
                } else {
                    DAY_MOUNTAIN
                }
            }
            Tile::HighMountain => {
                if night {
                    NIGHT_HIGH_MOUNTAIN
                } else {
                    DAY_HIGH_MOUNTAIN
                }
            }
            Tile::Err => Self::ERR,
        }
    }
}

pub struct FacilityAsset;

impl FacilityAsset {
    pub fn get_asset(facility: &Facility, night: bool) -> &[&[TermCell]] {
        match facility.r#type {
            FacilityType::FarmPlot => {
                if night {
                    NIGHT_FARM_PLOT
                } else {
                    DAY_FARM_PLOT
                }
            }
            _ => ERR_ART,
        }
    }
}

// Misc

pub const BLOCK: char = '▀';

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', WHITE, BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', WHITE, BLACK);

pub const SELECTION_TERMCELL: TermCell = TermCell::new('<', BLACK, WHITE);

pub const BKG_EL: TermCell = TermCell::new('.', RED, BLACK);

// Tiles

pub const DAY_GRASS: TileAsset = TileAsset {
    fg: DAY_GREEN_1,
    bg: DAY_GREEN_1,
    std: TermCell::new(' ', DAY_GREEN_0, DAY_GREEN_1),
    wind: TermCell::new('\"', DAY_GREEN_0, DAY_GREEN_1),
};

pub const NIGHT_GRASS: TileAsset = TileAsset {
    fg: NIGHT_GREEN_1,
    bg: NIGHT_GREEN_1,
    std: TermCell::new(' ', NIGHT_GREEN_0, NIGHT_GREEN_1),
    wind: TermCell::new('\"', NIGHT_GREEN_0, NIGHT_GREEN_1),
};

pub const DAY_WATER: TileAsset = TileAsset {
    fg: DAY_BLUE_1,
    bg: DAY_BLUE_0,
    std: TermCell::new(' ', DAY_BLUE_0, DAY_BLUE_1),
    wind: TermCell::new('~', DAY_BLUE_0, DAY_BLUE_1),
};

pub const NIGHT_WATER: TileAsset = TileAsset {
    fg: NIGHT_BLUE_1,
    bg: NIGHT_BLUE_0,
    std: TermCell::new(' ', NIGHT_BLUE_0, NIGHT_BLUE_1),
    wind: TermCell::new('~', NIGHT_BLUE_0, NIGHT_BLUE_1),
};

pub const DAY_WOODS: TileAsset = TileAsset {
    fg: DAY_GREEN_2,
    bg: DAY_GREEN_2,
    std: TermCell::new(' ', DAY_GREEN_1, DAY_GREEN_2),
    wind: TermCell::new('"', DAY_GREEN_3, DAY_GREEN_2),
};

pub const NIGHT_WOODS: TileAsset = TileAsset {
    fg: NIGHT_GREEN_2,
    bg: NIGHT_GREEN_2,
    std: TermCell::new(' ', NIGHT_GREEN_1, NIGHT_GREEN_2),
    wind: TermCell::new('"', NIGHT_GREEN_3, NIGHT_GREEN_2),
};

pub const DAY_MOUNTAIN: TileAsset = TileAsset {
    fg: DAY_GREY_0,
    bg: DAY_GREY_2,
    std: TermCell::new('^', DAY_GREY_2, DAY_GREY_1),
    wind: TermCell::new('^', DAY_WHITE, DAY_GREY_1),
};

pub const NIGHT_MOUNTAIN: TileAsset = TileAsset {
    fg: NIGHT_GREY_0,
    bg: NIGHT_GREY_2,
    std: TermCell::new('^', NIGHT_GREY_2, NIGHT_GREY_1),
    wind: TermCell::new('^', NIGHT_WHITE, NIGHT_GREY_1),
};

pub const DAY_HIGH_MOUNTAIN: TileAsset = TileAsset {
    fg: DAY_WHITE,
    bg: DAY_WHITE,
    std: TermCell::new(' ', DAY_WHITE, DAY_WHITE),
    wind: TermCell::new('^', DAY_BLUE_0, DAY_WHITE),
};

pub const NIGHT_HIGH_MOUNTAIN: TileAsset = TileAsset {
    fg: NIGHT_WHITE,
    bg: NIGHT_WHITE,
    std: TermCell::new(' ', NIGHT_WHITE, NIGHT_WHITE),
    wind: TermCell::new('^', NIGHT_BLUE_0, NIGHT_WHITE),
};

// Game elements

pub const MY_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', GREEN, BLACK)]];
pub const CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('@', WHITE, BLACK)]];
pub const DEAD_CASTLE_ART: &[&[TermCell]] = &[&[TermCell::new('X', RED, BLACK)]];
pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());

pub const MY_DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', GREEN, BLACK)]];
pub const DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('u', WHITE, BLACK)]];
pub const DEPLOYED_UNITS_ART_SIZE: (usize, usize) =
    (DEPLOYED_UNITS_ART.len(), DEPLOYED_UNITS_ART[0].len());

pub const ERR_ART: &[&[TermCell]] = &[&[TermCell::ERR]];
pub const ERR_ART_SIZE: (usize, usize) = (ERR_ART.len(), ERR_ART[0].len());

// Facilities

// +++++++++
// +▓▒▓▓░░▒+
// +░▓░▓░▓▓+
// +▒░▒▒▓▓░+
// +++++++++
pub const DAY_FARM_PLOT: &[&[TermCell]] = &[
    &[
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
    ],
    &[
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▒', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▒', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('+', DAY_BROWN, BLACK),
    ],
    &[
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('+', DAY_BROWN, BLACK),
    ],
    &[
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('▒', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▒', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▒', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('▓', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('░', DAY_GREEN_0, DAY_GREEN_1),
        TermCell::new('+', DAY_BROWN, BLACK),
    ],
    &[
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
        TermCell::new('+', DAY_BROWN, BLACK),
    ],
];

pub const NIGHT_FARM_PLOT: &[&[TermCell]] = &[
    &[
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
    ],
    &[
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▒', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▒', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
    ],
    &[
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
    ],
    &[
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▒', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▒', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▒', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('▓', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('░', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
    ],
    &[
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
        TermCell::new('+', NIGHT_GREEN_0, NIGHT_GREEN_1),
    ],
];

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
