use std::fs;

#[derive(Debug)]
pub struct Input {
    pub query: String,
    pub path: String,
}

pub fn arg_input(args: &[String]) -> Input{

    Input{
        query: args[1].to_string(),
        path: args[2].to_string(),
    }

}

pub fn parse (path: &str) -> String {
    fs::read_to_string(path).expect("shit happened")
} 