use rand::Rng;

#[derive(Debug)]
enum Face {
    Heads,
    Tails,
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Black,
}

enum Toss {
    Coin(Face),
    Dice(Color),
}

fn result (toss: Toss) {
    match toss {
        Toss::Coin(face) => {println!("Result of the coin toss: {face:?}")}
        Toss::Dice(color) => {println!("Result of die roll: {color:?}")}
    }
}

fn spin (r: &i32) -> Toss {
    match r {
        0 => {Toss::Coin(Face::Heads)}
        1 => {Toss::Coin(Face::Tails)}
        2 => {Toss::Dice(Color::Red)}
        3 => {Toss::Dice(Color::Green)}
        4 => {Toss::Dice(Color::Blue)}
        _ => {Toss::Dice(Color::Black)}
    }
}

fn main() {
    let r1 = (rand::thread_rng().gen_range(1..=100)) %2;
    let r2 = (rand::thread_rng().gen_range(1..=100)) %4 + 2;
    let coin_toss = spin(&r1);
    let die_toss = spin(&r2);

    result(coin_toss);
    result(die_toss);
}
