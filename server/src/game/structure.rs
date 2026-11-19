use common::{
    GameCoord, LimitedGroup,
    game_objs::{StructureE, StructureType},
    map::Tile,
};

use crate::game::game::{Attackable, KillRewards};

pub struct Structure {
    is_alive: bool,
    r#type: StructureType,
    pos: GameCoord,
}

impl Structure {
    pub fn new(ty: StructureType, pos: GameCoord) -> Self {
        Self {
            is_alive: true,
            r#type: ty,
            pos,
        }
    }

    pub fn export(&self) -> StructureE {
        StructureE {
            is_alive: true,
            r#type: self.r#type,
            pos: self.pos,
        }
    }

    pub fn get_pos(&self) -> GameCoord {
        self.pos
    }

    pub fn spawns_on(ty: StructureType) -> Vec<Tile> {
        match ty {
            StructureType::LoneFarm => vec![Tile::Grass],
            StructureType::NonEuclideanTown => vec![Tile::Woods],
            StructureType::DragonsLair => vec![Tile::Mountain, Tile::HighMountain],
        }
    }

    pub fn spawn_prob(ty: StructureType) -> u32 {
        match ty {
            StructureType::LoneFarm => 100,
            StructureType::NonEuclideanTown => 100,
            StructureType::DragonsLair => 100,
        }
    }
}

impl Attackable for Structure {
    fn get_strength(&self) -> u32 {
        match self.r#type {
            StructureType::LoneFarm => 15,
            StructureType::NonEuclideanTown => 50,
            StructureType::DragonsLair => 200,
        }
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }

    fn get_kill_rewards(&self) -> Vec<KillRewards> {
        match self.r#type {
            StructureType::LoneFarm => {
                let mut resources = LimitedGroup::new(None);
                resources.add_single_type(common::ResourceType::Wood, 100);
                resources.add_single_type(common::ResourceType::Stone, 100);
                vec![KillRewards::Resources(resources)]
            }
            StructureType::NonEuclideanTown => {
                let mut units = LimitedGroup::new(None);
                units.add_single_type(common::UnitType::Wizard, 3);
                vec![KillRewards::Units(units)]
            }
            StructureType::DragonsLair => {
                let mut units = LimitedGroup::new(None);
                units.add_single_type(common::UnitType::Dragon, 1);
                vec![KillRewards::Units(units)]
            }
        }
    }
}
