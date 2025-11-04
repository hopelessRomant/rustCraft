use lib::back_of_house::{Appetizer, Breakfast};
use lib::eat_at_restraunt as food;

fn main () {
    let meal = Breakfast::monday("chai");
    food();
    print!("{:#?}\n", meal); // the meal in the 'eat_at_restraunt function and the 'meal' defined in main have different scopes.

    let starter = Appetizer::Salad;
    println!("Starter order : {:?}", starter);
}
