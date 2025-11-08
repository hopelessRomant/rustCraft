use std::collections::HashMap;

pub fn problem1() {
    let mut names: HashMap<String,String> = HashMap::new();
    names.insert("sahil".to_string(), "Physics".to_string());
    let pointer = names.entry("sahil".to_string()).or_insert("electronics".to_string());
    *pointer = "electronics".to_string();
    println!("{:#?}",names);
}