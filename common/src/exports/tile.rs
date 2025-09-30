use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum TileE {
    Water,
    Grass,
    Woods,
}