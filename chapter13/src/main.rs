#[allow(dead_code)]
mod store;

use store::*;

fn main () {
    let stock = Inventory::build(14, 13, 16);
    let user1 = Some(Tshirts::Red);
    let user2 = None;

    let giveaway1 = stock.give_away(user1);
    let giveaway2 = stock.give_away(user2);

    println!(
        "The user with preference {:?} gets {:?}",
        user1.unwrap(), giveaway1
    );
    println!(
        "The user with no preference gets {:?}",
        giveaway2
    );
}