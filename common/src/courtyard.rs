use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    GameCoord, GroupElement, InhabitantType, LimitedGroup, ResourceType, UnitType, hitboxes::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacilityType {
    Wall,
    FarmPlot,
    Sawmill,
    Mines,
    Barracks,
    Carpenter,
    Treasury,
    Residence,
}

impl GroupElement for FacilityType {
    fn all() -> &'static [Self] {
        &[
            Self::Wall,
            Self::FarmPlot,
            Self::Sawmill,
            Self::Mines,
            Self::Barracks,
            Self::Carpenter,
            Self::Treasury,
            Self::Residence,
        ]
    }
}

impl FacilityType {
    pub fn max_count() -> Vec<u32> {
        let mut lim = vec![0; Self::amt()];
        lim[FacilityType::Wall.as_index()] = 1;
        lim[FacilityType::FarmPlot.as_index()] = 3;
        lim[FacilityType::Sawmill.as_index()] = 1;
        lim[FacilityType::Mines.as_index()] = 1;
        lim[FacilityType::Barracks.as_index()] = 1;
        lim[FacilityType::Carpenter.as_index()] = 1;
        lim[FacilityType::Treasury.as_index()] = 1;
        lim[FacilityType::Residence.as_index()] = 4;
        lim
    }

    pub fn max_lv(&self) -> u8 {
        match self {
            FacilityType::Wall => 10,
            FacilityType::FarmPlot => 10,
            FacilityType::Sawmill => 10,
            FacilityType::Mines => 10,
            FacilityType::Barracks => 10,
            FacilityType::Carpenter => 2,
            FacilityType::Treasury => 10,
            FacilityType::Residence => 10,
        }
    }

    pub fn base_cost(&self) -> LimitedGroup<ResourceType> {
        match self {
            FacilityType::Wall => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 50);
                cost.add_single_type(ResourceType::Stone, 50);
                cost
            }
            FacilityType::FarmPlot => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 15);
                cost.add_single_type(ResourceType::Stone, 15);
                cost
            }
            FacilityType::Sawmill => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 5);
                cost.add_single_type(ResourceType::Stone, 5);
                cost
            }
            FacilityType::Mines => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 5);
                cost.add_single_type(ResourceType::Stone, 5);
                cost
            }
            FacilityType::Barracks => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 50);
                cost.add_single_type(ResourceType::Stone, 50);
                cost
            }
            FacilityType::Carpenter => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 1000);
                cost.add_single_type(ResourceType::Stone, 1000);
                cost
            }
            FacilityType::Treasury => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 25);
                cost.add_single_type(ResourceType::Stone, 25);
                cost
            }
            FacilityType::Residence => {
                let mut cost = LimitedGroup::new(None);
                cost.add_single_type(ResourceType::Wood, 10);
                cost.add_single_type(ResourceType::Stone, 10);
                cost
            }
        }
    }

    pub fn hitbox(&self) -> Hitbox {
        match self {
            FacilityType::Wall => WALL_HITBOX,
            FacilityType::FarmPlot => FARMPLOT_HITBOX,
            FacilityType::Sawmill => SAWMILL_HITBOX,
            FacilityType::Mines => MINES_HITBOX,
            FacilityType::Barracks => BARRACKS_HITBOX,
            FacilityType::Carpenter => SHIPYARD_HITBOX,
            FacilityType::Treasury => SHIPYARD_HITBOX,
            FacilityType::Residence => SHIPYARD_HITBOX,
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            FacilityType::Wall => {
                "Rumors say that members of the dfensive army feel twice as powerful each time the wall has been improved on."
            }
            FacilityType::FarmPlot => {
                "Coziest place in the castle. Production of food is important for mantaining the peasants's sex drive."
            }
            FacilityType::Sawmill => {
                "Somewhere bored peasants can work wood they find on the outshirts of the castle."
            }
            FacilityType::Mines => "Somewhere strong peasants can mine stone. No casualties yet.",
            FacilityType::Barracks => {
                "A training place for peasants aspiring to be promoted as holy knights."
            }
            FacilityType::Carpenter => {
                "Special contraptions can be built here, assuming there is a special ideator living between the walls."
            }
            FacilityType::Treasury => {
                "Where all the resources are stored. It's the cleanest place of the castle."
            }
            FacilityType::Residence => "They are mutch more spatious than they look.",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Facility {
    pub lv: u8,
    pub pos: GameCoord,
    pub r#type: FacilityType,
}

impl Facility {
    pub fn new(r#type: FacilityType, pos: GameCoord) -> Self {
        Self { lv: 1, r#type, pos }
    }

    pub fn upgrade_cost(&self) -> LimitedGroup<ResourceType> {
        self.r#type.base_cost() * self.lv as u32 * self.lv as u32
    }

    pub fn production(&self) -> Vec<Production> {
        match self.r#type {
            FacilityType::FarmPlot => vec![Production::Inhabitants(
                InhabitantType::Peasant,
                self.lv as u32 * 2,
            )],
            FacilityType::Sawmill => vec![Production::Resources(
                ResourceType::Wood,
                self.lv as u32 * 2,
            )],
            FacilityType::Mines => vec![Production::Resources(
                ResourceType::Stone,
                self.lv as u32 * 2,
            )],
            FacilityType::Barracks => vec![Production::Units(UnitType::Knight, self.lv as u32)],
            FacilityType::Carpenter => {
                let mut prod = Vec::new();
                prod.push(Production::Units(UnitType::Wagon, self.lv as u32));
                prod.push(Production::Units(UnitType::Ship, 1));
                prod
            }
            _ => Vec::new(),
        }
    }

    pub fn treasury_lim_extension(lv: u8) -> Vec<u32> {
        let mut cap = vec![0; ResourceType::amt()];
        cap[ResourceType::as_index(ResourceType::Wood)] = (100 * lv * lv) as u32;
        cap[ResourceType::as_index(ResourceType::Stone)] = (100 * lv * lv) as u32;
        cap
    }

    pub fn residence_inhabitants_lim_extension(lv: u8) -> Vec<u32> {
        let mut cap = vec![0; InhabitantType::amt()];
        cap[InhabitantType::as_index(InhabitantType::Peasant)] = (10 * lv * lv) as u32;
        cap
    }

    pub fn residence_units_limit_extension(lv: u8) -> Vec<u32> {
        let mut cap = vec![0; UnitType::amt()];
        cap[UnitType::as_index(UnitType::Knight)] = (2 * lv * lv) as u32;
        cap[UnitType::as_index(UnitType::Wizard)] = lv as u32;
        cap[UnitType::as_index(UnitType::Dragon)] = lv.saturating_sub(7) as u32;
        cap[UnitType::as_index(UnitType::Wagon)] = 0;
        cap[UnitType::as_index(UnitType::Ship)] = 0;
        cap
    }
}

pub enum Production {
    Resources(ResourceType, u32),
    Inhabitants(InhabitantType, u32),
    Units(UnitType, u32),
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Self::Resources(ty, amt) => {
                format!("{:?}: {}", ty, amt)
            }
            Self::Units(ty, amt) => {
                format!("{:?}s: {}", ty, amt)
            }
            Self::Inhabitants(ty, amt) => {
                format!("{:?}s: {}", ty, amt)
            }
        };
        write!(f, "{}", str)
    }
}
