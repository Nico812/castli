use std::collections::HashMap;
use tokio::{sync::mpsc, time};

use crate::{
    client::Client,
    r#const::{CLIENT_COM_TICK, GAME_TICK, SERVER_COM_TICK},
    game::game::Game,
    server::{ClientID, S2L},
};
use common::{C2S4L, L2S4C, LogE, MainPacket, r#const::MAX_LOBBY_PLAYERS};

struct ClientCh {
    tx: mpsc::UnboundedSender<L2S4C>,
    rx: mpsc::UnboundedReceiver<C2S4L>,
}

pub struct Lobby {
    id: usize,
    clients_ch: HashMap<ClientID, ClientCh>,
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

                    self.send_updates().await;
                }
            }
        }
    }

    async fn add_client(
        &mut self,
        lobby_id: usize,
        client_id: ClientID,
        client_name: String,
        client_ch: ClientCh,
    ) -> Result<(), ()> {
        if self.num_players >= MAX_LOBBY_PLAYERS {
            Err(())
        } else {
            let client = Client::new(client_name, lobby_id);
            self.num_players += 1;
            println!("New player joined in a lobby, ID: {}", client_id);

            Self::send_map(&client_ch, &client, &self.game).await;
            Self::send_main_packet(&client_ch, &client, &self.game).await;
            println!("Sent initial data to client");

            self.clients_ch.insert(client_id, client_ch);
            self.clients.insert(client_id, client);
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
                        .add_client(
                            self.id,
                            client_id,
                            client_name,
                            ClientCh {
                                tx: client_tx,
                                rx: client_rx,
                            },
                        )
                        .await
                        .inspect_err(|err| eprintln!("LOBBY ERROR: {:?}", err));
                }
                S2L::Shutdown => {
                    println!("Lobby shutting down");
                    *running = false;
                }
            };
        }
    }

    async fn listen_clients(&mut self) {
        for (client_id, client_ch) in self.clients_ch.iter_mut() {
            let Some(client) = self.clients.get_mut(client_id) else {
                continue;
            };

            if let Ok(msg) = client_ch.rx.try_recv() {
                let mut log = None;
                match msg {
                    C2S4L::NewCastle(pos) => {
                        println!("Client ({}) requested to build a new castle", client_id);
                        if client.castle_id.is_none()
                            && let Some(castle_id) =
                                self.game.add_player_castle(client.name.clone(), pos)
                        {
                            client.set_castle_id(castle_id);
                        } else {
                            log = Some(LogE::CastleCreationErr);
                        }
                    }
                    C2S4L::AttackCastle(target_id, unit_group_e) => {
                        if let Some(castle_id) = client.castle_id {
                            if !self.game.attack_castle(castle_id, target_id, unit_group_e) {
                                log = Some(LogE::AttackDeployErr);
                            }
                        }
                    }
                    C2S4L::SendUnits(target_pos, unit_group_e) => {
                        if let Some(castle_id) = client.castle_id {
                            if !self.game.request_send_units(
                                castle_id,
                                target_pos,
                                unit_group_e,
                                None,
                            ) {
                                log = Some(LogE::UnitDeployErr);
                            }
                        }
                    }
                }
                if let Some(log) = log {
                    let _ = client_ch.tx.send(L2S4C::Log(log));
                }
            }
        }
    }

    async fn send_updates(&mut self) {
        for (client_id, client_ch) in self.clients_ch.iter_mut() {
            let Some(client) = self.clients.get_mut(client_id) else {
                continue;
            };

            Self::send_main_packet(client_ch, &client, &self.game).await
        }
    }

    async fn send_map(client_ch: &ClientCh, client: &Client, game: &Game) {
        let _ = client_ch.tx.send(L2S4C::Map(game.export_map()));
    }

    async fn send_main_packet(client_ch: &ClientCh, client: &Client, game: &Game) {
        let castle = if let Some(castle_id) = client.castle_id {
            game.export_owned_castle(castle_id)
        } else {
            None
        };

        let packet = MainPacket {
            time: game.time,
            objs: game.export_objs(),
            client: client.export(),
            castle,
        };

        let _ = client_ch.tx.send(L2S4C::MainPacket(packet));
    }

    pub fn is_full(&self) -> bool {
        self.num_players >= MAX_LOBBY_PLAYERS
    }
}
