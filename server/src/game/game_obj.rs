use common::GameObjE;

use crate::game::{castle::Castle, structure::Structure, unit_group::UnitGroup};

pub enum GameObj {
    Castle(Castle),
    Structure(Structure),
    UnitGroup(UnitGroup),
}

impl GameObj {
    pub fn export(&self) -> GameObjE {
        match self {
            Self::Castle(castle) => GameObjE::Castle(castle.export()),
            Self::Structure(structure) => GameObjE::Structure(structure.export()),
            Self::UnitGroup(unit_group) => GameObjE::UnitGroup(unit_group.export()),
        }
    }
}
