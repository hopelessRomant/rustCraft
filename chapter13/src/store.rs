pub enum Tshirts {
    Red,
    Blue,
    Green,
}

pub struct Inventory {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl Inventory {
    pub fn give_away(self, _choice: Option<Tshirts>) {
        // choice.unwrap_or_else(|| self.default())
    }

    fn default() {}
}
