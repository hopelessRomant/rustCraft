use std::collections::{HashMap};

pub fn maps() {
    let mut list: HashMap<String,i32> = HashMap::new();
    list.insert(String::from("Knife"), 25);
    list.insert(String::from("granade"), 45);
    list.insert(String::from("MP-5"), 25);
}