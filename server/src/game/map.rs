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
    exports::tile::TileE,
};

pub struct Map {
    pub tiles: Vec<Vec<TileE>>,
    pub obstacles: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = Self::cellular_automata();

        let mut obstacles = vec![vec![false; MAP_COLS]; MAP_ROWS];
        for (r, row) in tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == TileE::Water {
                    obstacles[r][c] = true;
                }
            }
        }

        Self { tiles, obstacles }
    }

    pub fn is_obstacle(&self, coord: GameCoord) -> bool {
        self.obstacles[coord.y][coord.x]
    }

    pub fn export(&self) -> Vec<Vec<TileE>> {
        self.tiles.clone()
    }

    fn cellular_automata() -> Vec<Vec<TileE>> {
        let mut tiles = vec![vec![TileE::Grass; MAP_COLS]; MAP_ROWS];

        // Creating oceans
        let mut before_add_random = tiles.clone();
        let water_spreads_on = vec![TileE::Grass];
        Self::add_random(
            &mut tiles,
            TileE::Water,
            &water_spreads_on,
            PERCENT_IS_WATER,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_WATER {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                TileE::Water,
                &water_spreads_on,
                COUNTS_TO_SPREAD_WATER,
                COUNTS_TO_SURVIVE_WATER,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating woodlands
        before_add_random.clone_from(&tiles);
        let woods_spreads_on = vec![TileE::Grass];
        Self::add_random(
            &mut tiles,
            TileE::Woods,
            &woods_spreads_on,
            PERCENT_IS_WOODS,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_WOODS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                TileE::Woods,
                &woods_spreads_on,
                COUNTS_TO_SPREAD_WOODS,
                COUNTS_TO_SURVIVE_WOODS,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating mountains
        before_add_random = tiles.clone();
        let mountains_spreads_on = vec![TileE::Grass, TileE::Woods];
        Self::add_random(
            &mut tiles,
            TileE::Mountain,
            &mountains_spreads_on,
            PERCENT_IS_MOUNTAINS,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_MOUNTAINS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                TileE::Mountain,
                &mountains_spreads_on,
                COUNTS_TO_SPREAD_MOUNTAINS,
                COUNTS_TO_SURVIVE_MOUNTAINS,
            );
            tiles.clone_from(&temp_tiles);
        }

        // Creating high mountains
        before_add_random = tiles.clone();
        let high_mountains_spreads_on = vec![TileE::Mountain];
        Self::add_random(
            &mut tiles,
            TileE::HighMountain,
            &high_mountains_spreads_on,
            PERCENT_IS_HIGH_MOUNTAINS,
        );

        let mut temp_tiles = tiles.clone();
        for _ in 0..CA_ITER_HIGH_MOUNTAINS {
            Self::step_life(
                &tiles,
                &mut temp_tiles,
                &before_add_random,
                TileE::HighMountain,
                &high_mountains_spreads_on,
                COUNTS_TO_SPREAD_HIGH_MOUNTAINS,
                COUNTS_TO_SURVIVE_HIGH_MOUNTAINS,
            );
            tiles.clone_from(&temp_tiles);
        }
        tiles
    }

    fn add_random(tiles: &mut Vec<Vec<TileE>>, add_type: TileE, add_on: &Vec<TileE>, percent: u8) {
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
        tiles: &Vec<Vec<TileE>>,
        temp_tiles: &mut Vec<Vec<TileE>>,
        before_add_random: &Vec<Vec<TileE>>,
        spreading_type: TileE,
        spreads_on: &Vec<TileE>,
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
}
