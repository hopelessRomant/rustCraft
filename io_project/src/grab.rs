use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Input {
    pub query: String,
    pub path: String,
    pub case: bool,
}

impl Input {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Input, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = !env::var("IGNORE_CASE").is_ok();

        Ok(Input { query, path: file_path, case: ignore_case })
    }
}

pub fn file_data (path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path);
    Ok(contents?)
}
