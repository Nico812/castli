use crate::GameCoord;
use crate::exports::tile::TileE;
use crate::exports::units::UnitGroupE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerE {
    pub id: usize,
    pub name: String,
    pub pos: GameCoord,
    pub units: UnitGroupE,
    pub peasants: u32,
}

impl PlayerE {
    pub fn undef() -> Self {
        Self {
            id: 0,
            name: "undefined".to_string(),
            pos: GameCoord { x: 0, y: 0 },
            units: UnitGroupE::undef(),
            peasants: 0,
        }
    }
}
