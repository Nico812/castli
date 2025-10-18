use std::{boxed::Box, collections::VecDeque};

use common::{
    GameCoord, GameID,
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

#[derive(Clone)]
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

    //TODO: totally hange this when coding the dynamic battles.
    pub fn get_strength(&self) -> u32 {
        let mut str = 0;

        for _ in self.iter_present() {
            str += 1;
        }
        str
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

    pub fn saturating_add(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.add_single_type(Unit::form_index(i).unwrap(), *quantity);
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
    pub owner_id: GameID,
    pub target_id: Option<GameID>,
    pub returning: bool,
    path: VecDeque<GameCoord>,
    path_index: usize,
    path_size: usize,
    pub unit_group: UnitGroup,
}

impl DeployedUnits {
    pub fn new(
        owner_id: GameID,
        target_id: Option<GameID>,
        path: VecDeque<GameCoord>,
        unit_group: UnitGroup,
    ) -> Self {
        Self {
            owner_id,
            target_id,
            path_size: path.len(),
            path,
            unit_group,
            path_index: 0,
            returning: false,
        }
    }

    pub fn get_pos(&self) -> GameCoord {
        self.path[self.path_index]
    }

    pub fn move_along_path(&mut self) {
        let next_index: usize = match self.returning {
            true => self.path_index.saturating_sub(1),
            false => self.path_index.saturating_add(1),
        };

        if let Some(_) = self.path.get(next_index) {
            self.path_index = next_index;
        }
    }

    pub fn pending(&self) -> bool {
        if self.path_index == 0 && self.returning == true {
            return true;
        }
        if self.path_index >= self.path_size - 1 && self.returning == false {
            return true;
        }
        false
    }

    pub fn arrived_home(&self) -> bool {
        self.pending() && self.returning
    }

    pub fn arrived_target(&self) -> bool {
        self.pending() && !self.returning
    }

    // This needs to be fully rethinked because i want dynamic battles!
    pub fn get_strength(&self) -> u32 {
        self.unit_group.get_strength()
    }

    pub fn r#return(&mut self) {
        self.returning = true;
    }

    pub fn export(&self) -> DeployedUnitsE {
        DeployedUnitsE {
            owner_id: self.owner_id,
            pos: self.get_pos(),
        }
    }
}
