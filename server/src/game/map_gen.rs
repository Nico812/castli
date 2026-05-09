use rand::Rng;

use common::{
    config::{TerrainGenConfig, config},
    map::Tile,
};

struct TerrainParams {
    spreading: Tile,
    spreads_on: &'static [Tile],
    iters: usize,
    percent: u8,
    counts_to_spread: u8,
    counts_to_survive: u8,
}

impl TerrainParams {
    fn from_config(spreading: Tile, spreads_on: &'static [Tile], cfg: &TerrainGenConfig) -> Self {
        Self {
            spreading,
            spreads_on,
            iters: cfg.iterations,
            percent: cfg.percent,
            counts_to_spread: cfg.counts_to_spread,
            counts_to_survive: cfg.counts_to_survive,
        }
    }
}

pub fn generate_tiles() -> Vec<Vec<Tile>> {
    let map_rows = config().world.map_rows;
    let map_cols = config().world.map_cols;

    let mut a = vec![vec![Tile::Grass; map_cols]; map_rows];
    let mut b = a.clone();

    let map_gen = &config().map_gen;
    let terrains = [
        TerrainParams::from_config(Tile::Water, &[Tile::Grass], &map_gen.water),
        TerrainParams::from_config(Tile::Woods, &[Tile::Grass], &map_gen.woods),
        TerrainParams::from_config(
            Tile::Mountain,
            &[Tile::Grass, Tile::Woods],
            &map_gen.mountain,
        ),
        TerrainParams::from_config(
            Tile::HighMountain,
            &[Tile::Mountain],
            &map_gen.high_mountain,
        ),
    ];

    for params in &terrains {
        run_terrain(&mut a, &mut b, params, map_rows, map_cols);
    }

    a
}

fn run_terrain(
    a: &mut Vec<Vec<Tile>>,
    b: &mut Vec<Vec<Tile>>,
    params: &TerrainParams,
    map_rows: usize,
    map_cols: usize,
) {
    let before_add_random = a.clone();
    add_random(
        a,
        params.spreading,
        params.spreads_on,
        params.percent,
        map_rows,
        map_cols,
    );
    b.clone_from(a);

    for _ in 0..params.iters {
        step_life(a, b, &before_add_random, params, map_rows, map_cols);
        std::mem::swap(a, b);
    }
}

fn add_random(
    tiles: &mut [Vec<Tile>],
    add_type: Tile,
    add_on: &[Tile],
    percent: u8,
    map_rows: usize,
    map_cols: usize,
) {
    tiles.iter_mut().enumerate().for_each(|(row, tile_row)| {
        let mut rng = rand::rng();
        for (col, tile) in tile_row.iter_mut().enumerate() {
            let is_edge = row == 0 || col == 0 || row == map_rows - 1 || col == map_cols - 1;
            let random_hit = rng.random_range(1..=100) <= percent;
            let is_valid_tile = add_on.contains(tile);

            if (is_edge || random_hit) && is_valid_tile {
                *tile = add_type;
            }
        }
    });
}

fn step_life(
    a: &[Vec<Tile>],
    b: &mut [Vec<Tile>],
    before_add_random: &[Vec<Tile>],
    params: &TerrainParams,
    map_rows: usize,
    map_cols: usize,
) {
    let spreading = params.spreading;
    let spreads_on = params.spreads_on;
    let counts_to_spread = params.counts_to_spread;
    let counts_to_survive = params.counts_to_survive;

    b.iter_mut().enumerate().for_each(|(row, b_row)| {
        if row == 0 || row == map_rows - 1 {
            return;
        }
        let prev = &a[row - 1];
        let curr = &a[row];
        let next = &a[row + 1];
        let before_row = &before_add_random[row];
        for col in 1..map_cols - 1 {
            let mut neightb_count = 0u8;
            for c in (col - 1)..=(col + 1) {
                if prev[c] == spreading {
                    neightb_count += 1;
                }
                if next[c] == spreading {
                    neightb_count += 1;
                }
            }
            if curr[col - 1] == spreading {
                neightb_count += 1;
            }
            if curr[col + 1] == spreading {
                neightb_count += 1;
            }

            let mut new_val = curr[col];
            if neightb_count >= counts_to_spread && spreads_on.contains(&new_val) {
                new_val = spreading;
            }
            if neightb_count < counts_to_survive && new_val == spreading {
                new_val = before_row[col];
            }
            b_row[col] = new_val;
        }
    });
}
