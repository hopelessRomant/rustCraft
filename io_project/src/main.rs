use std::env;

#[allow(dead_code)]
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse::arg_input(&args);

    println!("target string is '{}'", config.query);
    println!("at path '{}'", config.path);

    let content = parse::parse(&config.path);
    println!("with the content:\n{}", content);

}