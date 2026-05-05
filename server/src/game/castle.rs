use std::collections::HashMap;

use common::{
    GameCoord, Resources,
    courtyard::{Facility, FacilityType},
    game_objs::{CastleE, OwnedCastleE},
    units::{UnitGroup, UnitType},
};

use crate::game::courtyard::{Courtyard, CourtyardEvent};

pub struct Castle {
    name: String,
    pos: GameCoord,
    is_alive: bool,
    units: UnitGroup,
    resources: Resources,
    courtyard: Courtyard,
}

impl Castle {
    pub fn new(name: String, pos: GameCoord) -> Self {
        let mut units = UnitGroup::new();
        let mut resources = Resources::new(10, 10);

        if name == "gabbiano" {
            resources.saturating_add(&Resources::new(100, 100));
            units.add_single_type(UnitType::Knight, 100);
        } else if name == "pellicano" {
            resources.saturating_add(&Resources::new(1000, 1000));
            units.add_single_type(UnitType::Knight, 1000);
            units.add_single_type(UnitType::Mage, 1000);
            units.add_single_type(UnitType::Dragon, 1000);
        } else {
            units.add_single_type(UnitType::Knight, 5);
        }

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
        if !self.resources.contains(&cost) {
            return false;
        }
        if !self.courtyard.new_facility(r#type, pos) {
            return false;
        }
        self.resources.saturating_sub(&cost);
        true
    }

    pub fn add_units(&mut self, units: &UnitGroup) {
        self.units.saturating_add(units);
    }

    pub fn subtract_units_if_enough(&mut self, units: &UnitGroup) -> bool {
        self.units.subtract_if_enough(units)
    }

    pub fn get_strength(&self) -> u32 {
        self.units.get_strength()
    }

    pub fn get_pos(&self) -> GameCoord {
        self.pos
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn update(&mut self) {
        for event in self.courtyard.update().iter() {
            match event {
                CourtyardEvent::ResourceProduction(resources) => {
                    self.resources.saturating_add(resources);
                }
                CourtyardEvent::UnitsProduction(units) => {
                    self.units.saturating_add(units);
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

    pub fn export_courtyard(&self) -> HashMap<u8, Facility> {
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
