//! # Common Crate
//!
//! This crate defines the shared data structures, constants, and communication
//! protocols used by both the `server` and `client` components of the Castli project.
//! It ensures that both sides of the application agree on the format of data being exchanged.

pub mod r#const;
pub mod exports;
pub mod stream;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::exports::{game_object::GameObjE, player::PlayerE, tile::TileE, units::UnitGroupE};

/// Global IDs for game objects
pub type GameID = usize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct GameCoord {
    pub x: usize,
    pub y: usize,
}

/// Represents messages sent from the Server to the Client (S2C).
#[derive(Serialize, Deserialize, Debug)]
pub enum S2C {
    LobbyFound,
    ServerFull,
    ConnectionFailed,
    L2S4C(L2S4C),
}

/// Represents messages sent from a Lobby, to the Server, for a Client (L2S4C).
/// These are game-specific messages.
#[derive(Serialize, Deserialize, Debug)]
pub enum L2S4C {
    Map(Vec<Vec<TileE>>),
    GameObjs(HashMap<GameID, GameObjE>),
    Player(PlayerE),
    CreateCastle,
    Log(String),
}

/// Represents messages sent from the Client to the Server (C2S).
#[derive(Serialize, Deserialize, Debug)]
pub enum C2S {
    C2S4L(C2S4L),
    Login(String),
}

/// Represents messages sent from a Client, to the Server, for the Lobby (C2S4L).
#[derive(Serialize, Deserialize, Debug)]
pub enum C2S4L {
    NewCastle(GameCoord),
    AttackCastle(GameID, UnitGroupE),
    SendUnits(GameCoord, UnitGroupE),
    GiveObjs,
    GiveMap,
    GivePlayer,
}
