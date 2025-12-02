use std::{process,env};

#[allow(dead_code)]
mod grab;

#[allow(dead_code)]
mod search;

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

    let output = search::result(&config.query, &content);
    println!("Desired data lines are:\n{:#?}",output);
}