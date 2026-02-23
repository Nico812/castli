use super::cell::{BG_BLACK, FG_WHITE, TermCell};

pub fn render_frame_into(
    frame: &mut [Vec<TermCell>],
    title: &str,
    pos: (usize, usize),
    inner_rows: usize,
    inner_cols: usize,
) {
    let top = pos.0;
    let left = pos.1;
    let bottom = top + inner_rows + 1;
    let right = left + inner_cols + 1;
    let border = TermCell::new('-', FG_WHITE, BG_BLACK);
    let corner = TermCell::new('+', FG_WHITE, BG_BLACK);
    let side = TermCell::new('|', FG_WHITE, BG_BLACK);

    frame[top][left] = corner;
    frame[top][right] = corner;
    frame[bottom][left] = corner;
    frame[bottom][right] = corner;

    for c in 1..=inner_cols {
        frame[top][left + c] = border;
        frame[bottom][left + c] = border;
    }

    for r in 1..=inner_rows {
        frame[top + r][left] = side;
        frame[top + r][right] = side;
    }

    for (i, ch) in title.chars().enumerate() {
        let c = left + 3 + i;
        if c <= left + inner_cols {
            frame[top][c] = TermCell::new(ch, FG_WHITE, BG_BLACK);
        }
    }
}

pub fn draw_text_row(
    frame: &mut [Vec<TermCell>],
    text: &str,
    row: usize,
    col: usize,
    max_width: usize,
) {
    for (i, ch) in text.chars().enumerate() {
        if i >= max_width {
            break;
        }
        frame[row][col + i] = TermCell::new(ch, FG_WHITE, BG_BLACK);
    }
}
