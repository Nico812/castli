use common::{
    GameCoord,
    exports::{game_object::CastleE, owned_castle::OwnedCastleE, units::UnitType},
};

use crate::game::units::UnitGroup;

pub struct Castle {
    pub name: String,
    pub pos: GameCoord,
    pub units: UnitGroup,
    pub peasants: u32,
    pub alive: bool,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let mut units = UnitGroup::new();
        let peasants = 2;
        let alive = true;

        if name == "gabbiano" {
            units.add_single_type(UnitType::Knight, 1000);
        } else if name == "pellicano" {
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
            units: self.units.export(),
            peasants: self.peasants,
        }
    }
}
