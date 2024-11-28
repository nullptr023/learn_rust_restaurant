mod front_of_house; // load the file front_of_house in module tree
mod back_of_house;

use back_of_house::Breakfast;

// like declaring symbolink link to actual path
// we can use keyword 'as' to provide newName in case of conflict or very long name
pub use crate::front_of_house::hosting as host;

fn deliver_order() {

}

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_the_house::hosting::add_to_waitlist();
    // Relative path
    //front_of_the_house::hosting::add_to_waitlist();
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
