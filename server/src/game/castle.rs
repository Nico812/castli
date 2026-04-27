use common::{
    GameCoord, Resources,
    game_objs::{CastleE, OwnedCastleE},
    units::{UnitGroup, UnitType},
};

use crate::game::courtyard::Courtyard;

pub struct Castle {
    pub name: String,
    pub pos: GameCoord,
    pub alive: bool,
    pub peasants: u32,
    pub units: UnitGroup,
    pub resources: Resources,
    pub courtyard: Courtyard,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let mut units = UnitGroup::new();
        let peasants = 2;
        let alive = true;
        let mut resources = Resources::new(10, 10);

        if name == "gabbiano" {
            resources.saturating_add(&Resources::new(100, 100));
            units.add_single_type(UnitType::Knight, 100);
        } else if name == "pellicano" {
            resources.saturating_add(&Resources::new(1000, 1000));
            units.add_single_type(UnitType::Knight, 1000);
            units.add_single_type(UnitType::Mage, 1000);
            units.add_single_type(UnitType::Dragon, 1000);
        } else {
            units.add_single_type(UnitType::Knight, 5);
        }

        Self {
            name,
            pos,
            units,
            peasants,
            alive,
            courtyard: Courtyard::new(),
            resources: Resources::new(10, 10),
        }
    }

    pub fn export(&self) -> CastleE {
        CastleE {
            name: self.name.clone(),
            pos: self.pos,
            alive: self.alive,
        }
    }

    pub fn export_owned(&self) -> OwnedCastleE {
        OwnedCastleE {
            alive: self.alive,
            name: self.name.clone(),
            pos: self.pos,
            units: self.units.clone(),
            peasants: self.peasants,
        }
    }
}
