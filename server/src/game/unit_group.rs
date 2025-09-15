use std::collections::VecDeque;

use common::UnitGroupE;

pub struct UnitGroup {
    owner: String,
    pos: (usize, usize),
    path: VecDeque<(usize, usize)>,
}

impl UnitGroup {
    pub fn export(&self) -> UnitGroupE {
        UnitGroupE {
            owner: self.owner.clone(),
            pos: self.pos,
        }
    }
}
