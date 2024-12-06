use restaurant::back_of_house::*;
use restaurant::front_of_house::*;

fn main() {
    eat_at_restaurant();

    let b = Breakfast::summer("wholewheat");

    println!("{:?}", b);

    hosting::add_to_waitlist();
}
