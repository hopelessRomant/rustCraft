mod dealer;
use crate::dealer::cars::AstonMartin;

fn main() {
    let car = AstonMartin{ model: String::from("DB12")};
    println!("The car we are selling is : {car:#?}");
}
