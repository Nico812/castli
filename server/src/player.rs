// Players are managed at the Lobby level. They are linked to a castle. Castles are managed at the Game level.

use common::GameID;

// Player status is used when the user is in weird states, like he has not yet a castle (init).
// Std is the standars state.
#[derive(PartialEq)]
pub enum PlayerStatus {
    Std,
    Init,
}
pub struct Player {
    pub name: String,
    pub castle_id: Option<GameID>,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(name: String) -> Self {
        let name = name;
        let status = PlayerStatus::Init;
        println!("New player joined with the name: {}", name);
        Self {
            name,
            castle_id: None,
            status,
        }
    }

    pub fn set_castle_id(&mut self, castle_id: common::GameID) {
        self.castle_id = Some(castle_id);
        println!(
            "Player {} just got a new castle with GameID {}",
            self.name, castle_id
        );
    }
}
