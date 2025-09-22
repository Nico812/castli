//! # Common Crate
//!
//! This crate defines the shared data structures, constants, and communication
//! protocols used by both the `server` and `client` components of the Castli project.
//! It ensures that both sides of the application agree on the format of data being exchanged.

pub mod r#const;
pub mod stream;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    AttackCastle(GameID),
    GiveObjs,
    GiveMap,
    GivePlayer,
}

/// Exports:
/// Those define the types of data of a game that are sent to the client

/// These never change during a game.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum TileE {
    Water,
    Grass,
    Woods,
}

/// Exported information on the player info and owned castle
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerE {
    pub id: GameID,
    pub name: String,
    pub pos: GameCoord,
}

/// Exported information on an observable object
#[derive(Serialize, Deserialize, Debug)]
pub enum GameObjE {
    Castle(CastleE),
    Structure(StructureE),
    UnitGroup(UnitGroupE),
}

impl GameObjE {
    pub fn get_pos(&self) -> GameCoord {
        match self {
            GameObjE::Castle(c) => c.pos,
            GameObjE::Structure(s) => s.pos,
            GameObjE::UnitGroup(u) => u.pos,
        }
    }
}

/// Exported information on a not-owned castle
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CastleE {
    pub name: String,
    pub pos: GameCoord,
}

/// Exported information on NPSs (non player structures)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureE {
    pub name: String,
    pub r#type: StructureTypeE,
    pub pos: GameCoord,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum StructureTypeE {
    Farm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitGroupE {
    pub owner: String,
    pub pos: GameCoord,
}
