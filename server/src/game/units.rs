use std::collections::VecDeque;

use common::{GameCoord, GameId, game_objs::DeployedUnitsE, units::UnitGroup};

pub enum DeployedUnitsEvent {
    AtDest,
    AtHome,
}

#[derive(Clone)]
pub struct DeployedUnits {
    unit_group: UnitGroup,
    owner_id: GameId,
    target_id: Option<GameId>,
    returning: bool,
    path: Option<VecDeque<GameCoord>>,
    path_index: usize,
    path_size: usize,
}

impl DeployedUnits {
    pub fn new(
        owner_id: GameId,
        target_id: Option<GameId>,
        path: Option<VecDeque<GameCoord>>,
        unit_group: UnitGroup,
    ) -> Self {
        let path_size = path.as_ref().map(|path| path.len()).unwrap_or(0);

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

    pub fn get_unit_group(&self) -> &UnitGroup {
        &self.unit_group
    }

    pub fn get_target(&self) -> Option<GameId> {
        self.target_id
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
