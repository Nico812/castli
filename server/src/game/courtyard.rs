use std::collections::HashMap;

use crate::game::castle::Castle;
use common::{
    GameCoord, GameId, Resources,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::{Facility, FacilityType},
    units::UnitType,
};

pub struct Courtyard {
    facilities: HashMap<GameId, Facility>,
    occupied: [[Option<GameId>; COURTYARD_COLS]; COURTYARD_ROWS],
    owned_cnt: [u8; FacilityType::COUNT],
}

impl Courtyard {
    pub fn new() -> Self {
        let facilities = HashMap::new();
        let occupied = [[None; COURTYARD_COLS]; COURTYARD_ROWS];
        let owned_cnt = [0; FacilityType::COUNT];

        Self {
            occupied,
            facilities,
            owned_cnt,
        }
    }

    pub fn add(
        &mut self,
        resources: &mut Resources,
        facility: Facility,
        facility_id: GameId,
    ) -> bool {
        let type_idx = facility.r#type.as_index();
        let max_count = facility.r#type.max_count();

        if self.owned_cnt[type_idx] >= max_count {
            return false;
        }

        if !self.is_position_valid(facility.pos(), facility.size()) {
            return false;
        }

        let cost = facility.cost();
        if !resources.subtract_if_enough(&cost) {
            return false;
        }

        Self::mark_occupied(
            &mut self.occupied,
            facility_id,
            facility.pos(),
            facility.size(),
        );

        self.facilities.insert(facility_id, facility);
        true
    }

    pub fn export(&self) -> HashMap<GameId, Facility> {
        self.facilities.clone()
    }

    fn is_position_valid(&self, pos: GameCoord, size: GameCoord) -> bool {
        if pos.x + size.x > COURTYARD_COLS || pos.y + size.y > COURTYARD_ROWS {
            return false;
        }

        for x in pos.x..pos.x + size.x {
            for y in pos.y..pos.y + size.y {
                if self.occupied[y as usize][x as usize].is_some() {
                    return false;
                }
            }
        }
        true
    }

    fn mark_occupied(
        occupied: &mut [[Option<GameId>; COURTYARD_COLS]; COURTYARD_ROWS],
        facility_id: GameId,
        pos: GameCoord,
        size: GameCoord,
    ) {
        for x in pos.x..pos.x + size.x {
            for y in pos.y..pos.y + size.y {
                occupied[y as usize][x as usize] = Some(facility_id);
            }
        }
    }

    pub fn update(&self, castle: &mut Castle) {
        for facility in self.facilities.values() {
            match facility.r#type {
                FacilityType::FarmPlot => {
                    castle.peasants = castle.peasants.saturating_add(1 * facility.lv);
                }
                FacilityType::Sawmill => {
                    castle.resources.wood = castle.resources.wood.saturating_add(facility.lv * 10);
                }
                FacilityType::Mines => {
                    castle.resources.stone =
                        castle.resources.stone.saturating_add(facility.lv * 10);
                }
                FacilityType::Barracks => {
                    let knights_to_add = facility.lv * 5;
                    if castle.peasants > knights_to_add {
                        castle.peasants -= knights_to_add;
                        castle
                            .units
                            .add_single_type(UnitType::Knight, knights_to_add);
                    }
                }
                FacilityType::Shipyard => {
                    castle.units.add_single_type(UnitType::Ship, facility.lv);
                }
            }
        }
    }
}
