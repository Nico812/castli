use common::exports::game_object::CastleE;
use common::exports::game_object::DeployedUnitsE;
use common::exports::game_object::GameObjE;

use crate::ansi::*;
use crate::assets::*;

pub trait WithArt {
    fn get_art(&self, world: bool, owned: bool) -> &[&[TermCell]];
    fn get_art_size(&self, world: bool) -> (usize, usize);
}

impl WithArt for GameObjE {
    fn get_art(&self, world: bool, owned: bool) -> &[&[TermCell]] {
        match self {
            Self::Castle(castle) => castle.get_art(world, owned),
            Self::DeployedUnits(units) => units.get_art(world, owned),
            _ => ERR_ART,
        }
    }

    fn get_art_size(&self, world: bool) -> (usize, usize) {
        match self {
            Self::Castle(castle) => castle.get_art_size(world),
            Self::DeployedUnits(units) => units.get_art_size(world),
            _ => ERR_ART_SIZE,
        }
    }
}

impl WithArt for CastleE {
    fn get_art(&self, _world: bool, owned: bool) -> &[&[TermCell]] {
        if !self.alive {
            DEAD_CASTLE_ART
        } else if owned {
            MY_CASTLE_ART
        } else {
            CASTLE_ART
        }
    }
    fn get_art_size(&self, _world: bool) -> (usize, usize) {
        CASTLE_ART_SIZE
    }
}

impl WithArt for DeployedUnitsE {
    fn get_art(&self, _world: bool, owned: bool) -> &[&[TermCell]] {
        if owned {
            MY_DEPLOYED_UNITS_ART
        } else {
            DEPLOYED_UNITS_ART
        }
    }
    fn get_art_size(&self, _world: bool) -> (usize, usize) {
        DEPLOYED_UNITS_ART_SIZE
    }
}

pub fn add_frame(title: &str, renderable: &mut Vec<Vec<TermCell>>) {
    let renderable_rows = renderable.len();
    let renderable_cols = renderable[0].len();

    let bot_row = vec![TermCell::new('-', WHITE, BLACK); renderable_cols];

    let mut top_row = bot_row.clone();
    for (pos, char) in title.chars().enumerate() {
        if pos + 2 < renderable_cols {
            top_row[pos + 2] = TermCell::new(char, WHITE, BLACK);
        }
    }

    renderable.insert(0, top_row);
    renderable.push(bot_row);

    for (pos, rend_col) in renderable.iter_mut().enumerate() {
        let cell = if pos == 0 || pos == renderable_rows + 1 {
            TermCell::new('+', WHITE, BLACK)
        } else {
            TermCell::new('|', WHITE, BLACK)
        };
        rend_col.insert(0, cell);
        rend_col.push(cell);
    }
}

// TODO: make it steal the string, not borrow.
pub fn draw_text_in_row(
    content: &mut [Vec<TermCell>],
    string: &str,
    row: usize,
    pad_left: usize,
    pad_right: usize,
) {
    if row >= content.len() {
        return;
    }
    let space_available = content[0].len().saturating_sub(pad_right + pad_left);

    for (i, ch) in string.chars().enumerate() {
        if i < space_available {
            content[row][pad_left + i] = TermCell {
                ch,
                fg: WHITE,
                bg: BLACK,
            }
        }
    }
}
