use std::collections::VecDeque;

use common::UnitGroupE;

pub struct UnitGroup {
    owner: String,
    pos: (usize, usize),
    path: VecDeque<(usize, usize)>,
}

impl UnitGroup {
    pub fn new(owner: String, pos: (usize, usize), path: VecDeque<(usize, usize)>) -> Self {
        Self { owner, pos, path }
    }

    pub fn move_along_path(&mut self) {
        if let Some(next_pos) = self.path.pop_front() {
            self.pos = next_pos;
        }
    }

    pub fn export(&self) -> UnitGroupE {
        UnitGroupE {
            owner: self.owner.clone(),
            pos: self.pos,
        }
    }
}
