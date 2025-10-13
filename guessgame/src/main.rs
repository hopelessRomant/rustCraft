use std::io;

fn main(){
    println!("input your guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) // & is used for declairing reference, mut makes it mutable
        .expect("Failed to read input");
    
}