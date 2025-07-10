pub mod r#const;
pub mod stream;

use serde::{Deserialize, Serialize};

// Errors

#[derive(Serialize, Deserialize, Debug)]
pub enum S2C {
    LobbyFound,
    ServerFull,
    ConnectionFailed,
    L2S4C(L2S4C),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum L2S4C {
    Map(Vec<Vec<TileE>>),
    MapObjs(MapObjsE),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum C2S {
    C2S4L(C2S4L),
    Login(String),
    NewCastle((usize, usize)),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum C2S4L {
    GiveObjs,
    GiveMap,
}

// Server Exports

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
