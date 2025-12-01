use std::fs;

#[derive(Debug)]
pub struct Input {
    query: String,
    path: String,
}

pub fn arg_input(args: &[String]) -> Input{
    let query = &args[1];
    let path = &args[2];

    Input{query: query.to_string(), path: path.to_string()}
}

pub fn parse (path: &str) -> String {
    fs::read_to_string(path).expect("shit happened")
} 