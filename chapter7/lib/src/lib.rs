#[allow(dead_code)]
pub mod front_of_house;

pub fn deliver_order() {}

#[allow(dead_code)]
pub mod back_of_house;

pub use front_of_house::hosting::add_to_waitlist; // adding to all the scopes.

pub fn eat_at_restraunt() { // since eat_at_restraunt and front_of_house are siblings, front_of_house need not be public

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
    // calling from global scope
    add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::monday("chai");
    meal.drink = String::from("coffe");
    print!("{:#?} \n",meal);
}   
