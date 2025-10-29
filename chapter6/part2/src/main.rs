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

fn spin_coin (r: &i32) -> Show {
    match r {
        0 => {Show::Coin(Face::Tails)}
        _ => {Show::Coin(Face::Heads)}
    }
}

fn spin_dice (r: &i32) -> Show {
    match r {
        0 => {Show::Dice(Color::Red)}
        1 => {Show::Dice(Color::Green)}
        2 => {Show::Dice(Color::Blue)}
        _ => {Show::Dice(Color::Black)}
    }
}

fn main() {
    let r1 = (rand::thread_rng().gen_range(1..=100)) %2;
//    dbg!(r1);
    let r2 = (rand::thread_rng().gen_range(1..=100)) %4;
//    dbg!(r2);
    let coin_show = spin_coin(&r1);
    let die_show = spin_dice(&r2);

    result(coin_show);
    result(die_show);
}
