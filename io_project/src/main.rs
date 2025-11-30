use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1];
    let path = &args[2];

    println!("target string is {target}");
    println!("at path {path}");
}