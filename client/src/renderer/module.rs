use crossterm::style::Color;

use crate::{
    r#const::FRAME_WIDTH,
    coord::TermCoord,
    renderer::renderer::{DrawableAsset, DrawableCell},
};

#[derive(Clone)]
pub struct Module {
    name: String,
    size: TermCoord,
    padding: TermCoord,
    cells: Vec<Vec<DrawableCell>>,
    drawn: (Vec<bool>, Vec<bool>),
    size_reset: TermCoord,
}

// Set size.y = 0 for a dynamic sized Module
// The size covers border + padding + content
impl Module {
    pub fn new(size: TermCoord, padding: TermCoord) -> Self {
        let cells = vec![
            vec![
                DrawableCell::new(' ', Color::Black, Color::Black);
                size.x.max(padding.x * 2 + 2) as usize
            ];
            size.y.max(padding.y * 2 + 2) as usize
        ];
        let size_y = cells.len();
        let size_x = cells[0].len();
        Self {
            name: "Gesus of Nazaret".to_string(),
            size: TermCoord::new(size_y as isize, size_x as isize),
            padding,
            cells,
            drawn: (vec![false; size_y], vec![false; size_x]),
            size_reset: size,
        }
    }

    pub fn draw_cell(&mut self, asset: DrawableCell, pos: TermCoord) {
        let drawable_size = self.drawable_size();
        let content_start = self.content_start_pos();
        let (x, y) =
            if pos.y >= 0 && pos.x >= 0 && pos.y < drawable_size.y && pos.x < drawable_size.x {
                (
                    (pos.x + content_start.x) as usize,
                    (pos.y + content_start.y) as usize,
                )
            } else {
                return;
            };
        self.cells[y][x] = asset;
        self.drawn.0[y] = true;
        self.drawn.1[x] = true;
    }

    pub fn push_empty_row(&mut self) {
        let drawable_size = self.drawable_size();
        let content_start = self.content_start_pos();
        self.cells.insert(
            (drawable_size.y + content_start.y) as usize,
            vec![DrawableCell::new(' ', Color::Black, Color::Black); self.size.x as usize],
        );
        self.drawn
            .0
            .insert((drawable_size.y + content_start.y) as usize, true);
        self.size.y += 1;
    }

    pub fn get_cells(&mut self) -> Vec<Vec<DrawableCell>> {
        self.update_frame();
        let cells = self.cells.clone();
        self.reset();

        cells
    }

    fn reset(&mut self) {
        *self = Self::new(self.size_reset, self.padding);
    }

    pub fn center(&mut self) {
        let Some((first_drawn_row, _)) = self
            .drawn
            .0
            .iter()
            .enumerate()
            .find(|(_, is_drawn)| **is_drawn)
        else {
            return;
        };
        let Some((first_drawn_col, _)) = self
            .drawn
            .1
            .iter()
            .enumerate()
            .find(|(_, is_drawn)| **is_drawn)
        else {
            return;
        };

        let Some((last_drawn_row, _)) = self
            .drawn
            .0
            .iter()
            .enumerate()
            .rfind(|(_, is_drawn)| **is_drawn)
        else {
            return;
        };
        let Some((last_drawn_col, _)) = self
            .drawn
            .1
            .iter()
            .enumerate()
            .rfind(|(_, is_drawn)| **is_drawn)
        else {
            return;
        };

        let content_height = last_drawn_row - first_drawn_row + 1;
        let content_width = last_drawn_col - first_drawn_col + 1;

        let drawable_size = self.drawable_size();
        let target_start_row =
            (drawable_size.y - content_height as isize) / 2 + self.padding.y + FRAME_WIDTH as isize;
        let target_start_col =
            (drawable_size.x - content_width as isize) / 2 + self.padding.x + FRAME_WIDTH as isize;

        // TODO: maybe i shouldnt rewrite this code here
        let mut cells_new = vec![
            vec![
                DrawableCell::new(' ', Color::Black, Color::Black);
                self.size.x.max(self.padding.x * 2 + 2) as usize
            ];
            self.size.y.max(self.padding.y * 2 + 2) as usize
        ];

        for (i, row) in (first_drawn_row..=last_drawn_row).enumerate() {
            for (j, col) in (first_drawn_col..=last_drawn_col).enumerate() {
                cells_new[target_start_row as usize + i][target_start_col as usize + j] =
                    self.cells[row][col];
            }
        }

        self.cells = cells_new;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn draw_asset(&mut self, asset: DrawableAsset, pos: TermCoord) {
        for (art_row, art_row_iter) in asset.iter().enumerate() {
            for (art_col, art_cell_opt) in art_row_iter.iter().enumerate() {
                let cell_pos = TermCoord::new(pos.y + art_row as isize, pos.x + art_col as isize);
                if let Some(art_cell) = art_cell_opt {
                    self.draw_cell(*art_cell, cell_pos);
                }
            }
        }
    }

    pub fn draw_cell_last_row(&mut self, asset: DrawableCell, col: isize) {
        let pos = TermCoord::new(self.drawable_size().y - 1, col);
        self.draw_cell(asset, pos);
    }

    pub fn drawable_size(&self) -> TermCoord {
        let mut size = self.size - self.padding * 2 as usize;
        // canvas:
        size.y -= 2;
        size.x -= 2;
        size
    }

    pub fn draw_text_in_row(&mut self, string: &str, row: usize) {
        for (i, ch) in string.chars().enumerate() {
            let cell = DrawableCell {
                ch,
                fg: Color::White,
                bg: Color::Black,
            };
            self.draw_cell(cell, TermCoord::new(row as isize, i as isize));
        }
    }

    pub fn push_row_with_text(&mut self, text: &str) {
        self.push_empty_row();
        self.draw_text_in_row(text, (self.drawable_size().y as usize).saturating_sub(1));
    }

    fn update_frame(&mut self) {
        let bot_row =
            vec![DrawableCell::new('-', Color::White, Color::Black); self.size.x as usize];

        let mut top_row = bot_row.clone();
        for (pos, char) in self.name.chars().enumerate() {
            if pos + 2 < self.size.x as usize {
                top_row[pos + 2] = DrawableCell::new(char, Color::White, Color::Black);
            }
        }

        self.cells[0] = top_row;
        self.cells[(self.size.y as usize).saturating_sub(1)] = bot_row;

        for row in 0..self.size.y {
            let char = if row == 0 || row == self.size.y - 1 {
                '▓'
            } else {
                '|'
            };
            self.cells[row as usize][0] = DrawableCell::new(char, Color::White, Color::Black);
            self.cells[row as usize][(self.size.x as usize).saturating_sub(1)] =
                DrawableCell::new(char, Color::White, Color::Black);
        }
    }

    fn content_start_pos(&self) -> TermCoord {
        TermCoord::new(self.padding.y + 1, self.padding.x + 1)
    }
}
