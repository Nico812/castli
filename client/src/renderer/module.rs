use crate::{
    ansi::{BLACK, WHITE},
    assets::{BKG_EL, TermCell},
    coord::TermCoord,
};

pub struct Module {
    name: String,
    size: TermCoord,
    padding: TermCoord,
    pos: TermCoord,
    cells: Vec<Vec<TermCell>>,
}

impl Module {
    pub fn new(size: TermCoord, padding: TermCoord, pos: TermCoord) -> Self {
        let cells = vec![vec![BKG_EL; size.x.max(padding.x * 2)]; size.y.max(padding.y * 2)];
        Self {
            name: "Gesus of Nazaret".to_string(),
            size,
            padding,
            pos,
            cells,
        }
    }

    pub fn get_cells(&mut self) -> &Vec<Vec<TermCell>> {
        self.update_frame();
        &self.cells
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn draw_asset(&mut self, asset: &[&[TermCell]], pos: TermCoord) {
        for (art_row, art_row_iter) in asset.iter().enumerate() {
            for (art_col, art_cell) in art_row_iter.iter().enumerate() {
                let cell_pos_y = pos.y + art_row;
                let cell_pos_x = pos.x + art_col;
                if cell_pos_y < self.size.y && cell_pos_x < self.size.x {
                    self.cells[cell_pos_y][cell_pos_x] = *art_cell;
                }
            }
        }
    }

    pub fn draw_cell(&mut self, asset: TermCell, pos: TermCoord) {
        if pos.y < self.size.y && pos.x < self.size.x {
            self.cells[pos.y][pos.x] = asset;
        }
    }

    pub fn drawable_size(&self) -> TermCoord {
        self.size - self.padding * 2 as usize
    }

    pub fn push_empty_row(&mut self) {
        let drawable_size = self.drawable_size();
        self.cells.insert(
            drawable_size.y + self.padding.y,
            vec![TermCell::new(' ', BLACK, BLACK); self.size.x],
        );
    }

    pub fn draw_text_in_row(&mut self, string: &str, row: usize) {
        let drawable_size = self.drawable_size();
        if row >= drawable_size.y {
            return;
        }

        for (i, ch) in string.chars().enumerate() {
            if i < drawable_size.x {
                let cell = TermCell {
                    ch,
                    fg: WHITE,
                    bg: BLACK,
                };
                self.draw_cell(cell, TermCoord::new(row, i));
            }
        }
    }

    pub fn push_row_with_text(&mut self, text: &str) {
        self.cells
            .push(vec![TermCell::new(' ', BLACK, BLACK); self.size.x]);
        self.draw_text_in_row(text, self.cells.len() - 1);
    }

    fn update_frame(&mut self) {
        let bot_row = vec![TermCell::new('-', WHITE, BLACK); self.size.x];

        let mut top_row = bot_row.clone();
        for (pos, char) in self.name.chars().enumerate() {
            if pos + 2 < self.size.x {
                top_row[pos + 2] = TermCell::new(char, WHITE, BLACK);
            }
        }

        self.cells[0] = top_row;
        self.cells[self.size.y - 1] = bot_row;

        for row in 0..self.size.y {
            self.cells[row][0] = TermCell::new('|', WHITE, BLACK);
            self.cells[row][self.size.x - 1] = TermCell::new('|', WHITE, BLACK);
        }
    }
}
