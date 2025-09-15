use common::{StructureE, StructureTypeE};

pub struct Structure {
    name: String,
    r#type: StructureTypeE,
    pos: (usize, usize),
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
