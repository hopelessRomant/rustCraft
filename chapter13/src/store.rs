use std::{collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tshirts {
    Red,
    Green,
    Blue,
}

pub type Inventory = HashMap<Tshirts, i32>;
pub trait Event {
    fn give_away(&self, choice: Option<Tshirts>) -> Tshirts;
    fn most_stocked(&self) -> Tshirts;
    fn build(red: i32, green: i32, blue: i32) -> Inventory{
        let mut stock = Inventory::new();
        stock.insert(Tshirts::Red, red);
        stock.insert(Tshirts::Green, green);
        stock.insert(Tshirts::Blue, blue);

        stock
    }
}

impl Event for Inventory {
    fn give_away(&self, choice: Option<Tshirts>) -> Tshirts {
        choice.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> Tshirts {
        let gift = self.iter().max_by_key(|(_, v)| *v).map(|(k, _)| *k);
        gift.unwrap()
    }
}
