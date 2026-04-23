use serde::{Deserialize, Serialize};

use crate::{GameCoord, exports::units::UnitGroupE};

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnedCastleE {
    pub alive: bool,
    pub name: String,
    pub pos: GameCoord,
    pub units: UnitGroupE,
    pub peasants: u32,
}
