use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    GameCoord, GameId, ResourceGroup,
    hitboxes::{CASTLE_HITBOX, DEPLOYED_UNITS_HITBOX, Hitbox},
    units::UnitGroup,
};

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

    pub fn get_hitbox(&self) -> Hitbox {
        match self {
            GameObjE::Castle(_) => CASTLE_HITBOX,
            GameObjE::Structure(_) => CASTLE_HITBOX,
            GameObjE::DeployedUnits(_) => DEPLOYED_UNITS_HITBOX,
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
    pub is_alive: bool,
    pub r#type: StructureType,
    pub pos: GameCoord,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum StructureType {
    LoneFarm,
    NonEuclideanTown,
    DragonsLair,
}

impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{self:?}");
        let mut chars = name.chars().peekable();

        while let Some(c) = chars.next() {
            if let Some(next_c) = chars.peek()
                && next_c.is_uppercase()
            {
                write!(f, "{} ", c)?;
            } else {
                write!(f, "{}", c)?;
            }
        }
        write!(f, "")
    }
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
    pub resources: ResourceGroup,
}
