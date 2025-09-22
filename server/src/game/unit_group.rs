use std::collections::VecDeque;

use common::{GameCoord, UnitGroupE};

pub struct UnitGroup {
    owner: String,
    pos: GameCoord,
    path: VecDeque<GameCoord>,
}

impl UnitGroup {
    pub fn new(owner: String, pos: GameCoord, path: VecDeque<GameCoord>) -> Self {
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
