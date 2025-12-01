use std::fs;

struct Input {
    query: String,
    path: String,
}

pub fn arg_input(args: &[String]) -> (&str, &str){
    let query = &args[1];
    let path = &args[2];

    (query, path)
}

pub fn parse (path: &str) -> String {
    fs::read_to_string(path).expect("shit happened")
} 