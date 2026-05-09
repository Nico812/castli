use common::{
    GameCoord,
    r#const::{MAP_COLS, MAP_ROWS},
    map::Tile,
    packets::MapPayload,
};

use crate::game::map_gen;

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    obstacles: Vec<Vec<bool>>,
    occupied: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = map_gen::generate_tiles();
        let obstacles: Vec<Vec<bool>> = tiles
            .iter()
            .map(|row| row.iter().map(|t| *t == Tile::Water).collect())
            .collect();
        let occupied = vec![vec![false; MAP_COLS]; MAP_ROWS];

        println!("mappa caricata LOL");
        Self {
            tiles,
            obstacles,
            occupied,
        }
    }

    pub fn is_obstacle(&self, pos: GameCoord) -> bool {
        self.obstacles
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .copied()
            .unwrap_or(true)
    }

    pub fn is_occupied(&self, pos: GameCoord) -> bool {
        self.occupied
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .copied()
            .unwrap_or(true)
    }

    pub fn can_build(&self, pos: GameCoord, size: GameCoord) -> bool {
        let Some(end_y) = pos.y.checked_add(size.y) else {
            return false;
        };
        let Some(end_x) = pos.x.checked_add(size.x) else {
            return false;
        };

        for row in pos.y..end_y {
            for col in pos.x..end_x {
                let curr_pos = GameCoord::new(row, col);
                if self.is_occupied(curr_pos) || self.is_obstacle(curr_pos) {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_occupied(&mut self, pos: GameCoord, size: GameCoord) {
        let end_y = pos.y.saturating_add(size.y);
        let end_x = pos.x.saturating_add(size.x);

        for row in pos.y..end_y {
            if row >= MAP_ROWS {
                continue;
            }
            for col in pos.x..end_x {
                if col >= MAP_COLS {
                    continue;
                }
                self.occupied[row][col] = true;
            }
        }
    }

    pub fn get_obstacles(&self) -> &Vec<Vec<bool>> {
        &self.obstacles
    }

    pub fn get_tile(&self, pos: GameCoord) -> Option<Tile> {
        self.tiles
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .copied()
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
