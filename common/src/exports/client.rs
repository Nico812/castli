use serde::{Deserialize, Serialize};

use crate::GameID;

// SHould i do ClientE that keeps Option<CastleE> here too?

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientE {
    pub name: String,
    pub castle_id: Option<GameID>,
    pub lobby: usize,
}
