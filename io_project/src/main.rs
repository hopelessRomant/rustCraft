use std::env;

#[allow(dead_code)]
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _config = parse::arg_input(&args);

    // println!("target string is '{query}'");
    // println!("at path '{path}'");

    // let content = parse::parse(path);
    // println!("with the content:\n{}", content);

}