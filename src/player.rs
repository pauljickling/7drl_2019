#[derive(Debug)]
pub struct Player {
    pub hp: u8,
    pub movement: u8,
    pub powers: Vec<String>,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hp: 4,
            movement: 1,
            powers: Vec::new(),
            inventory: Vec::new(),
        }
    }
}
