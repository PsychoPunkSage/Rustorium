/*
- $ cargo new --lib <name> ::> To create a dir.
*/

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

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path - Starts from current module
    front_of_house::hosting::add_to_waitlist();
}
