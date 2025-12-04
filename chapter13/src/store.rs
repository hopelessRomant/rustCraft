use std::{collections::HashMap};

pub trait Event {
    fn give_away(&self, choice: Option<Tshirts>) -> Tshirts;
    fn most_stocked(&self) -> Tshirts;
}

pub type Inventory = HashMap<Tshirts, i32>;

#[derive(Debug, Clone, Copy)]
pub enum Tshirts {
    Red,
    Blue,
    Green,
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
