pub mod r#const;
pub mod courtyard;
pub mod game_objs;
pub mod map;
pub mod packets;
pub mod player;
pub mod stream;
pub mod units;

use serde::{Deserialize, Serialize};
use std::fmt;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Resources {
    pub wood: u32,
    pub stone: u32,
}

impl Resources {
    pub const fn new(wood: u32, stone: u32) -> Self {
        Self { wood, stone }
    }

    pub fn saturating_add(&mut self, other: &Self) {
        self.wood = self.wood.saturating_add(other.wood);
        self.stone = self.stone.saturating_add(other.stone);
    }

    pub fn saturating_sub(&mut self, other: &Self) {
        self.wood = self.wood.saturating_sub(other.wood);
        self.stone = self.stone.saturating_sub(other.stone);
    }

    pub fn subtract_if_enough(&mut self, other: &Self) -> bool {
        if self.wood >= other.wood && self.stone >= other.stone {
            self.saturating_sub(other);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.wood >= other.wood && self.stone >= other.stone
    }
}
