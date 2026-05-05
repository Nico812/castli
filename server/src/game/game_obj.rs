use common::game_objs::GameObjE;

use crate::game::{castle::Castle, structure::Structure, units::DeployedUnits};

pub enum GameObj {
    Castle(Castle),
    Structure(Structure),
    DeployedUnits(DeployedUnits),
}

impl GameObj {
    pub fn export(&self) -> Option<GameObjE> {
        match self {
            Self::Castle(castle) => Some(GameObjE::Castle(castle.export())),
            Self::Structure(structure) => Some(GameObjE::Structure(structure.export())),
            Self::DeployedUnits(deployed_units) => {
                if let Some(export) = deployed_units.export() {
                    return Some(GameObjE::DeployedUnits(export));
                } else {
                    None
                }
            }
        }
    }
}
