use common::{GameCoord, exports::units::UnitType};

use crate::game::castle::Castle;

pub struct Facilities {
    farm_plots: Option<Vec<FarmPlot>>,
    sawmill: Option<Sawmill>,
    mines: Option<Mines>,
    barracks: Option<Barracks>,
    shipyard: Option<Shipyard>,
}

impl Facilities {
    pub fn new() -> Self {
        let farm_plots = Some(vec![FarmPlot {
            pos: GameCoord { x: 10, y: 10 },
        }]);
        Self {
            farm_plots,
            sawmill: None,
            mines: None,
            barracks: None,
            shipyard: None,
        }
    }

    pub fn update(&self, castle: &mut Castle) {
        if let Some(ref farm_plots) = self.farm_plots {
            for _ in farm_plots.iter() {
                FarmPlot::update(castle);
            }
        }
        if let Some(_) = &self.sawmill {
            Sawmill::update(castle);
        }
        if let Some(_) = &self.mines {
            Mines::update(castle);
        }
        if let Some(_) = &self.barracks {
            Barracks::update(castle);
        }
        if let Some(_) = &self.shipyard {
            Shipyard::update(castle);
        }
    }
}

pub struct FarmPlot {
    pos: GameCoord,
}

impl FarmPlot {
    pub fn new(pos: GameCoord) -> Self {
        Self { pos }
    }

    pub fn update(castle: &mut Castle) {
        castle.peasants += 1;
    }
}

pub struct Sawmill {
    pos: GameCoord,
}

impl Sawmill {
    pub fn new(pos: GameCoord) -> Self {
        Self { pos }
    }

    pub fn update(castle: &mut Castle) {
        castle.resources.wood += 10;
    }
}

pub struct Mines {
    pos: GameCoord,
}

impl Mines {
    pub fn new(pos: GameCoord) -> Self {
        Self { pos }
    }

    pub fn update(castle: &mut Castle) {
        castle.resources.stone += 10;
    }
}

pub struct Barracks {
    pos: GameCoord,
}

impl Barracks {
    pub fn new(pos: GameCoord) -> Self {
        Self { pos }
    }

    pub fn update(castle: &mut Castle) {
        let knights_to_add = 1;
        if castle.peasants > knights_to_add {
            castle.peasants -= knights_to_add;
            castle
                .units
                .add_single_type(UnitType::Knight, knights_to_add);
        }
    }
}

pub struct Shipyard {
    pos: GameCoord,
}

impl Shipyard {
    pub fn new(pos: GameCoord) -> Self {
        Self { pos }
    }

    pub fn update(castle: &mut Castle) {}
}
