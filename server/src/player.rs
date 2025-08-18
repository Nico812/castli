//! # Player Representation
//!
//! This module defines the `Player` struct, which holds information
//! about a single player in the game.
pub struct Player {
    name: String,
    pos: Option<(usize, usize)>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let name = name.into();
        println!("New player joined with the name: {}", name);
        Self { name, pos: None }
    }

    pub fn has_castle(&self) {
        pos.is_some()
    }

    pub fn new_castle(&mut self, pos: (usize, usize)) {
        self.pos = pos;
        println!("New castle created by {}", self.name);
    }
}