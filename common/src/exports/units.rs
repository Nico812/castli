use serde::{Deserialize, Serialize};

use crate::r#const::{DRAGON_STR, KNIGHT_STR, MAGE_STR};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum UnitType {
    Knight,
    Mage,
    Dragon,
}

#[macro_export]
macro_rules! all_units {
    () => {
        [UnitType::Knight, UnitType::Mage, UnitType::Dragon]
    };
}

#[macro_export]
macro_rules! all_units_enum {
    () => {{
        const ALL_UNITS: [UnitType; 3] = all_units!();
        let mut result = [(0, UnitType::Knight); 3];
        let mut i = 0;
        for unit in ALL_UNITS {
            result[i] = (i, unit);
            i += 1;
        }
        result
    }};
}

impl UnitType {
    pub const COUNT: usize = 3;

    pub fn as_index(&self) -> usize {
        match self {
            Self::Knight => 0,
            Self::Mage => 1,
            Self::Dragon => 2,
        }
    }

    pub fn form_index(i: usize) -> Self {
        match i {
            0 => Self::Knight,
            1 => Self::Mage,
            2 => Self::Dragon,
            _ => panic!(),
        }
    }

    pub fn get_strength(&self) -> u8 {
        match self {
            Self::Knight => KNIGHT_STR,
            Self::Mage => MAGE_STR,
            Self::Dragon => DRAGON_STR,
        }
    }

    pub fn as_mask(&self) -> u8 {
        1 << self.as_index()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitGroupE {
    pub quantities: [u16; UnitType::COUNT],
}
