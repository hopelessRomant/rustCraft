use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1];
    let path = &args[2];

    println!("target string is '{target}'");
    println!("at path '{path}'");

    let content = fs::read_to_string(path).expect("unable to read the file");
    println!("with the content:\n{}", content);
}