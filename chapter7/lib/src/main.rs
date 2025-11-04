use lib::back_of_house;
use lib::eat_at_restraunt as food;

fn main () {
    let meal = back_of_house::Breakfast::monday("chai");
    food();
    print!("{:#?}\n", meal); // the meal in the 'eat_at_restraunt function and the 'meal' defined in main have different scopes.

    let starter = back_of_house::Appetizer::Salad;
    println!("Starter order : {:?}", starter);
}
