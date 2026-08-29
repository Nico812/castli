use std::{collections::HashMap, iter::Enumerate};

use common::{
    GameCoord, Resources,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::{Facility, FacilityType},
    hitboxes::Hitbox,
    units::{UnitGroup, UnitType},
};

pub enum CourtyardEvent {
    ResourceProduction(Resources),
    UnitsProduction(UnitGroup),
}

pub struct Courtyard {
    peasants: u32,
    facilities: HashMap<usize, Facility>,
    occupied: [[Option<usize>; COURTYARD_COLS]; COURTYARD_ROWS],
    owned_cnt: [u8; FacilityType::COUNT],
    id_cnt: usize,
}

impl Courtyard {
    pub fn new() -> Self {
        let facilities = HashMap::new();
        let occupied = [[None; COURTYARD_COLS]; COURTYARD_ROWS];
        let owned_cnt = [0; FacilityType::COUNT];

        let mut self_ = Self {
            peasants: 10,
            facilities,
            occupied,
            owned_cnt,
            id_cnt: 0,
        };

        self_.new_facility(FacilityType::Wall, GameCoord { x: 6, y: 6 });

        self_
    }

    pub fn update(&mut self) -> Vec<CourtyardEvent> {
        let mut events = Vec::new();
        let mut resource_prod = Resources::new(0, 0);
        let mut units_prod = UnitGroup::new();
        let mut peasants_prod = 0;

        for facility in self.facilities.values() {
            match facility.r#type {
                FacilityType::Wall => {}
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
        if !self.is_position_valid(pos, &r#type.get_hitbox()) {
            return false;
        }

        let id = self.new_id();
        let facility = Facility::new(r#type, pos);
        self.facilities.insert(id, facility);
        self.owned_cnt[type_idx] += 1;
        self.mark_occupied(id, pos, &r#type.get_hitbox());

        true
    }

    fn new_id(&mut self) -> usize {
        self.id_cnt += 1;
        self.id_cnt
    }

    fn is_position_valid(&self, pos: GameCoord, hitbox: &Hitbox) -> bool {
        let size = GameCoord::new(hitbox.len(), hitbox[0].len());

        if pos.x + size.x > COURTYARD_COLS || pos.y + size.y > COURTYARD_ROWS {
            return false;
        }

        for (i, row) in hitbox.iter().enumerate() {
            for (j, hit) in row.iter().enumerate() {
                if *hit && self.occupied[pos.y + i][pos.x + j].is_some() {
                    return false;
                }
            }
        }

        true
    }

    fn mark_occupied(&mut self, id: usize, pos: GameCoord, hitbox: &Hitbox) {
        for (i, row) in hitbox.iter().enumerate() {
            for (j, hit) in row.iter().enumerate() {
                if *hit {
                    self.occupied[pos.y + i][pos.x + j] = Some(id);
                }
            }
        }
    }

    pub fn export(&self) -> HashMap<usize, Facility> {
        self.facilities.clone()
    }
}
