use std::collections::VecDeque;

use common::{
    GameCoord, GameId,
    game_objs::DeployedUnitsE,
    map::Tile,
    units::{UnitGroup, UnitType, UnitsTarget},
};

pub enum DeployedUnitsEvent {
    AtDest,
    AtHome,
}

#[derive(Clone)]
pub struct DeployedUnits {
    pub unit_group: UnitGroup,
    owner_id: GameId,
    target: UnitsTarget,
    returning: bool,
    path: Option<VecDeque<GameCoord>>,
    path_index: usize,
    path_size: usize,
}

impl DeployedUnits {
    pub fn new(
        owner_id: GameId,
        target: UnitsTarget,
        path: Option<VecDeque<GameCoord>>,
        unit_group: UnitGroup,
    ) -> Self {
        let path_size = path.as_ref().map(|path| path.len()).unwrap_or(0);

        Self {
            owner_id,
            target,
            path_size,
            path,
            unit_group,
            path_index: 0,
            returning: false,
        }
    }

    pub fn get_pos(&self) -> Option<GameCoord> {
        self.path.as_ref().map(|path| path[self.path_index])
    }

    pub fn step(&mut self) -> Option<DeployedUnitsEvent> {
        if self.path.is_none() {
            return None;
        };

        self.path_index = match self.returning {
            true => self.path_index - 1,
            false => self.path_index + 1,
        };

        if self.path_index == self.path_size - 1 {
            self.returning = true;
            return Some(DeployedUnitsEvent::AtDest);
        };

        if self.path_index == 0 {
            return Some(DeployedUnitsEvent::AtHome);
        };

        None
    }

    pub fn get_obstacles(
        &self,
        tiles: &Vec<Vec<Tile>>,
        occupied: &Vec<Vec<bool>>,
    ) -> Vec<Vec<bool>> {
        let unit_obstacles: &[Tile] = if self.unit_group.contains(UnitType::Ship) {
            &[]
        } else {
            &[Tile::Water]
        };

        let obstacles = tiles
            .iter()
            .enumerate()
            .map(|(i, tile_row)| {
                tile_row
                    .iter()
                    .enumerate()
                    .map(|(j, tile)| unit_obstacles.contains(tile) || occupied[i][j])
                    .collect()
            })
            .collect();

        obstacles
    }

    // This needs to be fully rethinked because i want dynamic battles!
    pub fn get_strength(&self) -> u32 {
        self.unit_group.get_strength()
    }

    pub fn set_path(&mut self, path: VecDeque<GameCoord>) {
        self.path_size = path.len();
        self.path = Some(path);
    }

    pub fn get_owner_id(&self) -> GameId {
        self.owner_id
    }

    pub fn get_target(&self) -> UnitsTarget {
        self.target.clone()
    }

    pub fn export(&self) -> Option<DeployedUnitsE> {
        let Some(pos) = self.get_pos() else {
            return None;
        };

        Some(DeployedUnitsE {
            owner_id: self.owner_id,
            pos,
        })
    }
}
