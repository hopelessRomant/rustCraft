#[allow(dead_code)]
pub mod front_of_house;

#[allow(dead_code)]
pub mod back_of_house;


pub use front_of_house::*;
pub use back_of_house::*;

pub fn eat_at_restraunt() { // since eat_at_restraunt and front_of_house are siblings, front_of_house need not be public

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
    // calling from global scope;
    hosting::add_to_waitlist();

    let mut meal = managing::Breakfast::monday("chai");
    meal.drink = String::from("coffe");
    print!("{:#?} \n",meal);
}   
