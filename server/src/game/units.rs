use std::collections::VecDeque;

use common::{
    GameCoord, GameId, all_units, all_units_enum,
    exports::{
        game_object::DeployedUnitsE,
        units::{UnitGroupE, UnitType},
    },
};

#[derive(Clone)]
pub struct UnitGroup {
    quantities: [u32; UnitType::COUNT],
    present_mask: u8,
}

impl UnitGroup {
    pub fn new() -> Self {
        let quantities = [0; UnitType::COUNT];
        let present_mask = 0;

        Self {
            quantities,
            present_mask,
        }
    }

    //TODO: totally hange this when coding the dynamic battles.
    pub fn get_strength(&self) -> u32 {
        let mut str = 0;
        for (i, unit) in all_units_enum!().iter() {
            str += self.quantities[*i] * unit.get_strength();
        }
        str
    }

    pub fn add_single_type(&mut self, unit: UnitType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_add(count);

        if self.quantities[idx] > 0 {
            self.present_mask |= unit.as_mask();
        }
    }

    pub fn subtract_single_type(&mut self, unit: UnitType, count: u32) {
        let idx = unit.as_index();
        self.quantities[idx] = self.quantities[idx].saturating_sub(count);

        if self.quantities[idx] == 0 {
            self.present_mask &= !unit.as_mask();
        }
    }

    pub fn subtract_if_enough(&mut self, other: &Self) -> bool {
        if !other.is_subset(self) {
            return false;
        }
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(UnitType::form_index(i), *quantity);
        }
        true
    }

    pub fn subtract_unchecked(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.subtract_single_type(UnitType::form_index(i), *quantity);
        }
    }

    pub fn saturating_add(&mut self, other: &Self) {
        for (i, quantity) in other.quantities.iter().enumerate() {
            self.add_single_type(UnitType::form_index(i), *quantity);
        }
    }

    pub fn contains(&self, unit: UnitType) -> bool {
        self.present_mask & unit.as_mask() != 0
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for (i, quantity) in other.quantities.iter().enumerate() {
            if self.quantities[i] > *quantity {
                return false;
            }
        }
        true
    }

    pub fn iter_present(&self) -> impl Iterator<Item = (UnitType, u32)> + '_ {
        common::all_units!()
            .into_iter()
            .filter(move |u| self.contains(*u))
            .map(move |u: UnitType| (u, self.quantities[u.as_index()]))
    }

    pub fn export(&self) -> UnitGroupE {
        UnitGroupE {
            quantities: self.quantities,
        }
    }

    pub fn from_export(export: UnitGroupE) -> Self {
        let quantities = export.quantities;
        let mut unit_group = Self::new();
        for (i, quantity) in quantities.iter().enumerate() {
            unit_group.add_single_type(UnitType::form_index(i), *quantity)
        }
        unit_group
    }

    pub fn is_empty(&self) -> bool {
        for quantity in self.quantities.iter() {
            if *quantity != 0 {
                return false;
            }
        }
        true
    }
}

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
