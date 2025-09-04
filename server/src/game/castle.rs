use common::CastleE;

pub struct Castle {
    pub name: String,
    pub pos: (usize, usize),
}

impl Castle {
    pub fn new(name: String, pos: (usize, usize)) -> Self {
        Self { name, pos }
    }

    pub fn export(&self) -> CastleE {
        CastleE {
            name: self.name.clone(),
            pos: self.pos,
        }
    }
}
