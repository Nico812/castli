use std::collections::HashMap;

use common::{
    GameCoord, GroupElement, InhabitantType, LimitedGroup, ResourceType, UnitType,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::{Facility, FacilityType, Production},
    hitboxes::Hitbox,
};

pub enum CourtyardEvent {
    ResourceProduction(LimitedGroup<ResourceType>),
    UnitsProduction(LimitedGroup<UnitType>),
    ResourceLimExtension(Vec<u32>),
    UnitsLimExtension(Vec<u32>),
}

pub struct Courtyard {
    facilities: HashMap<usize, Facility>,
    owned_cnt: LimitedGroup<FacilityType>,
    inhabitants: LimitedGroup<InhabitantType>,
    occupied: [[Option<usize>; COURTYARD_COLS]; COURTYARD_ROWS],
    id_cnt: usize,
}

impl Courtyard {
    pub fn new() -> Self {
        let facilities = HashMap::new();
        let occupied = [[None; COURTYARD_COLS]; COURTYARD_ROWS];
        let owned_cnt = LimitedGroup::new(Some(FacilityType::max_count()));
        let inhabitants = LimitedGroup::<InhabitantType>::castle_starting_group();

        let mut self_ = Self {
            facilities,
            owned_cnt,
            inhabitants,
            occupied,
            id_cnt: 0,
        };

        self_.new_facility(FacilityType::Wall, GameCoord { x: 6, y: 6 });
        self_
    }

    pub fn update(&mut self) -> Vec<CourtyardEvent> {
        let mut events = Vec::new();
        let mut resource_prod = LimitedGroup::new(None);
        let mut units_prod = LimitedGroup::new(None);
        let mut inhabitants_prod = LimitedGroup::new(None);

        for facility in self.facilities.values() {
            for production in facility.production() {
                match production {
                    Production::Resources(ty, amt) => resource_prod.add_single_type(ty, amt),
                    Production::Units(ty, amt) => {
                        let owned_peasants =
                            &mut self.inhabitants.quant[InhabitantType::Peasant.as_index()];
                        if *owned_peasants >= amt {
                            *owned_peasants -= amt;
                            units_prod.add_single_type(ty, amt);
                        }
                    }
                    Production::Inhabitants(ty, amt) => inhabitants_prod.add_single_type(ty, amt),
                }
            }
        }

        if !resource_prod.is_empty() {
            events.push(CourtyardEvent::ResourceProduction(resource_prod));
        }

        if !units_prod.is_empty() {
            events.push(CourtyardEvent::UnitsProduction(units_prod));
        }

        if !inhabitants_prod.is_empty() {
            self.inhabitants.add(&inhabitants_prod);
        }

        events
    }

    pub fn new_facility(&mut self, ty: FacilityType, pos: GameCoord) -> bool {
        if self.owned_cnt.is_full_by_type(ty) {
            return false;
        }
        if !self.is_position_valid(pos, &ty.hitbox()) {
            return false;
        }

        let id = self.new_id();
        let facility = Facility::new(ty, pos);

        self.facilities.insert(id, facility);
        self.owned_cnt.add_single_type(ty, 1);
        self.mark_occupied(id, pos, &ty.hitbox());
        true
    }

    pub fn upgrade_facility(&mut self, id: usize) -> (bool, Vec<CourtyardEvent>) {
        let mut events = Vec::new();
        let Some(facility) = self.facilities.get_mut(&id) else {
            return (false, events);
        };
        if facility.lv >= facility.r#type.max_lv() {
            return (false, events);
        }

        facility.lv += 1;

        match facility.r#type {
            FacilityType::Treasury => events.push(CourtyardEvent::ResourceLimExtension(
                Facility::treasury_lim_extension(facility.lv),
            )),
            FacilityType::Residence => {
                events.push(CourtyardEvent::UnitsLimExtension(
                    Facility::residence_units_limit_extension(facility.lv),
                ));
                self.inhabitants
                    .add_to_lim(&Facility::residence_inhabitants_lim_extension(facility.lv));
            }
            _ => {}
        }

        (true, events)
    }

    pub fn get_facility(&self, facility_id: usize) -> Option<&Facility> {
        self.facilities.get(&facility_id)
    }

    pub fn strength_multiplyer(&self) -> u32 {
        for (_, facility) in self.facilities.iter() {
            if facility.r#type == FacilityType::Wall {
                return facility.lv as u32;
            }
        }
        1
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
