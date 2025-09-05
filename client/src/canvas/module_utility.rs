use crate::ansi::*;
use crate::assets::*;

fn add_frame(title: &str, renderable: &mut Vec<Vec<TermCell>>) {
    let renderable_rows = renderable.len();
    let renderable_cols = renderable[0].len();

    let bot_row = vec![TermCell::new('-', FG_WHITE, BG_BLACK); renderable_cols];

    let top_row = bot_row;
    for (pos, char) in title.chars().enumerate(){
        if pos+2 < renderable_cols {
            top_row[pos+2] = TermCell::new(char, FG_WHITE, BG_BLACK);
        }   
    }

    renderable.insert(0, top_row);
    renderable.push_back(bot_row);

    for (pos, rend_col) in renderable.iter_mut().enumerate() {
        let cell;
        if (pos == 0 || pos == renderable_rows - 1) {
            let cell = TermCell::new('+', FG_WHITE, BG_BLACK);
        } else {
            let cell = TermCell::new('|', FG_WHITE, BG_BLACK);
        }
        rend_col.insert(0, cell);
        rend_col.push_back(cell);
    }
}