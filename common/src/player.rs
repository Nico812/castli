use serde::{Deserialize, Serialize};

use crate::GameId;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerE {
    pub name: String,
    pub castle_id: Option<GameId>,
    pub lobby: usize,
}
