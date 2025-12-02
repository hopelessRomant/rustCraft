use std::{env::var, error::Error, fs};

#[derive(Debug)]
pub struct Input {
    pub query: String,
    pub path: String,
    pub case: bool,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        println!("{:#?}", args);
        if args.len() < 3 {
            return Err("Need the complete set of inputs to proceed");
        } else {
            Ok(Input { 
                query: args[1].to_string(), 
                path: args[2].to_string(), 
                case: !var("IGNORE_CASE").is_ok(),
            })
        }
    }
}

pub fn file_data (path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path);
    Ok(contents?)
}
