use serde::{Deserialize, Serialize};

use crate::{GameCoord, GameId, Resources, units::UnitGroup};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameObjE {
    Castle(CastleE),
    Structure(StructureE),
    DeployedUnits(DeployedUnitsE),
}

impl GameObjE {
    pub fn get_pos(&self) -> GameCoord {
        match self {
            GameObjE::Castle(c) => c.pos,
            GameObjE::Structure(s) => s.pos,
            GameObjE::DeployedUnits(u) => u.pos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CastleE {
    pub name: String,
    pub pos: GameCoord,
    pub alive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureE {
    pub name: String,
    pub r#type: StructureType,
    pub pos: GameCoord,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum StructureType {
    Farm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeployedUnitsE {
    pub owner_id: GameId,
    pub pos: GameCoord,
}

#[derive(Serialize, Deserialize)]
pub struct OwnedCastleE {
    pub alive: bool,
    pub name: String,
    pub pos: GameCoord,
    pub units: UnitGroup,
    pub resources: Resources,
}
