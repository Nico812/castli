use common::exports::game_object::GameObjE;

use super::cell::*;

pub const CURSOR_UP: TermCell = TermCell::new('\u{21B1}', FG_WHITE, BG_BLACK);
pub const CURSOR_DOWN: TermCell = TermCell::new('\u{21B3}', FG_WHITE, BG_BLACK);

pub const ERR_FG: &str = FG_MAGENTA;
pub const ERR_BG: &str = BG_BRIGHT_MAGENTA;
pub const ERR_EL: TermCell = TermCell::new('?', FG_MAGENTA, BG_BRIGHT_MAGENTA);
pub const BKG_EL: TermCell = TermCell::new('.', FG_RED, BG_BLACK);

pub const GRASS_FG: &str = FG_GREEN;
pub const GRASS_BG: &str = BG_GREEN;
pub const GRASS_EL_1: TermCell = TermCell::new(' ', FG_BRIGHT_GREEN, GRASS_BG);
pub const GRASS_EL_2: TermCell = TermCell::new('"', FG_BRIGHT_GREEN, GRASS_BG);

pub const WATER_FG: &str = FG_BLUE;
pub const WATER_BG: &str = BG_BLUE;
pub const WATER_EL_1: TermCell = TermCell::new(' ', FG_BRIGHT_BLUE, WATER_BG);
pub const WATER_EL_2: TermCell = TermCell::new('~', FG_BRIGHT_BLUE, WATER_BG);

pub const CASTLE_FG: &str = FG_WHITE;
pub const CASTLE_BG: &str = BG_BLACK;

pub const ERR_ART: &[&[TermCell]] = &[&[ERR_EL]];

pub const CASTLE_ART: &[&[TermCell]] = &[
    &[
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('M', CASTLE_FG, CASTLE_BG),
    ],
    &[
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    &[
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new('_', CASTLE_FG, CASTLE_BG),
        TermCell::new(' ', CASTLE_FG, CASTLE_BG),
        TermCell::new('|', CASTLE_FG, CASTLE_BG),
    ],
    &[
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

pub const CASTLE_ART_WORLD: &[&[TermCell]] = &[&[
    TermCell::new('C', FG_YELLOW, BG_BLACK),
    TermCell::new('C', FG_YELLOW, BG_BLACK),
]];

pub const DEPLOYED_UNITS_ART: &[&[TermCell]] = &[&[TermCell::new('U', FG_RED, BG_BLACK)]];

pub const ERR_ART_SIZE: (usize, usize) = (ERR_ART.len(), ERR_ART[0].len());
pub const CASTLE_ART_SIZE: (usize, usize) = (CASTLE_ART.len(), CASTLE_ART[0].len());
pub const CASTLE_ART_WORLD_SIZE: (usize, usize) =
    (CASTLE_ART_WORLD.len(), CASTLE_ART_WORLD[0].len());
pub const DEPLOYED_UNITS_ART_SIZE: (usize, usize) =
    (DEPLOYED_UNITS_ART.len(), DEPLOYED_UNITS_ART[0].len());

pub trait WithArt {
    fn get_art(&self, world: bool) -> &[&[TermCell]];
    fn get_art_size(&self, world: bool) -> (usize, usize);
}

impl WithArt for GameObjE {
    fn get_art(&self, world: bool) -> &[&[TermCell]] {
        match self {
            Self::Castle(_) => {
                if world {
                    CASTLE_ART_WORLD
                } else {
                    CASTLE_ART
                }
            }
            Self::DeployedUnits(_) => DEPLOYED_UNITS_ART,
            _ => ERR_ART,
        }
    }

    fn get_art_size(&self, world: bool) -> (usize, usize) {
        match self {
            Self::Castle(_) => {
                if world {
                    CASTLE_ART_WORLD_SIZE
                } else {
                    CASTLE_ART_SIZE
                }
            }
            Self::DeployedUnits(_) => DEPLOYED_UNITS_ART_SIZE,
            _ => ERR_ART_SIZE,
        }
    }
}
