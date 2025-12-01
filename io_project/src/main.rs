use std::env;

#[allow(dead_code)]
mod grab;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = grab::Input::new(&args)?;
    let content = grab::file_data(&config.path);
    
    println!("target string is '{}'", config.query);
    println!("at path '{}'", config.path);
    println!("with the content:\n{}", content);
    Ok(())
}