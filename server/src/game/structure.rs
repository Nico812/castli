use common::{
    GameCoord,
    game_objs::{StructureE, StructureType},
};

pub struct Structure {
    name: String,
    r#type: StructureType,
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
