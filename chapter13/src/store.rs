use std::collections::HashMap;

pub enum Tshirts {
    Red,
    Blue,
    Green,
}

pub struct Inventory {
    stock: HashMap<String, i32>
}

impl Inventory {
    pub fn give_away(self, _choice: Option<Tshirts>) {
        // choice.unwrap_or_else(|| self.default())
    }

    fn default() {}
}
