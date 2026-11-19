use std::collections::HashMap;

use common::{
    GameCoord, GroupElement, LimitedGroup, ResourceType, UnitType,
    courtyard::{Facility, FacilityType},
    game_objs::{CastleE, OwnedCastleE},
};

use crate::game::{
    courtyard::{Courtyard, CourtyardEvent},
    game::{Attackable, KillRewards},
};

pub struct Castle {
    name: String,
    pos: GameCoord,
    is_alive: bool,
    units: LimitedGroup<UnitType>,
    resources: LimitedGroup<ResourceType>,
    courtyard: Courtyard,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let units = LimitedGroup::new(Some(vec![0; UnitType::amt()]));
        let resources = LimitedGroup::<ResourceType>::castle_starting_group();

        Self {
            name,
            pos,
            is_alive: true,
            units,
            resources,
            courtyard: Courtyard::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn new_facility(&mut self, r#type: FacilityType, pos: GameCoord) -> bool {
        let cost = r#type.base_cost();
        if !cost.is_subset(&self.resources) {
            return false;
        };
        if !self.courtyard.new_facility(r#type, pos) {
            return false;
        }
        self.resources.subtract(&cost);
        true
    }

    pub fn upgrade_facility(&mut self, facility_id: usize) -> bool {
        let Some(cost) = self
            .courtyard
            .get_facility(facility_id)
            .map(|facility| facility.upgrade_cost())
        else {
            return false;
        };
        if !cost.is_subset(&self.resources) {
            return false;
        };

        let (success, events) = self.courtyard.upgrade_facility(facility_id);
        if !success {
            return false;
        }

        self.resources.subtract(&cost);
        self.listen_courtyard_events(events);
        true
    }

    pub fn add_units(&mut self, units: &LimitedGroup<UnitType>) {
        self.units.subtract(units);
    }

    pub fn add_resources(&mut self, resources: &LimitedGroup<ResourceType>) {
        self.resources.add(resources);
    }

    pub fn subtract_units_if_enough(&mut self, units: &LimitedGroup<UnitType>) -> bool {
        if self.units.is_uberset(units) {
            self.units.subtract(units);
            true
        } else {
            false
        }
    }

    pub fn get_pos(&self) -> GameCoord {
        self.pos
    }

    pub fn get_units(&self) -> &LimitedGroup<UnitType> {
        &self.units
    }

    pub fn get_units_mut(&mut self) -> &mut LimitedGroup<UnitType> {
        &mut self.units
    }

    pub fn update(&mut self) {
        let events = self.courtyard.update();
        self.listen_courtyard_events(events);
    }

    pub fn listen_courtyard_events(&mut self, events: Vec<CourtyardEvent>) {
        for event in events {
            match event {
                CourtyardEvent::ResourceProduction(resources) => {
                    self.resources.add(&resources);
                }
                CourtyardEvent::UnitsProduction(units) => {
                    self.units.add(&units);
                }
                CourtyardEvent::ResourceLimExtension(lim_ext) => {
                    self.resources.add_to_lim(&lim_ext);
                }
                CourtyardEvent::UnitsLimExtension(lim_ext) => {
                    self.units.add_to_lim(&lim_ext);
                }
            }
        }
    }

    pub fn export(&self) -> CastleE {
        CastleE {
            name: self.name.clone(),
            pos: self.pos,
            alive: self.is_alive,
        }
    }

    pub fn export_courtyard(&self) -> HashMap<usize, Facility> {
        self.courtyard.export()
    }

    pub fn export_owned(&self) -> OwnedCastleE {
        OwnedCastleE {
            alive: self.is_alive,
            name: self.name.clone(),
            pos: self.pos,
            units: self.units.clone(),
            resources: self.resources.clone(),
        }
    }
}

impl Attackable for Castle {
    fn get_strength(&self) -> u32 {
        self.units.get_strength() * self.courtyard.strength_multiplyer()
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }

    fn get_kill_rewards(&self) -> Vec<KillRewards> {
        vec![]
    }
}
