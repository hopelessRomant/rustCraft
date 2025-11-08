use std::{collections::HashMap};
use std::io;

pub fn intro() {
    let mut list: HashMap<String,i32> = HashMap::new();
    list.insert("Knives".to_string(), 25);
    list.insert("grenade".to_string(), 45);
    list.insert("MP-5".to_string(), 25);
    list.insert("AK-47".to_string(), 50);

    // let val = list.get(&"AK-47".to_string()).copied().unwrap_or(45);
    // println!("{:#?}",val);

    for (k,v) in &list {
        println!("{}: {}",k,v);
    }

    println!("{list:#?}")
}

pub fn update() {
    let mut price: HashMap<String, i32> = HashMap::new();
    price.insert("Striker".to_string(), 599);
    let _check = price.entry("twisted love".to_string()).or_insert(499);
    println!("{price:#?}");
}

pub fn track () {
    let mut statement = String::new();
    let mut record: HashMap<String,i32> = HashMap::new();

    println!("Please type you messege below:");
    io::stdin()
        .read_line(&mut statement)
        .expect("failed to read line");

    let clean:String = statement.chars().filter(|c| !c.is_whitespace()).collect();
    // println!("{clean}");

    for w in clean.chars() {
        let count = record.entry(w.to_string()).or_insert(0);
        *count +=1;
    }

    println!("{record:#?}");
}
