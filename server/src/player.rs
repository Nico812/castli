pub struct Player {
    name: String,
    pos: Option<(usize, usize)>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let name = name.into();
        println!("New player joined with the name: {}", name);
        Self { name, pos: None }
    }
}
