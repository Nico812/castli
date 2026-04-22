use serde::{Deserialize, Serialize};

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

    pub fn as_mask(&self) -> u8 {
        1 << self.as_index()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitGroupE {
    pub quantities: [u16; UnitType::COUNT],
}

impl UnitGroupE {
    pub fn undef() -> Self {
        Self {
            quantities: [0; UnitType::COUNT],
        }
    }
}
