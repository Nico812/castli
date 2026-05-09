use common::{GameId, player::PlayerE};

use crate::server::Client;

// Players are managed at the Lobby level. Theyr info is not needed for the game. Data is retrieved when he ocnnects (TODO)
pub struct Player {
    pub client: Client,
    pub name: String,
    pub castle_id: Option<GameId>,
    pub lobby: usize,
    pub in_courtyard: bool,
}

impl Player {
    pub fn new(lobby: usize, client: Client) -> Self {
        let name = client.name.clone();

        println!("New player joined with the name: {}", client.name);
        Self {
            client,
            name,
            castle_id: None,
            lobby,
            in_courtyard: false,
        }
    }

    pub fn set_castle_id(&mut self, castle_id: GameId) {
        self.castle_id = Some(castle_id);
        println!(
            "Client {} just got a new castle with GameId {}",
            self.name, castle_id
        );
    }

    pub fn export(&self) -> PlayerE {
        PlayerE {
            name: self.name.clone(),
            castle_id: self.castle_id,
            lobby: self.lobby,
        }
    }
}
