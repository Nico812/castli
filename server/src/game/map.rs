pub struct Map{
    pub tiles: Vec<Vec<TileE>>,
}

impl Map {
    pub fn new() -> Self {
        tiles = Self::cellular_automata();
        Self{tiles}
    }

    pub fn export(&self) -> Vec<Vec<TileE>> {
        self.tiles.clone()
    }

    fn cellular_automata() -> Vec<Vec<TileE>> {
        fn fill_random(tiles: &mut Vec<Vec<TileE>>) {
            let mut rng = rand::thread_rng();

            for row in 0..MAP_ROWS {
                for col in 0..MAP_COLS {
                    if row == 0
                        || col == 0
                        || row == MAP_ROWS - 1
                        || col == MAP_COLS - 1
                        || rng.gen_range(1..=100) <= PERCENT_ARE_WALLS
                    {
                        tiles[row][col] = TileE::Water;
                    }
                }
            }
        }

        fn step_life(tiles: &Vec<Vec<TileE>>, temp_tiles: &mut Vec<Vec<TileE>>) {
            for row in 0..MAP_ROWS {
                for col in 0..MAP_COLS {
                    let mut wall_count = 0;

                    for i in row.saturating_sub(1)..=(row + 1).min(MAP_ROWS - 1) {
                        for j in col.saturating_sub(1)..=(col + 1).min(MAP_COLS - 1) {
                            if i == row && j == col {
                                continue;
                            }
                            if tiles[i][j] == TileE::Water {
                                wall_count += 1;
                            }
                        }
                    }

                    if wall_count >= 4 && temp_tiles[row][col] != TileE::Water {
                        temp_tiles[row][col] = TileE::Water;
                    }
                    if wall_count <= 2 && temp_tiles[row][col] == TileE::Water {
                        temp_tiles[row][col] = TileE::Grass;
                    }
                }
            }
        }

        let mut tiles = vec![vec![TileE::Grass; MAP_COLS]; MAP_ROWS];
        fill_random(&mut tiles);
        let mut temp_tiles = tiles.clone();

        for _ in 0..CA_ITER {
            step_life(&tiles, &mut temp_tiles);
            tiles.clone_from(&temp_tiles);
        }
        tiles
    }

    fn print(&self) {
        for rows in self.tiles.iter() {
            for tile in rows {
                if *tile == TileE::Grass {
                    print!("G");
                } else {
                    print!("W");
                }
            }
            print!("\n");
        }
    }
}