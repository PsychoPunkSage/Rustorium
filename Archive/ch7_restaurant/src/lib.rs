/*
- $ cargo new --lib <name> ::> To create a dir.
*/

// Get contents of `func_of_mod_k` from a file called `func_of_mod_k.rs`
mod func_of_mod_k;

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

// SUPER keyword
fn serve_order() {}
mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        /// By-default if enum = public ;; All the invariants is `public`
        Soup,
        Salad,
    }
    pub struct Breakfast {
        /// By-default fields within a Struct is `private`.
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

    fn fix_wrong_order() {
        cook_order(); // it is within same parent, So, its visible.
        super::serve_order() // Access data from Parent of Parent
    }

    fn cook_order() {}
}
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Parent Module can't see in Children Module BUT Children can see all the things defined inside the Parent module.
*/

use crate::front_of_house::hosting; // Absolute
                                    // use self::front_of_house::hosting; // Relative
                                    // pub use crate::front_of_house::hosting; // External code can access `hosting`

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path - Starts from current module
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}", order1);
    println!("{:?}", order2);

    // `USE` keyword
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//// WAY 1
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     Ok(())
// }

// fn function2() -> io::Result<()> {
//     Ok(())
// }

//// WAY 2
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
