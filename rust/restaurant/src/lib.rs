pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
    
        pub fn seat_at_table() {
            println!("seated at table");
        }
    }
    
    mod serving {
        fn _take_order() {}
    
        fn _serve_order() {}
    
        fn _take_payment() {}
    }
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // Relative path
        hosting::seat_at_table();
    }
}

pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // change to &str with lifetimes?
    }

    const SEASONAL_FRUITS: [&str; 4] = ["peaches", "apples", "strawberries", "watermelon"];

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            let current = 2;
            println!("The seasonal fruit is {}", SEASONAL_FRUITS[current]);

            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from(SEASONAL_FRUITS[current])
            }
        }
    }
}
