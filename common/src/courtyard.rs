use std::fmt;

use serde::{Deserialize, Serialize};

use crate::units::{UnitGroup, UnitType};
use crate::{GameCoord, Resources};
use crate::{ResourceType, hitboxes::*};

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

    pub fn max_lv(&self) -> u8 {
        match self {
            FacilityType::Wall => 10,
            FacilityType::FarmPlot => 10,
            FacilityType::Sawmill => 10,
            FacilityType::Mines => 10,
            FacilityType::Barracks => 10,
            FacilityType::Shipyard => 2,
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

    pub fn get_desc(&self) -> &str {
        match self {
            FacilityType::Wall => {
                "Its a fucking wall... what do you want from me????? ????????????? ??????????????? ???????? ???????????? ??????????????????????? ????????????????? ???????"
            }
            FacilityType::FarmPlot => "yessssss as asas as asasa",
            FacilityType::Sawmill => "yes",
            FacilityType::Mines => "a",
            FacilityType::Barracks => "a",
            FacilityType::Shipyard => "a",
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

    pub fn lv_up_cost(&self) -> Resources {
        self.r#type.base_cost() * self.lv * self.lv
    }

    pub fn production(&self) -> Option<Production> {
        match self.r#type {
            FacilityType::FarmPlot => Some(Production::Peasants(self.lv * 2)),
            FacilityType::Sawmill => Some(Production::Resources(ResourceType::Wood, self.lv * 2)),
            FacilityType::Mines => Some(Production::Resources(ResourceType::Stone, self.lv * 2)),
            FacilityType::Barracks => Some(Production::Units(UnitType::Knight, self.lv)),
            FacilityType::Shipyard => Some(Production::Units(UnitType::Ship, self.lv)),
            _ => None,
        }
    }
}

pub enum Production {
    Resources(ResourceType, u32),
    Peasants(u32),
    Units(UnitType, u32),
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Self::Resources(r#type, amt) => {
                format!("{:?}: {}", r#type, amt)
            }
            Self::Units(r#type, amt) => {
                format!("{:?}: {}", r#type, amt)
            }
            Self::Peasants(amt) => {
                format!("Peasants: {}", amt)
            }
        };
        write!(f, "{}", str)
    }
}
