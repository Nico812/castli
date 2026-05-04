use crate::game::castle::Castle;
use common::{
    GameCoord, Resources,
    r#const::{COURTYARD_COLS, COURTYARD_ROWS},
    courtyard::{Facility, FacilityType},
    units::UnitType,
};

pub struct Courtyard {
    occupied: [[bool; COURTYARD_COLS]; COURTYARD_ROWS],
    facilities: [Vec<Facility>; FacilityType::COUNT],
}

impl Courtyard {
    pub fn new() -> Self {
        let mut facilities = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let start_pos = GameCoord { x: 5, y: 5 };
        facilities[FacilityType::FarmPlot.index()].push(Facility::new(
            FacilityType::FarmPlot,
            1,
            start_pos,
        ));

        let mut occupied = [[false; COURTYARD_COLS]; COURTYARD_ROWS];
        Self::mark_occupied(&mut occupied, start_pos, FacilityType::FarmPlot.size());

        Self {
            occupied,
            facilities,
        }
    }

    pub fn add(&mut self, resources: &mut Resources, facility: Facility) -> bool {
        let type_idx = facility.r#type.index();
        let max_count = facility.r#type.max_count();

        if self.facilities[type_idx].len() >= max_count {
            return false;
        }

        if !self.is_position_valid(facility.pos(), facility.size()) {
            return false;
        }

        let cost = facility.cost();
        if !resources.subtract_if_enough(&cost) {
            return false;
        }

        Self::mark_occupied(&mut self.occupied, facility.pos(), facility.size());
        self.facilities[type_idx].push(facility);
        true
    }

    pub fn export(&self) -> [Vec<Facility>; FacilityType::COUNT] {
        self.facilities.clone()
    }

    fn is_position_valid(&self, pos: GameCoord, size: GameCoord) -> bool {
        if pos.x + size.x > COURTYARD_COLS || pos.y + size.y > COURTYARD_ROWS {
            return false;
        }

        for x in pos.x..pos.x + size.x {
            for y in pos.y..pos.y + size.y {
                if self.occupied[y as usize][x as usize] {
                    return false;
                }
            }
        }
        true
    }

    fn mark_occupied(
        occupied: &mut [[bool; COURTYARD_COLS]; COURTYARD_ROWS],
        pos: GameCoord,
        size: GameCoord,
    ) {
        for x in pos.x..pos.x + size.x {
            for y in pos.y..pos.y + size.y {
                occupied[y as usize][x as usize] = true;
            }
        }
    }

    pub fn update(&self, castle: &mut Castle) {
        for facility in self.facilities[FacilityType::FarmPlot.index()].iter() {
            castle.peasants = castle.peasants.saturating_add(1 * facility.lv);
        }

        for facility in self.facilities[FacilityType::Sawmill.index()].iter() {
            castle.resources.wood = castle.resources.wood.saturating_add(facility.lv * 10);
        }

        for facility in self.facilities[FacilityType::Mines.index()].iter() {
            castle.resources.stone = castle.resources.stone.saturating_add(facility.lv * 10);
        }

        for facility in self.facilities[FacilityType::Barracks.index()].iter() {
            let knights_to_add = facility.lv * 5;
            if castle.peasants > knights_to_add {
                castle.peasants -= knights_to_add;
                castle
                    .units
                    .add_single_type(UnitType::Knight, knights_to_add);
            }
        }

        for facility in self.facilities[FacilityType::Shipyard.index()].iter() {
            castle.units.add_single_type(UnitType::Ship, facility.lv);
        }
    }
}
