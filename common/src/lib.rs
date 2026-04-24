pub mod r#const;
pub mod exports;
pub mod stream;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

use crate::exports::{
    client::ClientE, game_object::GameObjE, owned_castle::OwnedCastleE, tile::TileE,
    units::UnitGroupE,
};

/// Global IDs for game objects
pub type GameID = usize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameCoord {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for GameCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Time {
    tick_cnt: u16,
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
        if self.tick_cnt == 10 {
            self.h += 1;
            if self.h == 24 {
                self.h = 0;
            } else if self.h == 6 {
                self.night = false;
            } else if self.h == 21 {
                self.night = true;
            }
            self.tick_cnt = 0;
        }
    }
}

/// Represents messages sent from the Server to the Client (S2C).
#[derive(Serialize, Deserialize)]
pub enum S2C {
    LobbyFound,
    ServerFull,
    ConnectionFailed,
    ServerShutdown,
    L2S4C(L2S4C),
}

#[derive(Serialize, Deserialize)]
pub struct MainPacket {
    pub time: Time,
    pub objs: HashMap<GameID, GameObjE>,
    pub client: ClientE,
    pub castle: Option<OwnedCastleE>,
}

#[derive(Serialize, Deserialize)]
pub enum LogE {
    CastleCreationErr,
    UnitDeployErr,
    AttackDeployErr,
}

/// Represents messages sent from a Lobby, to the Server, for a Client (L2S4C).
/// These are game-specific messages.
#[derive(Serialize, Deserialize)]
pub enum L2S4C {
    MainPacket(MainPacket),
    Map(Vec<Vec<TileE>>),
    Log(LogE),
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
}
