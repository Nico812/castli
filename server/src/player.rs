//! # Player Representation
//!
//! This module defines the `Player` struct, which holds information
//! about a single player in the game.

pub struct Player {
    pub name: String,
    pub castle_id: Option<common::GameID>,
}

impl Player {
    pub fn new(name: String) -> Self {
        let name = name;
        println!("New player joined with the name: {}", name);
        Self {
            name,
            castle_id: None,
        }
    }

    pub fn has_castle(&self) -> bool {
        self.castle_id.is_some()
    }

    pub fn set_castle_id(&mut self, castle_id: common::GameID) {
        self.castle_id = Some(castle_id);
        println!(
            "Player {} just got a new castle with GameID {}",
            self.name, castle_id
        );
    }
}
