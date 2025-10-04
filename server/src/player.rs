//! # Player Representation
//!
//! This module defines the `Player` struct, which holds information
//! about a single player in the game.
use common::GameID;

#[derive(PartialEq)]
pub enum PlayerStatus {
    Alive,
    Dead,
    Init,
    Disconnected,
}
pub struct Player {
    pub name: String,
    pub castle_id: Option<GameID>,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(name: String) -> Self {
        let name = name;
        let status = PlayerStatus::Init;
        println!("New player joined with the name: {}", name);
        Self {
            name,
            castle_id: None,
            status,
        }
    }

    pub fn set_castle_id(&mut self, castle_id: common::GameID) {
        self.castle_id = Some(castle_id);
        self.status = PlayerStatus::Alive;
        println!(
            "Player {} just got a new castle with GameID {}",
            self.name, castle_id
        );
    }
}
