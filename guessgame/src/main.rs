use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome thee player!");

    let secret = rand::thread_rng().gen_range(1..=100);
//    println!("secret number is : {secret}");

    loop {
        println!("input your guess :");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };
        println!("your guessed number : {guess}");

        match guess.cmp(&secret){
            Ordering::Greater => println!("you guessed a greater number"),
            Ordering::Less => println!("you guessed a lesser numeber"),
            Ordering::Equal => {
                println!("years of objective practicing finally taught you something!!! 🥳");
                break;
            }
        }
}   
}   