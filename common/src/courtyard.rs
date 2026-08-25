use serde::{Deserialize, Serialize};

use crate::hitboxes::*;
use crate::{GameCoord, Resources};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacilityType {
    Wall,
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
            $crate::courtyard::FacilityType::Wall,
            $crate::courtyard::FacilityType::FarmPlot,
            $crate::courtyard::FacilityType::Sawmill,
            $crate::courtyard::FacilityType::Mines,
            $crate::courtyard::FacilityType::Barracks,
            $crate::courtyard::FacilityType::Shipyard,
        ]
    };
}

impl FacilityType {
    pub const COUNT: usize = all_facilities!().len();

    pub fn max_count(&self) -> u8 {
        match self {
            FacilityType::Wall => 1,
            FacilityType::FarmPlot => 4,
            FacilityType::Sawmill => 1,
            FacilityType::Mines => 1,
            FacilityType::Barracks => 1,
            FacilityType::Shipyard => 1,
        }
    }

    pub fn base_cost(&self) -> Resources {
        match self {
            FacilityType::Wall => Resources::new(10, 10),
            FacilityType::FarmPlot => Resources::new(4, 4),
            FacilityType::Sawmill => Resources::new(2, 2),
            FacilityType::Mines => Resources::new(2, 2),
            FacilityType::Barracks => Resources::new(50, 50),
            FacilityType::Shipyard => Resources::new(1000, 1000),
        }
    }

    pub fn get_hitbox(&self) -> Hitbox {
        match self {
            FacilityType::Wall => WALL_HITBOX,
            FacilityType::FarmPlot => FARMPLOT_HITBOX,
            FacilityType::Sawmill => SAWMILL_HITBOX,
            FacilityType::Mines => MINES_HITBOX,
            FacilityType::Barracks => BARRACKS_HITBOX,
            FacilityType::Shipyard => SHIPYARD_HITBOX,
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
