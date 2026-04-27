use serde::{Deserialize, Serialize};

use crate::r#const::{DRAGON_STR, KNIGHT_STR, MAGE_STR, SHIP_STR};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum UnitType {
    Knight,
    Mage,
    Dragon,
    Ship,
}

#[macro_export]
macro_rules! all_units {
    () => {
        [
            $crate::units::UnitType::Knight,
            $crate::units::UnitType::Mage,
            $crate::units::UnitType::Dragon,
            $crate::units::UnitType::Ship,
        ]
    };
}

#[macro_export]
macro_rules! all_units_enumerate {
    () => {{
        const ALL_UNITS: [$crate::units::UnitType; $crate::units::UnitType::COUNT] =
            $crate::all_units!();

        let mut result = [(0, $crate::units::UnitType::Knight); $crate::units::UnitType::COUNT];

        let mut i = 0;
        for unit in ALL_UNITS {
            result[i] = (i, unit);
            i += 1;
        }
        result
    }};
}

impl UnitType {
    pub const COUNT: usize = 4;

    pub fn as_index(&self) -> usize {
        match self {
            Self::Knight => 0,
            Self::Mage => 1,
            Self::Dragon => 2,
            Self::Ship => 3,
        }
    }

    pub fn form_index(i: usize) -> Self {
        match i {
            0 => Self::Knight,
            1 => Self::Mage,
            2 => Self::Dragon,
            3 => Self::Ship,
            _ => panic!(),
        }
    }

    pub fn get_strength(&self) -> u32 {
        match self {
            Self::Knight => KNIGHT_STR,
            Self::Mage => MAGE_STR,
            Self::Dragon => DRAGON_STR,
            Self::Ship => SHIP_STR,
        }
    }

    pub fn as_mask(&self) -> u8 {
        1 << self.as_index()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnitGroup {
    pub quantities: [u32; UnitType::COUNT],
    present_mask: u8,
}

impl UnitGroup {
    pub fn new() -> Self {
        let quantities = [0; UnitType::COUNT];
        let present_mask = 0;

        Self {
            quantities,
            present_mask,
        }
    }

    //TODO: maybe client doesnt need to know the strength
    pub fn get_strength(&self) -> u32 {
        let mut str = 0;
        for (i, unit) in all_units_enumerate!().iter() {
            str += self.quantities[*i] * unit.get_strength();
        }
        str
    }

    pub fn add_single_type(&mut self, unit: UnitType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_add(count);

        if self.quantities[idx] > 0 {
            self.present_mask |= unit.as_mask();
        }
    }

    pub fn subtract_single_type(&mut self, unit: UnitType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_sub(count);

        if self.quantities[idx] == 0 {
            self.present_mask &= !unit.as_mask();
        }
    }

    pub fn subtract_if_enough(&mut self, other: &Self) -> bool {
        if !other.is_subset(self) {
            return false;
        }
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(UnitType::form_index(i), *quantity);
        }
        true
    }

    pub fn subtract_unchecked(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(UnitType::form_index(i), *quantity);
        }
    }

    pub fn saturating_add(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.add_single_type(UnitType::form_index(i), *quantity);
        }
    }

    pub fn contains(&self, unit: UnitType) -> bool {
        self.present_mask & unit.as_mask() != 0
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for (i, quantity) in other.quantities.iter().enumerate() {
            if self.quantities[i] > *quantity {
                return false;
            }
        }
        true
    }

    pub fn iter_present(&self) -> impl Iterator<Item = (UnitType, u32)> + '_ {
        all_units!()
            .into_iter()
            .filter(move |u| self.contains(*u))
            .map(move |u: UnitType| (u, self.quantities[u.as_index()]))
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
