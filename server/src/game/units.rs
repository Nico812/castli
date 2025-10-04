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
    pub const COUNT: usize = 3;

    pub fn as_index(&self) -> usize {
        match self {
            Self::Knight => 0,
            Self::Mage => 1,
            Self::Dragon => 2,
        }
    }

    pub fn form_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Knight),
            1 => Some(Self::Mage),
            2 => Some(Self::Dragon),
            _ => None,
        }
    }

    pub fn as_mask(&self) -> u8 {
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

    pub fn add_single_type(&mut self, unit: Unit, count: u16) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_add(count);

        if self.quantities[idx] > 0 {
            self.present_mask |= unit.as_mask();
        }
    }

    pub fn subtract_single_type(&mut self, unit: Unit, count: u16) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_sub(count);

        if self.quantities[idx] == 0 {
            self.present_mask &= !unit.as_mask();
        }
    }

    pub fn subtract_if_enough(&mut self, other: &Self) -> bool {
        for (i, quantity) in other.quantities.iter().enumerate() {
            if self.quantities[i] < *quantity {
                return false;
            }
        }
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(Unit::form_index(i).unwrap(), *quantity);
        }
        true
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

    pub fn from_export(export: UnitGroupE) -> Self {
        let quantities = export.quantities;
        let mut unit_group = Self::new();
        for (i, quantity) in quantities.iter().enumerate() {
            match Unit::form_index(i) {
                Some(unit_type) => unit_group.add_single_type(unit_type, *quantity),
                None => continue,
            }
        }
        unit_group
    }

    pub fn is_empty(&self) -> bool {
        for quantity in self.quantities.iter() {
            if *quantity != 0 {
                return false;
            }
        }
        true
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
