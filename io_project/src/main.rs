use std::{process,env};

#[allow(dead_code)]
mod grab;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grab::Input::build(&args).unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });
    let content = grab::file_data(&config.path);
    
    println!("target string is '{}'", config.query);
    println!("at path '{}'", config.path);
    println!("with the content:\n{}", content);
}