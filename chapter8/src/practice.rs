use std::collections::HashMap;
use std::io::{self};

pub fn problem1() {
    let mut names: HashMap<String,String> = HashMap::new();
    names.insert("sahil".to_string(), "Physics".to_string());
    let pointer = names.entry("sahil".to_string()).or_insert("electronics".to_string());
    *pointer = "electronics".to_string();
    println!("{:#?}",names);
}

pub fn company () {
    let mut sales_team = sales();
    let mut members: Vec<String> = Vec::new();
    let mut department: HashMap<String,Vec<String>> = HashMap::new();

    for i in sales_team.split_whitespace() {
        
    }
}

fn sales() -> String {
    let mut sales = String::new();
    println!("Enter the member names for Sales department");

    io::stdin()
    .read_line(&mut sales)
    .expect("input not read");

    return sales;
}
