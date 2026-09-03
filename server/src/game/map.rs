use common::{
    GameCoord,
    r#const::{MAP_COLS, MAP_ROWS},
    hitboxes::Hitbox,
    map::Tile,
    packets::MapPayload,
};

use crate::game::map_gen;

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    occupied: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = map_gen::generate_tiles();
        let occupied = vec![vec![false; MAP_COLS]; MAP_ROWS];

        println!("mappa caricata LOL");
        Self { tiles, occupied }
    }

    pub fn is_water(&self, pos: GameCoord) -> bool {
        self.get_tile(pos) == Some(Tile::Water)
    }

    pub fn is_occupied(&self, pos: GameCoord) -> bool {
        self.occupied
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .copied()
            .unwrap_or(true)
    }

    pub fn can_build(&self, pos: GameCoord, hitbox: Hitbox) -> bool {
        for (i, row) in hitbox.iter().enumerate() {
            for (j, hit) in row.iter().enumerate() {
                let curr_pos = GameCoord::new(pos.y + i, pos.x + j);
                if *hit && (self.is_occupied(curr_pos) || self.is_water(curr_pos)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_occupied(&mut self, pos: GameCoord, hitbox: Hitbox) {
        for (i, row) in hitbox.iter().enumerate() {
            for (j, hit) in row.iter().enumerate() {
                let curr_pos = GameCoord::new(pos.y + i, pos.x + j);
                if *hit {
                    self.occupied
                        .get_mut(curr_pos.y)
                        .and_then(|row| row.get_mut(curr_pos.x))
                        .map(|cell| *cell = true);
                }
            }
        }
    }

    pub fn get_tile(&self, pos: GameCoord) -> Option<Tile> {
        self.tiles
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .copied()
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn get_occupied_map(&self) -> &Vec<Vec<bool>> {
        &self.occupied
    }

    pub fn export(&self) -> MapPayload {
        let rows = self.tiles.len();
        let cols = self.tiles.first().map(|row| row.len()).unwrap_or(0);
        let mut tiles = Vec::with_capacity(rows * cols);
        for row in &self.tiles {
            tiles.extend_from_slice(row);
        }
        MapPayload {
            rows: rows as u32,
            cols: cols as u32,
            tiles,
        }
    }
}
