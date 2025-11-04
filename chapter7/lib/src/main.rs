#[allow(dead_code)]
pub mod front_of_house;

#[allow(dead_code)]
pub mod back_of_house;

pub use front_of_house::*;
pub use back_of_house::*;

fn main () {
    hosting::add_to_waitlist();

    let meal = managing::Breakfast::monday("chai");
    print!("{:#?}\n", meal);

    let starter = managing::Appetizer::Salad;
    println!("Starter order : {:?}", starter);
}
