use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use common::{
    GameId,
    r#const::MAX_LOBBY_PLAYERS,
    courtyard::Facility,
    packets::{C2S4L, CourtyardPacket, L2S4C, LogE, MainPacket},
    player::PlayerE,
};

use crate::{
    r#const::{GAME_TICK, LOBBY_POOL_LEN},
    game::game::Game,
    server::{Client, ClientId, S2L},
    thread_pool::ThreadPool,
};

struct ClientCh {
    tx: Sender<L2S4C>,
    rx: Receiver<C2S4L>,
}

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

    pub fn set_castle_id(&mut self, castle_id: common::GameId) {
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

pub struct Lobby {
    id: usize,
    clients_ch: HashMap<ClientId, ClientCh>,
    players: HashMap<ClientId, Player>,
    num_players: usize,
    game: Game,
    pool: ThreadPool,
}

impl Lobby {
    pub fn new(id: usize) -> Self {
        let players = HashMap::new();
        let clients_ch = HashMap::new();
        let num_players = 0;
        let game = Game::new();
        let pool = ThreadPool::new(LOBBY_POOL_LEN);

        println!("New lobby initialized");

        Self {
            id,
            players,
            clients_ch,
            num_players,
            game,
            pool,
        }
    }

    pub fn run(mut self, mut main_rx: Receiver<S2L>) {
        let tick_duration = Duration::from_millis(GAME_TICK);
        let mut next_tick = Instant::now();
        let mut running = true;

        while running {
            self.listen_server(&mut main_rx, &mut running);
            self.listen_clients();

            let dead_castles = self.game.step();
            for dead_castle in dead_castles.iter() {
                if let Some((_, player)) = self
                    .players
                    .iter_mut()
                    .find(|(_, player)| Some(*dead_castle) == player.castle_id)
                {
                    player.castle_id = None;
                }
            }

            self.send_updates();

            next_tick += tick_duration;
            thread::sleep(next_tick.saturating_duration_since(Instant::now()));
        }
    }

    fn add_player(&mut self, client: Client, client_ch: ClientCh) {
        let client_id = client.id;
        let player = Player::new(self.id, client);
        self.num_players += 1;
        println!("New player joined in a lobby, ID: {}", client_id);

        Self::send_map(&client_ch, &self.game);
        Self::send_main_packet(&client_ch, &player, &self.game);
        println!("Sent initial data to client");

        self.clients_ch.insert(client_id, client_ch);
        self.players.insert(client_id, player);
    }

    fn listen_server(&mut self, main_rx: &mut Receiver<S2L>, running: &mut bool) {
        if let Ok(msg) = main_rx.try_recv() {
            match msg {
                S2L::IsFull(temp_tx) => {
                    let _ = temp_tx.send(self.is_full());
                }
                S2L::NewClient(client, client_tx, client_rx) => {
                    let _ = self.add_player(
                        client,
                        ClientCh {
                            tx: client_tx,
                            rx: client_rx,
                        },
                    );
                }
                S2L::Shutdown => {
                    println!("Lobby shutting down");
                    *running = false;
                }
                S2L::Disconnection(client_id) => {
                    println!("Removed client from lobby");
                    self.clients_ch.remove_entry(&client_id);
                    self.players.remove_entry(&client_id);
                }
            };
        }
    }

    fn listen_clients(&mut self) {
        for (client_id, client_ch) in self.clients_ch.iter_mut() {
            let Some(player) = self.players.get_mut(client_id) else {
                continue;
            };

            if let Ok(msg) = client_ch.rx.try_recv() {
                let mut log = None;
                match msg {
                    C2S4L::NewCastle(pos) => {
                        println!("Client ({}) requested to build a new castle", client_id);
                        if player.castle_id.is_none()
                            && let Some(castle_id) =
                                self.game.add_player_castle(player.name.clone(), pos)
                        {
                            player.set_castle_id(castle_id);
                        } else {
                            log = Some(LogE::CastleCreationErr);
                        }
                    }
                    C2S4L::AttackCastle(target_id, unit_group_e) => {
                        if let Some(castle_id) = player.castle_id {
                            if !self.game.attack_castle(
                                castle_id,
                                target_id,
                                unit_group_e,
                                &self.pool,
                            ) {
                                log = Some(LogE::AttackDeployErr);
                            }
                        }
                    }
                    C2S4L::SendUnits(target_pos, unit_group_e) => {
                        if let Some(castle_id) = player.castle_id {
                            if !self.game.request_send_units(
                                castle_id,
                                target_pos,
                                unit_group_e,
                                None,
                                &self.pool,
                            ) {
                                log = Some(LogE::UnitDeployErr);
                            }
                        }
                    }
                    C2S4L::InCourtyard => {
                        player.in_courtyard = true;
                    }
                    C2S4L::OutCourtyard => {
                        player.in_courtyard = false;
                    }
                    C2S4L::NewFacility((pos, facility_type)) => {
                        let Some(castle_id) = player.castle_id else {
                            continue;
                        };
                        let Some(castle) = self.game.get_castle_mut(castle_id) else {
                            continue;
                        };

                        if !castle
                            .courtyard
                            .add(&mut castle.resources, Facility::new(facility_type, 1, pos))
                        {
                            log = Some(LogE::FacilityCreationErr);
                        }
                    }
                }
                if let Some(log) = log {
                    let _ = client_ch.tx.send(L2S4C::Log(log));
                }
            }
        }
    }

    fn send_updates(&mut self) {
        for (client_id, client_ch) in self.clients_ch.iter_mut() {
            let Some(player) = self.players.get_mut(client_id) else {
                continue;
            };

            match player.in_courtyard {
                false => Self::send_main_packet(client_ch, &player, &self.game),
                true => Self::send_courtyard_packet(client_ch, &player, &self.game),
            }
        }
    }

    fn send_map(client_ch: &ClientCh, game: &Game) {
        let _ = client_ch.tx.send(L2S4C::Map(game.export_map()));
    }

    fn send_main_packet(client_ch: &ClientCh, player: &Player, game: &Game) {
        let castle_export = player
            .castle_id
            .map(|castle_id| {
                game.get_castle(castle_id)
                    .map(|castle| castle.export_owned())
            })
            .flatten();

        let packet = MainPacket {
            time: game.time,
            objs: game.export_objs(),
            player: player.export(),
            castle: castle_export,
        };

        let _ = client_ch.tx.send(L2S4C::MainPacket(packet));
    }

    fn send_courtyard_packet(client_ch: &ClientCh, player: &Player, game: &Game) {
        let Some(castle_id) = player.castle_id else {
            return;
        };
        let Some(castle) = game.get_castle(castle_id) else {
            return;
        };

        let packet = CourtyardPacket {
            time: game.time,
            player: player.export(),
            castle: castle.export_owned(),
            facilities: castle.courtyard.export(),
        };

        let _ = client_ch.tx.send(L2S4C::CourtyardPacket(packet));
    }

    pub fn is_full(&self) -> bool {
        self.num_players >= MAX_LOBBY_PLAYERS
    }
}
