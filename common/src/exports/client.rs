use serde::{Deserialize, Serialize};

use crate::GameId;

// SHould i do ClientE that keeps Option<CastleE> here too?
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerE {
    pub name: String,
    pub castle_id: Option<GameId>,
    pub lobby: usize,
}
