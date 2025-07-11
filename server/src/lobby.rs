use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::time;

use crate::game;
use crate::player;
use crate::server;
use common;
use common::r#const::MAX_LOBBY_PLAYERS;

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

    pub async fn run(&mut self, mut main_rx: mpsc::UnboundedReceiver<server::S2L>) {
        let mut tick = time::interval(time::Duration::from_millis(1000));
        let running = true;

        while running {
            tokio::select! {
                Some(msg) =  main_rx.recv()=> {
                    match msg {
                        server::S2L::IsFull(temp_tx) => {
                            let _ = temp_tx
                                .send(self.is_full()).await;
                        }
                        server::S2L::NewClient(client_id,client_tx ,client_rx ) => {
                            self.add_client(client_id, client_tx,client_rx).await.inspect_err(|err| eprintln!("\x1b[35mLOBBY ERROR: {:?}\x1b[0m", err));
                        }
                    };
                }
                _ = self.listen_clients_request()=> {}
                _ = tick.tick() => {self.step();}
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
        if self.num_players < MAX_LOBBY_PLAYERS {
            for iter in 0..MAX_LOBBY_PLAYERS {
                if self.players[iter].is_none() {
                    let new_player = player::Player::new("No Name Set");
                    self.players[iter] = Some(new_player);
                    self.num_players += 1;
                    self.clients.insert(client_id, (client_tx, client_rx));

                    println!("New player joined in a lobby, ID: {}", client_id);
                    return Ok(());
                }
            }
        }
        Err(LobbyErr::AddClientFail)
    }

    async fn listen_clients_request(&mut self) {
        for (_, (client_tx, client_rx)) in self.clients.iter_mut() {
            if let Ok(msg) = client_rx.try_recv() {
                match msg {
                    common::C2S4L::GiveMap => {
                        let _ = client_tx.send(common::L2S4C::Map(self.game.export_map()));
                    }
                    common::C2S4L::GiveObjs => {
                        let _ = client_tx.send(common::L2S4C::MapObjs(self.game.export_objs()));
                    }
                };
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_players == MAX_LOBBY_PLAYERS
    }
}
