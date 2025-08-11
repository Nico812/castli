//! # Common Crate
//!
//! This crate defines the shared data structures, constants, and communication
//! protocols used by both the `server` and `client` components of the Castli project.
//! It ensures that both sides of the application agree on the format of data being exchanged.

pub mod r#const;
pub mod stream;

use serde::{Deserialize, Serialize};

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
    MapObjs(MapObjsE),
    PlayerData(PlayerDataE),
}

/// Represents messages sent from the Client to the Server (C2S).
#[derive(Serialize, Deserialize, Debug)]
pub enum C2S {
    C2S4L(C2S4L),
    Login(String),
    NewCastle((usize, usize)),
}

/// Represents messages sent from a Client, to the Server, for the Lobby (C2S4L).
#[derive(Serialize, Deserialize, Debug)]
pub enum C2S4L {
    GiveObjs,
    GiveMap,
    GivePlayerData,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MapObjsE {
    pub structures: Vec<StructureE>,
    pub unit_groups: Vec<UnitGroupE>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDataE {
    pub name: String,
    pub pos: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureE {
    pub name: String,
    pub struc_type: StructureTypeE,
    pub pos: (usize, usize),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum StructureTypeE {
    Castle,
    Farm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitGroupE {
    pub owner: String,
}
