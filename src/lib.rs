use back_of_house::Breakfast;

// like declaring symbolink link to actual path
// we can use keyword 'as' to provide newName in case of conflict or very long name
pub use crate::front_of_the_house::hosting as host;

mod front_of_the_house {
    pub mod hosting {
        pub fn add_to_waitlist() {

        }
        fn seat_at_table() {

        }
    }

    mod serving {
        fn take_order() {

        }
        fn serve_order() {

        }
        fn take_payment() {

        }
    }
}

mod back_of_house {
    // NOTE: we could not create Breakfast directly because seasonal is private 
    // so we need functions for creating the instance
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad,
    }


    fn fix_incorrect_order() {
        cook_order();
        
    }
    fn cook_order() {
        // calling parent module path to deliver_order via super
        super::deliver_order();
    }
}

fn deliver_order() {

}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // Relative path
    front_of_the_house::hosting::add_to_waitlist();
    // using use for short but must declare the crate/module in the scope or current crate
    //hosting::add_to_waitlist();
    host::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about the bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // IMPORTANT! below will not work because field 'seasonal_fruit' is private
    // in this case, we need function like Breakfast::breakfast to instantiate default object
    /* 
    let m = back_of_house::Breakfast {
        toast: String::from("value"),
        seasonal_fruit: String::from("value")
    }; */

    // using enums 
    // NOTE: by default all enum variants are public so we only need to make the enum public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // eleaner + freischuz sr (attack/cd), arines, kry, wiggle, diana

}
