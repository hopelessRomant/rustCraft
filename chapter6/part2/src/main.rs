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

enum Show {
    Coin(Face),
    Dice(Color),
}

fn result (show: Show) {
    match show {
        Show::Coin(face) => {println!("Result of the coin toss: {face:?}")}
        Show::Dice(color) => {println!("Result of die roll: {color:?}")}
    }
}

fn spin (r: &i32) -> Show {
    match r {
        0 => {Show::Coin(Face::Heads)}
        1 => {Show::Coin(Face::Tails)}
        2 => {Show::Dice(Color::Red)}
        3 => {Show::Dice(Color::Green)}
        4 => {Show::Dice(Color::Blue)}
        _ => {Show::Dice(Color::Black)}
    }
}

fn main() {
    let r1 = (rand::thread_rng().gen_range(1..=100)) %2;
    let r2 = (rand::thread_rng().gen_range(1..=100)) %4 + 2;
    let coin_show = spin(&r1);
    let die_show = spin(&r2);

    result(coin_show);
    result(die_show);
}
