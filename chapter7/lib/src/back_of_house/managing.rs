#[derive(Debug)]
pub struct Breakfast {
    pub drink: String,
    special: String,
}

impl Breakfast{
    pub fn monday (drink: &str) -> Breakfast {
        Breakfast { drink: String::from(drink), special: String::from("red-velvet sunday") }
    }
}

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}