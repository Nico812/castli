use common::{CastleE, GameCoord};

pub struct Castle {
    pub name: String,
    pub pos: GameCoord,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        Self { name, pos }
    }

    pub fn export(&self) -> CastleE {
        CastleE {
            name: self.name.clone(),
            pos: self.pos,
        }
    }
}
