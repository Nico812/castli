use crate::GameCoord;
use crate::exports::player::PlayerE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureE {
    pub name: String,
    pub r#type: StructureTypeE,
    pub pos: GameCoord,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum StructureTypeE {
    Farm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeployedUnitsE {
    pub owner: String,
    pub pos: GameCoord,
}
