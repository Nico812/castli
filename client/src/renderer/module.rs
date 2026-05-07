use crate::{
    ansi::{BLACK, WHITE},
    assets::{BKG_EL, TermCell},
    coord::TermCoord,
    renderer::r#const::{FRAME_BK_COLOR, FRAME_WIDTH, MOD_BK_COLOR},
};

pub struct Module {
    name: String,
    size: TermCoord,
    padding: TermCoord,
    cells: Vec<Vec<TermCell>>,
    drawn: (Vec<bool>, Vec<bool>),
}

// Set size.y = 0 for a dynamic sized Module
// The size covers border + padding + content
impl Module {
    pub fn new(size: TermCoord, padding: TermCoord) -> Self {
        let cells =
            vec![
                vec![TermCell::new(' ', BLACK, MOD_BK_COLOR); size.x.max(padding.x * 2 + 2)];
                size.y.max(padding.y * 2 + 2)
            ];
        let size_y = cells.len();
        let size_x = cells[0].len();
        Self {
            name: "Gesus of Nazaret".to_string(),
            size: TermCoord::new(size_y, size_x),
            padding,
            cells,
            drawn: (vec![false; size_y], vec![false; size_x]),
        }
    }

    pub fn draw_cell(&mut self, asset: TermCell, pos: TermCoord) {
        let drawable_size = self.drawable_size();
        if pos.y < drawable_size.y && pos.x < drawable_size.x {
            let content_start = self.content_start_pos();
            self.cells[pos.y + content_start.y][pos.x + content_start.x] = asset;
            self.drawn.0[pos.y + content_start.y] = true;
            self.drawn.1[pos.x + content_start.x] = true;
        }
    }

    pub fn push_empty_row(&mut self) {
        let drawable_size = self.drawable_size();
        let content_start = self.content_start_pos();
        self.cells.insert(
            drawable_size.y + content_start.y,
            vec![TermCell::new(' ', BLACK, MOD_BK_COLOR); self.size.x],
        );
        self.drawn.0.insert(drawable_size.y + content_start.y, true);
        self.size.y += 1;
    }

    pub fn get_cells(&mut self) -> &Vec<Vec<TermCell>> {
        self.update_frame();
        &self.cells
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
            (drawable_size.y - content_height) / 2 + self.padding.y + FRAME_WIDTH;
        let target_start_col = (drawable_size.x - content_width) / 2 + self.padding.x + FRAME_WIDTH;

        // TODO: maybe i shouldnt rewrite this code here
        let mut cells_new = vec![
            vec![
                TermCell::new(' ', BLACK, MOD_BK_COLOR);
                self.size.x.max(self.padding.x * 2 + 2)
            ];
            self.size.y.max(self.padding.y * 2 + 2)
        ];

        for (i, row) in (first_drawn_row..=last_drawn_row).enumerate() {
            for (j, col) in (first_drawn_col..=last_drawn_col).enumerate() {
                cells_new[target_start_row + i][target_start_col + j] = self.cells[row][col];
            }
        }

        self.cells = cells_new;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn draw_asset(&mut self, asset: &[&[TermCell]], pos: TermCoord) {
        for (art_row, art_row_iter) in asset.iter().enumerate() {
            for (art_col, art_cell) in art_row_iter.iter().enumerate() {
                let cell_pos = TermCoord::new(pos.y + art_row, pos.x + art_col);
                self.draw_cell(*art_cell, cell_pos);
            }
        }
    }

    pub fn draw_cell_last_row(&mut self, asset: TermCell, col: usize) {
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
            let cell = TermCell {
                ch,
                fg: WHITE,
                bg: MOD_BK_COLOR,
            };
            self.draw_cell(cell, TermCoord::new(row, i));
        }
    }

    pub fn push_row_with_text(&mut self, text: &str) {
        self.push_empty_row();
        self.draw_text_in_row(text, self.drawable_size().y - 1);
    }

    fn update_frame(&mut self) {
        let bot_row = vec![TermCell::new('-', WHITE, FRAME_BK_COLOR); self.size.x];

        let mut top_row = bot_row.clone();
        for (pos, char) in self.name.chars().enumerate() {
            if pos + 2 < self.size.x {
                top_row[pos + 2] = TermCell::new(char, WHITE, FRAME_BK_COLOR);
            }
        }

        self.cells[0] = top_row;
        self.cells[self.size.y - 1] = bot_row;

        for row in 0..self.size.y {
            self.cells[row][0] = TermCell::new('|', WHITE, FRAME_BK_COLOR);
            self.cells[row][self.size.x - 1] = TermCell::new('|', WHITE, FRAME_BK_COLOR);
        }
    }

    fn content_start_pos(&self) -> TermCoord {
        TermCoord::new(self.padding.y + 1, self.padding.x + 1)
    }
}
