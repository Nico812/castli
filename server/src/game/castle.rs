use common::{GameCoord, exports::game_object::CastleE};

use crate::game::units::{Unit, UnitGroup};

pub struct Castle {
    pub name: String,
    pub pos: GameCoord,
    pub units: UnitGroup,
    pub peasants: u32,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let mut units = UnitGroup::new();
        let peasants = 2;

        units.add_single_type(Unit::Knight, 6);

        Self {
            name,
            pos,
            units,
            peasants,
        }
    }

    pub fn export(&self) -> CastleE {
        CastleE {
            name: self.name.clone(),
            pos: self.pos,
        }
    }
}
