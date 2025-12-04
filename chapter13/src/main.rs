#[allow(dead_code)]
mod store;

// use store::*;

fn main () {
    // let stock = Inventory::build(14, 13, 16);
    // let user1 = Some(Tshirts::Red);
    // let user2 = None;

    // let giveaway1 = stock.give_away(user1);
    // let giveaway2 = stock.give_away(user2);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
