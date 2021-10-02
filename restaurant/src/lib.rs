mod front_of_house;
use front_of_house::{hosting, serving};

pub mod restaurant_stuff {
    pub fn eat_at_restaurant() {
        // absolute path
        //    use crate::front_of_house::hosting;
        // relative path
        //    use front_of_house::hosting;
        //    hosting::add_to_waitlist();

        super::hosting::add_to_waitlist();
        super::hosting::seat_at_table();

        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        super::back_of_house::cook_order();
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // super corresponds to '..' in file system
        // references parent scope (here 'crate')
        super::serving::serve_order();
    }
    pub fn cook_order() {
        println!("Order up!");
    }
}
