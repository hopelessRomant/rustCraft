use lib::back_of_house;

fn main () {
    let mut meal = back_of_house::Breakfast::monday("chai");
    meal.drink = String::from("cofee");
    print!("{:#?}\n", meal);
}
