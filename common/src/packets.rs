use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    GameCoord, GameId, Time,
    game_objs::{GameObjE, OwnedCastleE},
    map::Tile,
    player::PlayerE,
    units::UnitGroup,
};

/// Represents messages sent from the Server to the Client (S2C).
#[derive(Serialize, Deserialize)]
pub enum S2C {
    LobbyFound,
    LobbyFull,
    ConnectionFailed,
    ServerShutdown,
    L2S4C(L2S4C),
}

#[derive(Serialize, Deserialize)]
pub struct MainPacket {
    pub time: Time,
    pub objs: HashMap<GameId, GameObjE>,
    pub player: PlayerE,
    pub castle: Option<OwnedCastleE>,
}

// TODO: Change this name to Feedback or something similar
#[derive(Serialize, Deserialize)]
pub enum LogE {
    CastleCreationErr,
    UnitDeployErr,
    AttackDeployErr,
}

// Represents messages sent from a Lobby, to the Server, for a Client (L2S4C).
#[derive(Serialize, Deserialize)]
pub enum L2S4C {
    MainPacket(MainPacket),
    Map(Vec<Vec<Tile>>),
    Log(LogE),
}

// Represents messages sent from the Client to the Server (C2S).
#[derive(Serialize, Deserialize)]
pub enum C2S {
    C2S4L(C2S4L),
    Login(String),
    Lobby(usize),
}

// Represents messages sent from a Client, to the Server, for the Lobby (C2S4L).
#[derive(Serialize, Deserialize)]
pub enum C2S4L {
    NewCastle(GameCoord),
    AttackCastle(GameId, UnitGroup),
    SendUnits(GameCoord, UnitGroup),
}
