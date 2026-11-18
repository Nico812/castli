use common::{GameCoord, game_objs::GameObjE};

use crate::game::{castle::Castle, structure::Structure, units::DeployedUnits};

pub enum GameObj {
    Castle(Castle),
    Structure(Structure),
    DeployedUnits(DeployedUnits),
}

impl GameObj {
    pub fn get_pos(&self) -> Option<GameCoord> {
        match self {
            Self::Castle(castle) => Some(castle.get_pos()),
            Self::Structure(structure) => Some(structure.get_pos()),
            Self::DeployedUnits(deployed_units) => deployed_units.get_pos(),
        }
    }

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

    pub fn is_attackable(&self) -> bool {
        match self {
            Self::Castle(_) => true,
            Self::Structure(_) => true,
            Self::DeployedUnits(_) => false,
        }
    }
}
