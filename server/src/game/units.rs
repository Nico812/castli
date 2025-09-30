use std::{boxed::Box, collections::VecDeque};

use common::{
    GameCoord,
    exports::{game_object::DeployedUnitsE, units::UnitGroupE},
};

#[derive(Clone, Copy)]
pub enum Unit {
    Knight,
    Mage,
    Dragon,
}

macro_rules! all_units {
    () => {
        [Unit::Knight, Unit::Mage, Unit::Dragon]
    };
}

impl Unit {
    pub const COUNT: usize = 4;

    fn as_index(&self) -> usize {
        match self {
            Self::Knight => 0,
            Self::Mage => 1,
            Self::Dragon => 2,
        }
    }

    fn as_mask(&self) -> u8 {
        1 << self.as_index()
    }
}

pub struct UnitGroup {
    quantities: Box<[u16; Unit::COUNT]>,
    present_mask: u8,
}

impl UnitGroup {
    pub fn new() -> Self {
        let quantities = Box::new([0; Unit::COUNT]);
        let present_mask = 0;

        Self {
            quantities,
            present_mask,
        }
    }

    pub fn add(&mut self, unit: Unit, count: u16) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_add(count);

        if self.quantities[idx] > 0 {
            self.present_mask |= unit.as_mask();
        }
    }

    pub fn remove(&mut self, unit: Unit, count: u16) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_sub(count);

        if self.quantities[idx] == 0 {
            self.present_mask &= unit.as_mask();
        }
    }

    pub fn contains(&self, unit: Unit) -> bool {
        self.present_mask & unit.as_mask() != 0
    }

    pub fn iter_present(&self) -> impl Iterator<Item = (Unit, u16)> + '_ {
        all_units!()
            .into_iter()
            .filter(move |u| self.contains(*u))
            .map(move |u| (u, self.quantities[u.as_index()]))
    }

    pub fn export(&self) -> UnitGroupE {
        UnitGroupE {
            quantities: *self.quantities.clone(),
        }
    }
}

pub struct DeployedUnits {
    owner: String,
    pos: GameCoord,
    path: VecDeque<GameCoord>,
    unit_group: UnitGroup,
}

impl DeployedUnits {
    pub fn new(
        owner: String,
        pos: GameCoord,
        path: VecDeque<GameCoord>,
        unit_group: UnitGroup,
    ) -> Self {
        Self {
            owner,
            pos,
            path,
            unit_group,
        }
    }

    pub fn move_along_path(&mut self) {
        if let Some(next_pos) = self.path.pop_front() {
            self.pos = next_pos;
        }
    }

    pub fn export(&self) -> DeployedUnitsE {
        DeployedUnitsE {
            owner: self.owner.clone(),
            pos: self.pos,
        }
    }
}
