use std::io;

fn main(){
    println!("input your guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) // & is used for declairing reference, mut makes it mutable
        .expect("Failed to read input");
    // 2 possible variants for the enum 'Result' returned by the io::stdin()

    println!("guessed number : {guess}");
}