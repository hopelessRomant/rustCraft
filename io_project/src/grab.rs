use std::fs;

#[derive(Debug)]
pub struct Input {
    pub query: String,
    pub path: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("Need complete set of argument to proceed");
        }

        Ok(
            Input { query: args[1].to_string(), path: args[2].to_string() }
        )
    }
}

pub fn file_data (path: &str) -> String {
    fs::read_to_string(path).expect("file not read")
} 