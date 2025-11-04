mod dealer;
use crate::dealer::cars::*;

fn main() {
    let car = AstonMartin::car("DB-12");
    println!("The car we are selling is : {car:#?}");
}
