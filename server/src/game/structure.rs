use common::{
    GameCoord,
    exports::game_object::{StructureE, StructureTypeE},
};

pub struct Structure {
    name: String,
    r#type: StructureTypeE,
    pos: GameCoord,
}

impl Structure {
    pub fn export(&self) -> StructureE {
        StructureE {
            name: self.name.clone(),
            r#type: self.r#type,
            pos: self.pos,
        }
    }
}
