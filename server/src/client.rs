// Clients are managed at the Lobby level. They are linked to a castle. Castles are managed at the Game level.

use common::{GameID, exports::client::ClientE};

pub struct Client {
    pub name: String,
    pub castle_id: Option<GameID>,
    pub lobby: usize,
}

impl Client {
    pub fn new(name: String, lobby: usize) -> Self {
        println!("New player joined with the name: {}", name);
        Self {
            name,
            castle_id: None,
            lobby,
        }
    }

    pub fn set_castle_id(&mut self, castle_id: common::GameID) {
        self.castle_id = Some(castle_id);
        println!(
            "Client {} just got a new castle with GameID {}",
            self.name, castle_id
        );
    }

    pub fn export(&self) -> ClientE {
        ClientE {
            name: self.name.clone(),
            castle_id: self.castle_id,
            lobby: self.lobby,
        }
    }
}
