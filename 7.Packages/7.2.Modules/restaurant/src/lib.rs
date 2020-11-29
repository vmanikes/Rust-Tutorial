// modules take in module name and a body
// modules can have nested modules
// modules, can hold, struct, enum, functions, constants, traits etc
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}

        const TOTAL_SEATS: u8 = 10;
    }

    mod serving {
        struct Waiter {
            name: String,
            age: u8
        }

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Accessing functions
// We call the module functions using ::
pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();
}