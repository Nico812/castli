pub mod r#const;
pub mod courtyard;
pub mod game_objs;
pub mod hitboxes;
pub mod map;
pub mod packets;
pub mod player;
pub mod stream;

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::r#const::*;

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

pub trait GroupElement: Copy + PartialEq + 'static {
    fn all() -> &'static [Self];

    fn amt() -> usize {
        Self::all().len()
    }

    fn from_index(i: usize) -> Self {
        Self::all()[i]
    }

    fn as_index(self) -> usize {
        Self::all()
            .iter()
            .position(|el| *el == self)
            .expect("element must be included in GroupElement::all")
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LimitedGroup<T: GroupElement> {
    pub quant: Vec<u32>,
    pub lim: Vec<u32>,
    marker: PhantomData<T>,
}

impl<T: GroupElement> LimitedGroup<T> {
    pub fn new(lim: Option<Vec<u32>>) -> Self {
        let quant = vec![0; T::amt()];

        Self {
            quant,
            lim: lim.unwrap_or(vec![u32::MAX; T::amt()]),
            marker: PhantomData,
        }
    }

    pub fn change_lim(&mut self, new_lim: Vec<u32>) {
        self.lim = new_lim;
    }

    pub fn add_single_type(&mut self, ty: T, count: u32) {
        let i = ty.as_index();
        self.quant[i] = self.quant[i].saturating_add(count).min(self.lim[i]);
    }

    pub fn add(&mut self, other: &Self) {
        for (i, (self_quant, other_quant)) in
            self.quant.iter_mut().zip(other.quant.iter()).enumerate()
        {
            *self_quant = self_quant.saturating_add(*other_quant).min(self.lim[i]);
        }
    }

    pub fn add_to_lim(&mut self, other: &Vec<u32>) {
        for (self_lim, other_lim) in self.lim.iter_mut().zip(other.iter()) {
            *self_lim = self_lim.saturating_add(*other_lim);
        }
    }

    pub fn subtract_single_type(&mut self, ty: T, count: u32) {
        let i = ty.as_index();
        self.quant[i] = self.quant[i].saturating_sub(count);
    }

    pub fn subtract(&mut self, other: &Self) {
        for (self_quant, other_quant) in self.quant.iter_mut().zip(other.quant.iter()) {
            *self_quant = self_quant.saturating_sub(*other_quant);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.quant
            .iter()
            .zip(other.quant.iter())
            .all(|(self_quant, other_quant)| self_quant <= other_quant)
    }

    pub fn is_uberset(&self, other: &Self) -> bool {
        self.quant
            .iter()
            .zip(other.quant.iter())
            .all(|(self_quant, other_quant)| self_quant >= other_quant)
    }

    pub fn contains(&self, ty: T) -> bool {
        self.quant[ty.as_index()] > 0
    }

    pub fn is_full_by_type(&self, ty: T) -> bool {
        self.quant[ty.as_index()] >= self.lim[ty.as_index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (T, u32)> + '_ {
        T::all()
            .iter()
            .copied()
            .map(|ty| (ty, self.quant[ty.as_index()]))
    }

    pub fn iter_nonzero(&self) -> impl Iterator<Item = (T, u32)> + '_ {
        self.iter().filter(|(_, amt)| *amt > 0)
    }

    pub fn is_empty(&self) -> bool {
        self.quant.iter().all(|amt| *amt == 0)
    }

    pub fn as_string_vector(&self) -> Vec<String>
    where
        T: std::fmt::Debug,
    {
        self.iter()
            .map(|(ty, amt)| format!("{:?}: {}", ty, amt))
            .collect()
    }

    pub fn as_string_vector_with_lims(&self) -> Vec<String>
    where
        T: std::fmt::Debug,
    {
        self.iter()
            .zip(self.lim.iter())
            .map(|((ty, amt), lim)| format!("{:?}: {}/{}", ty, amt, lim))
            .collect()
    }

    pub fn as_string_vector_nonzero(&self) -> Vec<String>
    where
        T: std::fmt::Debug,
    {
        self.iter_nonzero()
            .map(|(ty, amt)| format!("{:?}: {}", ty, amt))
            .collect()
    }
}

impl<T: GroupElement> Add for LimitedGroup<T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (i, &amount) in rhs.quant.iter().enumerate() {
            self.add_single_type(T::from_index(i), amount);
        }
        self
    }
}

impl<T: GroupElement> Sub for LimitedGroup<T> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (i, &amount) in rhs.quant.iter().enumerate() {
            self.subtract_single_type(T::from_index(i), amount);
        }
        self
    }
}

impl<T: GroupElement> Mul<u32> for LimitedGroup<T> {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self {
        for i in 0..T::amt() {
            self.quant[i] *= rhs;
        }
        self
    }
}

impl<T: GroupElement> Div<u32> for LimitedGroup<T> {
    type Output = Self;

    fn div(mut self, rhs: u32) -> Self {
        for i in 0..T::amt() {
            self.quant[i] = self.quant[i].div_ceil(rhs);
        }
        self
    }
}

impl<T> fmt::Display for LimitedGroup<T>
where
    T: GroupElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = T::all()
            .iter()
            .map(|ty| format!("{ty:?}: {}", self.quant[ty.as_index()]))
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{output}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Wood,
    Stone,
}

impl GroupElement for ResourceType {
    fn all() -> &'static [Self] {
        &[Self::Wood, Self::Stone]
    }
}

impl LimitedGroup<ResourceType> {
    pub fn castle_starting_group() -> Self {
        let mut base_lim = vec![0; ResourceType::amt()];
        base_lim[ResourceType::Wood.as_index()] = 25;
        base_lim[ResourceType::Stone.as_index()] = 25;

        let mut group = Self::new(Some(base_lim));
        group.add_single_type(ResourceType::Stone, 10);
        group.add_single_type(ResourceType::Wood, 10);
        group
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum UnitType {
    Knight,
    Wizard,
    Dragon,
    Wagon,
    Ship,
}

impl UnitType {
    pub fn get_strength(&self) -> u32 {
        match self {
            Self::Knight => KNIGHT_STR,
            Self::Wizard => MAGE_STR,
            Self::Dragon => DRAGON_STR,
            Self::Ship => SHIP_STR,
            Self::Wagon => WAGON_STR,
        }
    }
}

impl GroupElement for UnitType {
    fn all() -> &'static [Self] {
        &[
            Self::Knight,
            Self::Wizard,
            Self::Dragon,
            Self::Wagon,
            Self::Ship,
        ]
    }
}

impl LimitedGroup<UnitType> {
    pub fn get_strength(&self) -> u32 {
        let mut str = 0;

        self.iter_nonzero()
            .for_each(|(ty, amt)| str += amt * ty.get_strength());
        str
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UnitsTarget {
    Coord(GameCoord),
    GameObj(GameId),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InhabitantType {
    Peasant,
}

impl GroupElement for InhabitantType {
    fn all() -> &'static [Self] {
        &[Self::Peasant]
    }
}

impl LimitedGroup<InhabitantType> {
    pub fn castle_starting_group() -> Self {
        let mut base_lim = vec![0; InhabitantType::amt()];
        base_lim[InhabitantType::Peasant.as_index()] = 10;

        let mut group = Self::new(Some(base_lim));
        group.add_single_type(InhabitantType::Peasant, 10);
        group
    }
}
