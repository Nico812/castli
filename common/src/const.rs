use crate::GameCoord;

// Used as the array dimensions of the courtyard occupancy grid.
pub const COURTYARD_ROWS: usize = 60;
pub const COURTYARD_COLS: usize = 60;

// Used to size the per-server fixed-size lobby table.
pub const MAX_LOBBIES: usize = 10;

// Sizes consumed by const fns (e.g. FacilityType::size).
pub const CASTLE_SIZE: GameCoord = GameCoord::new(2, 1);
pub const FARM_PLOT_SIZE: GameCoord = GameCoord::new(8, 6);
