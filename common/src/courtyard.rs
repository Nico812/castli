use serde::{Deserialize, Serialize};

use crate::{GameCoord, Resources};

pub const COURTYARD_ROWS: usize = 64;
pub const COURTYARD_COLS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacilityType {
    FarmPlot,
    Sawmill,
    Mines,
    Barracks,
    Shipyard,
}

impl FacilityType {
    pub const COUNT: usize = 5;

    pub const fn all() -> [FacilityType; Self::COUNT] {
        [
            FacilityType::FarmPlot,
            FacilityType::Sawmill,
            FacilityType::Mines,
            FacilityType::Barracks,
            FacilityType::Shipyard,
        ]
    }

    pub fn max_count(&self) -> usize {
        match self {
            FacilityType::FarmPlot => 4,
            FacilityType::Sawmill => 1,
            FacilityType::Mines => 1,
            FacilityType::Barracks => 1,
            FacilityType::Shipyard => 1,
        }
    }

    pub fn base_cost(&self) -> Resources {
        match self {
            FacilityType::FarmPlot => Resources::new(10, 10),
            FacilityType::Sawmill => Resources::new(10, 10),
            FacilityType::Mines => Resources::new(10, 10),
            FacilityType::Barracks => Resources::new(10, 10),
            FacilityType::Shipyard => Resources::new(10, 10),
        }
    }

    pub fn size(&self) -> GameCoord {
        match self {
            FacilityType::FarmPlot => GameCoord::new(5, 5),
            FacilityType::Sawmill => GameCoord::new(5, 5),
            FacilityType::Mines => GameCoord::new(5, 5),
            FacilityType::Barracks => GameCoord::new(5, 5),
            FacilityType::Shipyard => GameCoord::new(5, 5),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            FacilityType::FarmPlot => 0,
            FacilityType::Sawmill => 1,
            FacilityType::Mines => 2,
            FacilityType::Barracks => 3,
            FacilityType::Shipyard => 4,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Facility {
    pub lv: u32,
    pub pos: GameCoord,
    pub r#type: FacilityType,
}

impl Facility {
    pub fn new(r#type: FacilityType, lv: u32, pos: GameCoord) -> Self {
        Self { lv, pos, r#type }
    }

    pub fn cost(&self) -> Resources {
        let base = self.r#type.base_cost();
        Resources::new(base.wood * self.lv, base.stone * self.lv)
    }

    pub fn size(&self) -> GameCoord {
        self.r#type.size()
    }

    pub fn pos(&self) -> GameCoord {
        self.pos
    }
}
