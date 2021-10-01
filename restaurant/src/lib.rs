mod front_of_house;

pub fn eat_at_restaurant() {
    // absolute path
//    use crate::front_of_house::hosting;
    // relative path 
//    use front_of_house::hosting;
//    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
//     meal.seasonal_fruit = String::from("blueberries");

}

fn serve_order() {}

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
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }   
        }
    }
    

    fn fix_incorrect_order() {
        cook_order();
        // super corresponds to '..' in file system
        // references parent scope (here 'crate')
        super::serve_order();
    }
    fn cook_order() {}
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
