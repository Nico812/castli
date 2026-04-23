use std::collections::HashMap;
use tokio::{sync::mpsc, time};

use crate::{
    client::Client,
    r#const::{CLIENT_COM_TICK, GAME_TICK, SERVER_COM_TICK},
    game::game::Game,
    server::{ClientID, S2L},
};
use common::{C2S4L, L2S4C, r#const::MAX_LOBBY_PLAYERS};

#[derive(Debug)]
enum LobbyErr {
    AddClientFail,
}

pub struct Lobby {
    id: usize,
    clients_ch: HashMap<ClientID, (mpsc::UnboundedSender<L2S4C>, mpsc::UnboundedReceiver<C2S4L>)>,
    clients: HashMap<ClientID, Client>,
    num_players: usize,
    game: Game,
}

impl Lobby {
    pub fn new(id: usize) -> Self {
        let clients = HashMap::new();
        let clients_ch = HashMap::new();
        let num_players = 0;
        let game = Game::new();

        println!("New lobby initialized");

        Self {
            id,
            clients,
            clients_ch,
            num_players,
            game,
        }
    }

    /// Lobby listens for messages from the server, listens and responds to messages from clients,
    /// and periodically updates the game state.
    pub async fn run(mut self, mut main_rx: mpsc::UnboundedReceiver<S2L>) {
        let mut client_comunication_tick =
            time::interval(time::Duration::from_millis(CLIENT_COM_TICK));
        let mut server_comunication_tick =
            time::interval(time::Duration::from_millis(SERVER_COM_TICK));
        let mut game_tick = time::interval(time::Duration::from_millis(GAME_TICK));

        let mut running = true;

        while running {
            tokio::select! {
                _ = server_comunication_tick.tick() => {
                    self.listen_server(&mut main_rx, &mut running).await;
                }
                _ = client_comunication_tick.tick() => {
                    self.listen_clients().await;
                }
                _ = game_tick.tick() => {
                    let dead_castles = self.game.step().await;
                    for dead_castle in dead_castles.iter(){
                        if let Some((_, client)) = self.clients.iter_mut().find(|(_, client)|{ Some(*dead_castle) == client.castle_id}) {
                            client.castle_id = None;
                        }
                    }
                }
            }
        }
    }

    async fn add_client(
        &mut self,
        lobby_id: usize,
        client_id: ClientID,
        client_name: String,
        client_tx: mpsc::UnboundedSender<L2S4C>,
        client_rx: mpsc::UnboundedReceiver<C2S4L>,
    ) -> Result<(), LobbyErr> {
        if self.num_players >= MAX_LOBBY_PLAYERS {
            Err(LobbyErr::AddClientFail)
        } else {
            let client = Client::new(client_name, lobby_id);
            self.clients_ch.insert(client_id, (client_tx, client_rx));
            self.clients.insert(client_id, client);
            self.num_players += 1;
            println!("New player joined in a lobby, ID: {}", client_id);
            Ok(())
        }
    }

    async fn listen_server(
        &mut self,
        main_rx: &mut mpsc::UnboundedReceiver<S2L>,
        running: &mut bool,
    ) {
        if let Ok(msg) = main_rx.try_recv() {
            match msg {
                S2L::IsFull(temp_tx) => {
                    let _ = temp_tx.send(self.is_full()).await;
                }
                S2L::NewClient(client_id, client_name, client_tx, client_rx) => {
                    let _ = self
                        .add_client(self.id, client_id, client_name, client_tx, client_rx)
                        .await
                        .inspect_err(|err| eprintln!("\x1b[35mLOBBY ERROR: {:?}\x1b[0m", err));
                }
                S2L::Shutdown => {
                    println!("Lobby shutting down");
                    *running = false;
                }
            };
        }
    }

    async fn listen_clients(&mut self) {
        for (client_id, (client_tx, client_rx)) in self.clients_ch.iter_mut() {
            let Some(client) = self.clients.get_mut(client_id) else {
                continue;
            };

            if let Ok(msg) = client_rx.try_recv() {
                let mut log = None;
                match msg {
                    C2S4L::NewCastle(pos) => {
                        println!("Client ({}) requested to build a new castle", client_id);
                        if client.castle_id.is_none()
                            && let Some(castle_id) =
                                self.game.add_player_castle(client.name.clone(), pos)
                        {
                            client.set_castle_id(castle_id);
                            log = Some("Castle created successfully.".to_string());
                        } else {
                            log = Some("Castle could not be created.".to_string());
                        }
                    }
                    C2S4L::AttackCastle(target_id, unit_group_e) => {
                        if let Some(castle_id) = client.castle_id {
                            match self.game.attack_castle(castle_id, target_id, unit_group_e) {
                                true => {
                                    log = Some(format!("Attacking castle with ID: {}", target_id))
                                }
                                false => {
                                    log = Some(format!(
                                        "Failed attacking castle with ID: {}",
                                        target_id
                                    ))
                                }
                            }
                        }
                    }
                    C2S4L::SendUnits(target_pos, unit_group_e) => {
                        if let Some(castle_id) = client.castle_id {
                            match self.game.request_send_units(
                                castle_id,
                                target_pos,
                                unit_group_e,
                                None,
                            ) {
                                true => {
                                    log = Some(format!("Sending units to {}", target_pos));
                                }
                                false => {
                                    log = Some(format!("Failed sending units to: {}", target_pos));
                                }
                            }
                        }
                    }
                    C2S4L::GiveMap => {
                        let _ = client_tx.send(L2S4C::Map(self.game.export_map()));
                    }
                    C2S4L::GiveObjs => {
                        let _ = client_tx.send(L2S4C::GameObjs(self.game.export_objs()));
                    }
                    C2S4L::GiveOwnedCastle => {
                        if let Some(castle_id) = client.castle_id
                            && let Some(castle) = self.game.export_owned_castle(castle_id)
                        {
                            let _ = client_tx.send(L2S4C::OwnedCastle(castle));
                        } else {
                            let _ = client_tx.send(L2S4C::CreateCastle);
                        }
                    }
                    C2S4L::GiveClient => {
                        let _ = client_tx.send(L2S4C::Client(client.export()));
                    }
                };
                if let Some(log) = log {
                    let _ = client_tx.send(L2S4C::Log(log));
                }
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_players >= MAX_LOBBY_PLAYERS
    }
}
