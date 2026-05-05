use std::collections::HashMap;

use common::{
    GameCoord, Resources,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::{Facility, FacilityType},
    units::{UnitGroup, UnitType},
};

pub enum CourtyardEvent {
    ResourceProduction(Resources),
    UnitsProduction(UnitGroup),
}

pub struct Courtyard {
    peasants: u32,
    facilities: HashMap<u8, Facility>,
    occupied: [[Option<u8>; COURTYARD_COLS]; COURTYARD_ROWS],
    owned_cnt: [u8; FacilityType::COUNT],
    id_cnt: u8,
}

impl Courtyard {
    pub fn new() -> Self {
        Self {
            peasants: 10,
            facilities: HashMap::new(),
            occupied: [[None; COURTYARD_COLS]; COURTYARD_ROWS],
            owned_cnt: [0; FacilityType::COUNT],
            id_cnt: 0,
        }
    }

    pub fn update(&mut self) -> Vec<CourtyardEvent> {
        let mut events = Vec::new();
        let mut resource_prod = Resources::new(0, 0);
        let mut units_prod = UnitGroup::new();
        let mut peasants_prod = 0;

        for facility in self.facilities.values() {
            match facility.r#type {
                FacilityType::FarmPlot => peasants_prod += facility.lv,
                FacilityType::Sawmill => resource_prod.wood += facility.lv * 10,
                FacilityType::Mines => resource_prod.stone += facility.lv * 10,
                FacilityType::Barracks => {
                    let knights = facility.lv * 5;
                    if self.peasants >= knights {
                        self.peasants -= knights;
                        units_prod.add_single_type(UnitType::Knight, knights);
                    }
                }
                FacilityType::Shipyard => {
                    units_prod.add_single_type(UnitType::Ship, facility.lv);
                }
            }
        }

        if resource_prod.wood > 0 || resource_prod.stone > 0 {
            events.push(CourtyardEvent::ResourceProduction(resource_prod));
        }

        if !units_prod.is_empty() {
            events.push(CourtyardEvent::UnitsProduction(units_prod));
        }

        if peasants_prod > 0 {
            self.peasants = self.peasants.saturating_add(peasants_prod);
        }

        events
    }

    pub fn new_facility(&mut self, r#type: FacilityType, pos: GameCoord) -> bool {
        let type_idx = r#type.as_index();
        let max_count = r#type.max_count();

        if self.owned_cnt[type_idx] >= max_count {
            return false;
        }
        if !self.is_position_valid(pos, r#type.size()) {
            return false;
        }

        let id = self.new_id();
        let facility = Facility::new(r#type, pos);
        self.facilities.insert(id, facility);
        self.mark_occupied(id, pos, r#type.size());

        true
    }

    fn new_id(&mut self) -> u8 {
        self.id_cnt += 1;
        self.id_cnt
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

    fn mark_occupied(&mut self, id: u8, pos: GameCoord, size: GameCoord) {
        for x in pos.x..pos.x + size.x {
            for y in pos.y..pos.y + size.y {
                self.occupied[y as usize][x as usize] = Some(id);
            }
        }
    }

    pub fn export(&self) -> HashMap<u8, Facility> {
        self.facilities.clone()
    }
}
