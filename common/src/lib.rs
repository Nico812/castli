pub mod r#const;
pub mod courtyard;
pub mod game_objs;
pub mod hitboxes;
pub mod map;
pub mod packets;
pub mod player;
pub mod stream;
pub mod units;

use serde::{Deserialize, Serialize};
use std::{
    array::from_fn,
    fmt,
    ops::{Add, Mul},
};

/// Global IDs for game objects
pub type GameId = usize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameCoord {
    pub x: usize,
    pub y: usize,
}

impl GameCoord {
    pub const fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    // You can only build on even coords
    pub fn is_even(&self) -> bool {
        self.x & 1 == 0 && self.y & 1 == 0
    }
}

impl Add for GameCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for GameCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Time {
    pub tick_cnt: u16,
    pub h: u8,
    pub night: bool,
}

impl Time {
    pub fn new() -> Self {
        Self {
            tick_cnt: 0,
            h: 12,
            night: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick_cnt += 1;
        if self.tick_cnt == 64 {
            self.h += 1;
            if self.h == 24 {
                self.h = 0;
            } else if self.h == 7 {
                self.night = false;
            } else if self.h == 19 {
                self.night = true;
            }
            self.tick_cnt = 0;
        }
    }
}

#[macro_export]
macro_rules! all_resources {
    () => {
        [$crate::ResourceType::Wood, $crate::ResourceType::Stone]
    };
}

#[derive(Copy, Clone, Debug)]
pub enum ResourceType {
    Wood,
    Stone,
}

impl ResourceType {
    pub const COUNT: usize = 2;

    pub fn from_index(i: usize) -> Self {
        all_resources!()[i]
    }

    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn form_index(i: usize) -> Self {
        match i {
            0 => Self::Wood,
            1 => Self::Stone,
            _ => panic!(),
        }
    }

    pub fn as_mask(&self) -> u8 {
        1 << self.as_index()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceGroup {
    pub quantities: [u32; ResourceType::COUNT],
}

impl ResourceGroup {
    pub fn new() -> Self {
        let quantities = [0; ResourceType::COUNT];

        Self { quantities }
    }

    pub fn add_single_type(&mut self, unit: ResourceType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_add(count);
    }

    pub fn subtract_single_type(&mut self, unit: ResourceType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_sub(count);
    }

    pub fn subtract_if_enough(&mut self, other: &Self) -> bool {
        if !other.is_subset(self) {
            return false;
        }
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(ResourceType::form_index(i), *quantity);
        }
        true
    }

    pub fn saturating_sub(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(ResourceType::form_index(i), *quantity);
        }
    }

    pub fn saturating_add(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.add_single_type(ResourceType::form_index(i), *quantity);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for (i, quantity) in other.quantities.iter().enumerate() {
            if self.quantities[i] > *quantity {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> impl Iterator<Item = (ResourceType, &u32)> + '_ {
        all_resources!()
            .into_iter()
            .map(move |ty| (ty, &self.quantities[ty.as_index()]))
    }

    pub fn iter_present(&self) -> impl Iterator<Item = (ResourceType, &u32)> + '_ {
        self.iter().filter(move |(ty, _)| self.contains(*ty))
    }

    pub fn is_empty(&self) -> bool {
        for quantity in self.quantities.iter() {
            if *quantity != 0 {
                return false;
            }
        }
        true
    }

    pub fn contains(&self, unit: ResourceType) -> bool {
        self.quantities[unit as usize] > 0
    }

    pub fn as_string_vector(&self) -> Vec<String> {
        let mut strings = Vec::new();
        for (ty, amt) in self.iter() {
            let s = format!("{:?}: {}", ty, amt);
            strings.push(s);
        }
        strings
    }

    pub fn as_string_vector_nonempty(&self) -> Vec<String> {
        let mut strings = Vec::new();
        for (ty, amt) in self.iter_present() {
            let s = format!("{:?}: {}", ty, amt);
            strings.push(s);
        }
        strings
    }
}

impl Mul<u32> for ResourceGroup {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        let mut copy = self.clone();

        for (i, (_, ref mut amt)) in self.iter().enumerate() {
            copy.quantities[i] = amt.saturating_mul(rhs);
        }
        copy
    }
}

impl fmt::Display for ResourceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strs: [String; ResourceType::COUNT] = std::array::from_fn(|index| {
            format!(
                "{:?}: {}",
                ResourceType::from_index(index),
                self.quantities[index]
            )
        });

        let joined = strs.join(" ");
        write!(f, "{}", joined)
    }
}
