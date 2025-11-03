#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[allow(dead_code)]
fn deliver_order() {}

#[allow(dead_code)]
pub mod back_of_house{
    fn cook_order() {
        // --snip--
        // absolute path
        crate::deliver_order();

        //relative path
        super::deliver_order();
    }
    #[derive(Debug)]
    pub struct Breakfast {
        pub drink: String,
        special: String,
    }

    impl Breakfast{
        pub fn monday (drink: &str) -> Breakfast {
            Breakfast { drink: String::from(drink), special: String::from("red-velvet sunday") }
        }
    }   
}

pub fn eat_at_restraunt() { // since eat_at_restraunt and front_of_house are siblings, front_of_house need not be public

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::monday("chai");
    meal.drink = String::from("coffe");
    print!("{:#?} \n",meal);
}   
