use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Tile {
    Water,
    Grass,
    Woods,
    Mountain,
    HighMountain,
    Err,
}
