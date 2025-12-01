use std::{error::Error, fs};

#[derive(Debug)]
pub struct Input {
    pub query: String,
    pub path: String,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("Need the complete set of arguments to proceed");
        } else {
            Ok(Input { 
                query: args[1].to_string(), 
                path: args[2].to_string() 
            })
        }
    }
}

pub fn file_data (path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path);
    Ok(contents?)
} 