use serde::{Deserialize, Serialize};

use crate::{GameCoord, GameId, Resources, r#const::FARM_PLOT_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacilityType {
    FarmPlot,
    Sawmill,
    Mines,
    Barracks,
    Shipyard,
}

#[macro_export]
macro_rules! all_facilities {
    () => {
        [
            $crate::courtyard::FacilityType::FarmPlot,
            $crate::courtyard::FacilityType::Sawmill,
            $crate::courtyard::FacilityType::Mines,
            $crate::courtyard::FacilityType::Barracks,
            $crate::courtyard::FacilityType::Shipyard,
        ]
    };
}

impl FacilityType {
    pub const COUNT: usize = 5;

    pub fn max_count(&self) -> u8 {
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
            FacilityType::FarmPlot => Resources::new(4, 4),
            FacilityType::Sawmill => Resources::new(2, 2),
            FacilityType::Mines => Resources::new(2, 2),
            FacilityType::Barracks => Resources::new(50, 50),
            FacilityType::Shipyard => Resources::new(1000, 1000),
        }
    }

    pub fn size(&self) -> GameCoord {
        match self {
            FacilityType::FarmPlot => FARM_PLOT_SIZE,
            FacilityType::Sawmill => GameCoord::new(5, 5),
            FacilityType::Mines => GameCoord::new(5, 5),
            FacilityType::Barracks => GameCoord::new(5, 5),
            FacilityType::Shipyard => GameCoord::new(5, 5),
        }
    }

    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(i: usize) -> Self {
        all_facilities!()[i]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Facility {
    pub lv: u32,
    pub pos: GameCoord,
    pub r#type: FacilityType,
}

impl Facility {
    pub fn new(r#type: FacilityType, pos: GameCoord) -> Self {
        Self { lv: 1, r#type, pos }
    }
}
