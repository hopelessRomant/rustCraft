use std::collections::{HashMap};

pub fn maps() {
    let mut list: HashMap<String,i32> = HashMap::new();
    list.insert("Knives".to_string(), 25);
    list.insert("grenade".to_string(), 45);
    list.insert("MP-5".to_string(), 25);
}