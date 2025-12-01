use std::env;

#[allow(dead_code)]
mod grab;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grab::input_args(&args);

    println!("target string is '{}'", config.query);
    println!("at path '{}'", config.path);

    let content = grab::file_data(&config.path);
    println!("with the content:\n{}", content);

}