use std::collections::HashSet;

#[derive(Debug)]
pub struct Player {
    pub hp: u8,
    pub movement: u8,
    pub powers: Vec<String>,
    pub inventory: HashSet<char>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hp: 4,
            movement: 1,
            powers: Vec::new(),
            inventory: HashSet::new(),
        }
    }

    pub fn get_item(mut self, item: char) {
        let mut items = HashSet::new();
        items.insert('!');
        items.insert('$');
        items.insert('/');
        items.insert('?');
        items.insert('[');
        items.insert(']');
        items.insert('%');
        items.insert('*');
        items.insert('+');
        items.insert('=');
        items.insert('(');
        items.insert(')');
        if items.contains(&item) {
            self.inventory.insert(item);
        }
    }
}
