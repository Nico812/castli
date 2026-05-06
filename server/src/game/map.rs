use rand::Rng;

use crate::r#const::{
    CA_ITER_HIGH_MOUNTAINS, CA_ITER_MOUNTAINS, CA_ITER_WATER, CA_ITER_WOODS,
    COUNTS_TO_SPREAD_HIGH_MOUNTAINS, COUNTS_TO_SPREAD_MOUNTAINS, COUNTS_TO_SPREAD_WATER,
    COUNTS_TO_SPREAD_WOODS, COUNTS_TO_SURVIVE_HIGH_MOUNTAINS, COUNTS_TO_SURVIVE_MOUNTAINS,
    COUNTS_TO_SURVIVE_WATER, COUNTS_TO_SURVIVE_WOODS, PERCENT_IS_HIGH_MOUNTAINS,
    PERCENT_IS_MOUNTAINS, PERCENT_IS_WATER, PERCENT_IS_WOODS,
};
use common::{
    GameCoord,
    r#const::{MAP_COLS, MAP_ROWS},
    map::Tile,
    packets::MapPayload,
};

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    obstacles: Vec<Vec<bool>>,
    occupied: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = Self::cellular_automata();
        let mut obstacles = vec![vec![false; MAP_COLS]; MAP_ROWS];
        let occupied = vec![vec![false; MAP_COLS]; MAP_ROWS];

        for (r, row) in tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == Tile::Water {
                    obstacles[r][c] = true;
                }
            }
        }

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
            .map(|row| row.get(pos.x))
            .flatten()
            .copied()
            .unwrap_or(true)
    }

    pub fn is_occupied(&self, pos: GameCoord) -> bool {
        self.occupied
            .get(pos.y)
            .map(|row| row.get(pos.x))
            .flatten()
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
            .map(|row| row.get(pos.x))
            .flatten()
            .copied()
    }

    fn cellular_automata() -> Vec<Vec<Tile>> {
        let mut tiles = vec![vec![Tile::Grass; MAP_COLS]; MAP_ROWS];

        // Creating oceans
        let mut before_add_random = tiles.clone();
        let water_spreads_on = [Tile::Grass];
        Self::add_random(&mut tiles, Tile::Water, &water_spreads_on, PERCENT_IS_WATER);

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_WATER {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                Tile::Water,
                &water_spreads_on,
                COUNTS_TO_SPREAD_WATER,
                COUNTS_TO_SURVIVE_WATER,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating woodlands
        before_add_random.clone_from(&tiles);
        let woods_spreads_on = [Tile::Grass];
        Self::add_random(&mut tiles, Tile::Woods, &woods_spreads_on, PERCENT_IS_WOODS);

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_WOODS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                Tile::Woods,
                &woods_spreads_on,
                COUNTS_TO_SPREAD_WOODS,
                COUNTS_TO_SURVIVE_WOODS,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating mountains
        before_add_random = tiles.clone();
        let mountains_spreads_on = [Tile::Grass, Tile::Woods];
        Self::add_random(
            &mut tiles,
            Tile::Mountain,
            &mountains_spreads_on,
            PERCENT_IS_MOUNTAINS,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_MOUNTAINS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                Tile::Mountain,
                &mountains_spreads_on,
                COUNTS_TO_SPREAD_MOUNTAINS,
                COUNTS_TO_SURVIVE_MOUNTAINS,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating high mountains
        before_add_random = tiles.clone();
        let high_mountains_spreads_on = [Tile::Mountain];
        Self::add_random(
            &mut tiles,
            Tile::HighMountain,
            &high_mountains_spreads_on,
            PERCENT_IS_HIGH_MOUNTAINS,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_HIGH_MOUNTAINS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                Tile::HighMountain,
                &high_mountains_spreads_on,
                COUNTS_TO_SPREAD_HIGH_MOUNTAINS,
                COUNTS_TO_SURVIVE_HIGH_MOUNTAINS,
            );
            tiles.clone_from(&temp_tiles);
        }
        tiles
    }

    fn add_random(tiles: &mut [Vec<Tile>], add_type: Tile, add_on: &[Tile], percent: u8) {
        let mut rng = rand::rng();

        for row in 0..MAP_ROWS {
            for col in 0..MAP_COLS {
                let is_edge = row == 0 || col == 0 || row == MAP_ROWS - 1 || col == MAP_COLS - 1;
                let random_hit = rng.random_range(1..=100) <= percent;
                let is_valid_tile = add_on.contains(&tiles[row][col]);

                if (is_edge || random_hit) && is_valid_tile {
                    tiles[row][col] = add_type;
                }
            }
        }
    }

    fn step_life(
        tiles: &[Vec<Tile>],
        temp_tiles: &mut [Vec<Tile>],
        before_add_random: &[Vec<Tile>],
        spreading_type: Tile,
        spreads_on: &[Tile],
        counts_to_spread: u8,
        counts_to_survive: u8,
    ) {
        for row in 1..MAP_ROWS - 1 {
            for col in 1..MAP_COLS - 1 {
                let mut neightb_count = 0;

                for i in row.saturating_sub(1)..=(row + 1).min(MAP_ROWS - 1) {
                    for j in col.saturating_sub(1)..=(col + 1).min(MAP_COLS - 1) {
                        if i == row && j == col {
                            continue;
                        }
                        if tiles[i][j] == spreading_type {
                            neightb_count += 1;
                        }
                    }
                }

                if neightb_count >= counts_to_spread && spreads_on.contains(&temp_tiles[row][col]) {
                    temp_tiles[row][col] = spreading_type;
                }
                if neightb_count < counts_to_survive && temp_tiles[row][col] == spreading_type {
                    let new_tile = before_add_random[row][col];
                    temp_tiles[row][col] = new_tile;
                }
            }
        }
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
