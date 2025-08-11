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
    players: Box<[Option<player::Player>; MAX_LOBBY_PLAYERS]>,
    num_players: usize,
    game: game::Game,
}

impl Lobby {
    pub fn new() -> Self {
        let clients = HashMap::new();
        let players = Box::new([const { None }; MAX_LOBBY_PLAYERS]);
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
                    self.step();
                }
            }
        }
    }

    fn step(&mut self) {}

    async fn add_client(
        &mut self,
        client_id: server::ClientID,
        client_tx: mpsc::UnboundedSender<common::L2S4C>,
        client_rx: mpsc::UnboundedReceiver<common::C2S4L>,
    ) -> Result<(), LobbyErr> {
        if self.num_players >= MAX_LOBBY_PLAYERS {
            return Err(LobbyErr::AddClientFail);
        }
        if let Some(player_slot) = self.players.iter_mut().find(|player| player.is_none()) {
            *player_slot = Some(player::Player::new("No Name Set"));
            self.num_players += 1;
            self.clients.insert(client_id, (client_tx, client_rx));

            println!("New player joined in a lobby, ID: {}", client_id);
            return Ok(());
        } else {
            println!("This should never have been printed. Lobby num_players is incorrect.");
            return Err(LobbyErr::AddClientFail);
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
                server::S2L::NewClient(client_id, client_tx, client_rx) => {
                    let _ = self
                        .add_client(client_id, client_tx, client_rx)
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
                    common::C2S4L::GiveMap => {
                        println!("Player requested to give map, ID: {}", client_id);
                        let _ = client_tx.send(common::L2S4C::Map(self.game.export_map()));
                    }
                    common::C2S4L::GiveObjs => {
                        let _ = client_tx.send(common::L2S4C::MapObjs(self.game.export_objs()));
                    }
                    common::C2S4L::GivePlayerData => {
                        let _ = client_tx.send(common::L2S4C::PlayerData(self.game.export_player_data(client_id)));
                    }
                };
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_players >= MAX_LOBBY_PLAYERS
    }
}
