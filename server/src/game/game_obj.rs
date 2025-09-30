use common::exports::game_object::GameObjE;

use crate::game::{castle::Castle, structure::Structure, units::DeployedUnits};

pub enum GameObj {
    Castle(Castle),
    Structure(Structure),
    DeployedUnits(DeployedUnits),
}

impl GameObj {
    pub fn export(&self) -> GameObjE {
        match self {
            Self::Castle(castle) => GameObjE::Castle(castle.export()),
            Self::Structure(structure) => GameObjE::Structure(structure.export()),
            Self::DeployedUnits(deployed_units) => GameObjE::DeployedUnits(deployed_units.export()),
        }
    }
}
