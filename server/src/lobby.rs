//! # Game Lobby Management
//!
//! This module defines the `Lobby` struct, which represents a single game session.
//! A lobby contains a group of players, a game instance, and manages the communication
//! between them.
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time;

use crate::game;
use crate::player;
use crate::server;
use common;
use common::r#const::MAX_LOBBY_PLAYERS;

/// Represents errors that can occur within a `Lobby`.
#[derive(Debug)]
enum LobbyErr {
    AddClientFail,
}

pub struct Lobby {
    clients: HashMap<
        server::ClientID,
        (
            mpsc::UnboundedSender<common::L2S4C>,
            mpsc::UnboundedReceiver<common::C2S4L>,
        ),
    >,
    players: HashMap<Server::ClientID, player::Player>,
    num_players: usize,
    game: game::Game,
}

impl Lobby {
    pub fn new() -> Self {
        let clients = HashMap::new();
        let players = HashMap::new();
        let num_players = 0;
        let game = game::Game::new();

        println!("New lobby initialized");

        Self {
            clients,
            players,
            num_players,
            game,
        }
    }

    /// Runs the main loop for the lobby.
    ///
    /// This loop listens for messages from the server, listens and responds to messages from clients,
    /// and periodically updates the game state.
    pub async fn run(&mut self, mut main_rx: mpsc::UnboundedReceiver<server::S2L>) {
        let mut client_comunication_tick = time::interval(time::Duration::from_millis(100));
        let mut server_comunication_tick = time::interval(time::Duration::from_millis(1000));
        let mut game_tick = time::interval(time::Duration::from_millis(1000));

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
                }
            }
        }
    }

    async fn add_client(
        &mut self,
        client_id: server::ClientID,
        player_name: String,
        client_tx: mpsc::UnboundedSender<common::L2S4C>,
        client_rx: mpsc::UnboundedReceiver<common::C2S4L>,
    ) -> Result<(), LobbyErr> {
        if self.num_players >= MAX_LOBBY_PLAYERS {
            Err(LobbyErr::AddClientFail)
        }
        else {
            let player = player::Player::new(player_name);
            self.clients.insert(client_id, (client_tx, client_rx));
            self.players.insert(client_id, player);
            self.num_players += 1;
            println!("New player joined in a lobby, ID: {}", client_id);
            Ok(())
        }
    }

    async fn listen_server(
        &mut self,
        main_rx: &mut mpsc::UnboundedReceiver<server::S2L>,
        running: &mut bool,
    ) {
        if let Ok(msg) = main_rx.try_recv() {
            match msg {
                server::S2L::IsFull(temp_tx) => {
                    let _ = temp_tx.send(self.is_full()).await;
                }
                server::S2L::NewClient(client_id, player_name, client_tx, client_rx) => {
                    let _ = self
                        .add_client(client_id, player_name, client_tx, client_rx)
                        .await
                        .inspect_err(|err| eprintln!("\x1b[35mLOBBY ERROR: {:?}\x1b[0m", err));
                }
                server::S2L::Shutdown => {
                    println!("Lobby shutting down");
                    *running = false;
                }
            };
        }
    }

    async fn listen_clients(&mut self) {
        for (client_id, (client_tx, client_rx)) in self.clients.iter_mut() {
            if let Ok(msg) = client_rx.try_recv() {
                match msg {
                    common::C2S4L::NewCastle(pos) => {
                        println!("Player requested to build a new castle, ID: {}", client_id);
                        // Here i should get the ClientID and updating the Players with the new castle GameID. The two ID are different.
                        // The game itself should manage the castle ID! So i wont pass clientId
                        if let Some(player) = self.players.get(*client_id) {
                            let castle_id = self.game.add_player_castle(player.name, pos);
                            player.set_castle_id(castle_id);
                        };
                    }
                    common::C2S4L::GiveMap => {
                        println!("Player requested to give map, ID: {}", client_id);
                        let _ = client_tx.send(common::L2S4C::Map(self.game.export_map()));
                    }
                    common::C2S4L::GiveObjs => {
                        println!("Player requested to give objs, ID: {}", client_id);
                        let _ = client_tx.send(common::L2S4C::GameObjs(self.game.export_objs()));
                    }
                    common::C2S4L::GivePlayerData => {
                        println!("Player requested to give player data, ID: {}", client_id);
                        if let Some(player) = self.players.get(*client_id) {
                            if player.has_castle() {
                                let _ = client_tx.send(common::L2S4C::PlayerData(
                                    self.game.export_player_data(*client_id),
                                ));
                            } else {
                                let _ = client_tx.send(common::L2S4C::CreateCastle);
                            }
                        };
                    }
                };
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_players >= MAX_LOBBY_PLAYERS
    }
}
