use crate::garden::*;
use crate::garden::vegetables::*;

use ::restaurant::*;

pub mod garden;

fn main() {
    println!("Hello, world!");
    test_func();

    let asp = Vegetable { name: String::from("ASPARAGUS"), mood: String::from("angery") };
    asp.amazing();

    let potato = Vegetable { name: String::from("POTATO"), mood: String::from("chill") };
    potato.amazing();

    restaurant::front_of_house::eat_at_restaurant();

    let b = restaurant::back_of_house::Breakfast::summer("white");
    println!("{:?}", b);
}
