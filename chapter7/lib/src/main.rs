use lib::back_of_house;
use lib::back_of_house::Appetizer;
use lib::eat_at_restraunt;

fn main () {
    let meal = back_of_house::Breakfast::monday("chai");
    eat_at_restraunt();
    print!("{:#?}\n", meal); // the meal in the 'eat_at_restraunt function and the 'meal' defined in main are different things.

    let starter = Appetizer::Salad;
    println!("Starter order : {:?}", starter);
}
