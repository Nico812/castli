use common::exports::game_object::GameObjE;

use crate::ansi::*;
use crate::assets::*;

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

pub fn add_frame(title: &str, renderable: &mut Vec<Vec<TermCell>>) {
    let renderable_rows = renderable.len();
    let renderable_cols = renderable[0].len();

    let bot_row = vec![TermCell::new('-', FG_WHITE, BG_BLACK); renderable_cols];

    let mut top_row = bot_row.clone();
    for (pos, char) in title.chars().enumerate() {
        if pos + 2 < renderable_cols {
            top_row[pos + 2] = TermCell::new(char, FG_WHITE, BG_BLACK);
        }
    }

    renderable.insert(0, top_row);
    renderable.push(bot_row);

    for (pos, rend_col) in renderable.iter_mut().enumerate() {
        let cell;
        if pos == 0 || pos == renderable_rows + 1 {
            cell = TermCell::new('+', FG_WHITE, BG_BLACK);
        } else {
            cell = TermCell::new('|', FG_WHITE, BG_BLACK);
        }
        rend_col.insert(0, cell);
        rend_col.push(cell);
    }
}

pub fn draw_text(content: &mut Vec<Vec<TermCell>>, text: &str, row: usize, col: usize) {
    if row >= content.len() {
        return;
    }
    for (i, ch) in text.chars().enumerate() {
        if col + i < content[row].len() {
            content[row][col + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
        }
    }
}
