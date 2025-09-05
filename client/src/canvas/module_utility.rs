use crate::ansi::*;
use crate::assets::*;

fn add_frame(title: &String, renderable: &mut Vec<Vec<TermCell>>) {
    let renderable_rows = renderable.len();
    let renderable_cols = renderable[0].len();

    let bot_row = vec![TermCell::new('-', FG_WHITE, BG_BLACK); renderable_cols];

    let top_row = bot_row;
    for (pos, char) in title.chars().enumerate(){
        if pos+2 < renderable_cols {
            top_line[pos+2] = TermCell::new(char, FG_WHITE, BG_BLACK);
        }   
    }

    renderable.push_front(top_row);
    renderable.push_back(bot_row);

    for (pos, rend_col) in renderable.iter_mut().enumerate() {
        let norm_cell = TermCell::new('|', FG_WHITE, BG_BLACK);
        let corner_cell = TermCell::new('+', FG_WHITE, BG_BLACK);
        if (pos == 0 || pos == renderable_rows - 1) {
            rend_col.push_front(corner_cell);
            rend_col.push_back(corner_cell);
        } else {
            rend_col.push_front(norm_cell);
            rend_col.push_back(norm_cell);
        }
    }
}