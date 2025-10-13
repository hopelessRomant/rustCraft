use std::io;
use rand::Rng;

fn main(){
    println!("Welcome thee player!");

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("secret number is : {secret}");
    println!("input your guess :");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) // & is used for declairing reference, mut makes it mutable
        .expect("Failed to read input");
    // 2 possible variants for the enum 'Result' returned by the io::stdin()

    println!("your guessed number : {guess}");
}