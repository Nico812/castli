use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UnitE {
    Knight,
    Mage,
    Dragon,
}

impl UnitE {
    pub const COUNT: usize = 3;

    pub fn as_index(&self) -> usize {
        match self {
            Self::Knight => 0,
            Self::Mage => 1,
            Self::Dragon => 2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitGroupE {
    pub quantities: [u16; UnitE::COUNT],
}

impl UnitGroupE {
    pub fn undef() -> Self {
        Self {
            quantities: [0; UnitE::COUNT],
        }
    }
}
