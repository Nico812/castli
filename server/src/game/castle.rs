use common::{
    GameCoord,
    exports::{game_object::CastleE, owned_castle::OwnedCastleE, units::UnitType},
};

use crate::game::{facilities::Facilities, units::UnitGroup};

pub struct Resources {
    pub wood: u32,
    pub stone: u32,
}

impl Resources {
    pub fn new(wood: u32, stone: u32) -> Self {
        Self { wood, stone }
    }

    pub fn add(&mut self, other: Self) {
        self.wood += other.wood;
        self.stone += other.stone;
    }
}

pub struct Castle {
    pub name: String,
    pub pos: GameCoord,
    pub alive: bool,
    pub peasants: u32,
    pub units: UnitGroup,
    pub facilities: Facilities,
    pub resources: Resources,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let mut units = UnitGroup::new();
        let peasants = 2;
        let alive = true;
        let mut resources = Resources::new(10, 10);

        if name == "gabbiano" {
            resources.add(Resources::new(100, 100));
            units.add_single_type(UnitType::Knight, 100);
        } else if name == "pellicano" {
            resources.add(Resources::new(1000, 1000));
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
            facilities: Facilities::new(),
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
            units: self.units.export(),
            peasants: self.peasants,
        }
    }
}
