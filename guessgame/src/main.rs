use std::io;
use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum with 3 states: Less, Greater, Equal.

fn main(){
    println!("Welcome thee player!");

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret number is : {secret}");

    println!("input your guess :");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) // '&' is used for declairing reference, 'mut' makes it mutable
        .expect("Failed to read input");
    // 2 possible variants (enum states) for the enum 'Result' returned by the io::stdin()
    // 'guess', must be an integer to be compared to the 'secret' number which is a 'i32'

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("your guessed number : {guess}");
    // using 'match' to write the compairing logic
    match guess.cmp(&secret){
        Ordering::Greater => println!("you guessed a greater number"),
        Ordering::Equal => println!("years of objective practicing finally taught you something!!! 🥳"),
        Ordering::Less => println!("you guessed a lesser numeber"),
    }
    
    
}   