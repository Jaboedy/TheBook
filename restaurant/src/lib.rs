// Example of brining two types with the same name into scope using "as"
use std::fmt::Result;
use std::io::Result as IoResult;

// Bringing in 2 items from the same crate/module without bringing in the whole crate/module
use std::{cmp::Ordering, io};

// Bringing an item and the module it belongs to into scope at the same time (commented out to prevent errors due to previously importing io)
// use std::io::{self, Write};

// Glob operator, bring all public items into scope
use std::collections::*;

fn func1() -> Option<Result> {
    let _equal = Ordering::Equal;
    let _result = io::Result::Ok(());
    let _map: HashMap<String, i32> = HashMap::new();
    None
}

fn func2() -> Option<IoResult<()>> {
    None
}

pub fn add(left: u64, right: u64) -> u64 {
    func1();
    func2();
    println!("Left: {left}\nRight: {right}");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            fix_incorrect_order();
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}

// When bringing in functions it is idiomatically correct
// to bring the parent module into scope
// For structs, enums, and other items it is correct to
// bring the item itself into scope
// Adding pub to the front would make it so users of this
// don't also have to specify the full path, and can use
// restaurant::hosting::add_to_waitlist() instead
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    deliver_order();
}

fn deliver_order() {}
