use std::collections::VecDeque;

use common::{GameCoord, GameId, game_objs::DeployedUnitsE, units::UnitGroup};

pub struct DeployedUnits {
    pub owner_id: GameId,
    pub target_id: Option<GameId>,
    pub returning: bool,
    path: Option<VecDeque<GameCoord>>,
    path_index: usize,
    path_size: usize,
    pub unit_group: UnitGroup,
}

impl DeployedUnits {
    pub fn new(
        owner_id: GameId,
        target_id: Option<GameId>,
        path: Option<VecDeque<GameCoord>>,
        unit_group: UnitGroup,
    ) -> Self {
        let path_size = match path {
            Some(ref path) => path.len(),
            None => 0,
        };
        Self {
            owner_id,
            target_id,
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

    pub fn move_along_path(&mut self) {
        if let Some(ref path) = self.path {
            let next_index: usize = match self.returning {
                true => self.path_index.saturating_sub(1),
                false => (self.path_index + 1).min(self.path_size - 1),
            };

            if path.get(next_index).is_some() {
                self.path_index = next_index;
            }
        }
    }

    pub fn pending(&self) -> bool {
        if self.path_index == 0 && self.returning {
            return true;
        }
        if self.path_index >= self.path_size.saturating_sub(1) && !self.returning {
            return true;
        }
        false
    }

    pub fn arrived_home(&self) -> bool {
        self.pending() && self.returning
    }

    pub fn arrived_target(&self) -> bool {
        self.pending() && !self.returning
    }

    // This needs to be fully rethinked because i want dynamic battles!
    pub fn get_strength(&self) -> u32 {
        self.unit_group.get_strength()
    }

    pub fn r#return(&mut self) {
        self.returning = true;
    }

    pub fn set_path(&mut self, path: VecDeque<GameCoord>) {
        self.path_size = path.len();
        self.path = Some(path);
    }

    pub fn export(&self) -> DeployedUnitsE {
        DeployedUnitsE {
            owner_id: self.owner_id,
            pos: self.get_pos().unwrap(),
        }
    }
}
