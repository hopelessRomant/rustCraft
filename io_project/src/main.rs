use std::{process,env};

#[allow(dead_code)]
mod grab;

#[allow(dead_code)]
mod ops;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = grab::Input::build(&args).unwrap_or_else(|err| {
        println!("Input Error: {err}");
        process::exit(1);
    });

    let content = grab::file_data(&config.path).unwrap_or_else(|err| {
        println!("File Error: {err}");
        process::exit(1);
    });

    let output = ops::cs_search(&config.query, &content);
    if output.len() == 0 {
        println!("No such line containing '{}' was found in {}", config.query, config.path);
    } else {
        println!("Search results for lines containing '{}' in the given file are:\n{:#?}", config.query, output);
    }
}