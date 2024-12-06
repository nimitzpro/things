#![allow(unused)]

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn handler(x: Option<i32>, y: Option<i32>) {
    if x.is_some() && y.is_some() {
        let x = x.unwrap();
        let y = y.unwrap();
        println!("{} + {} = {}", x, y, x+y);
    }
    else {
        println!("Error fetching one of the parameters...");
    }
}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn increment(number: Option<i32>) -> Option<i32> {
    match number {
        Some(i) => Some(i+1),
        None => None
    }
}


fn main() {

    let addr1 = IpAddr::V4(String::from("192.168.0.1"));
    let addr2 = IpAddr::V4(String::from("192.168.1.1"));
    let addr3 = IpAddr::V6(String::from("::1"));

    println!("{:?}", addr1);
    
    let some_char = Some('e');

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    handler(some_number, absent_number);
    handler(Some(5), Some(10));
    
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    println!("{:?}", increment(Some(32)));
    println!("{:?}", increment(None));

    match 13 {
        10 => println!("ten!"),
        15 => println!("fifteen!"),
        other => println!("nop: {}", other)
        // _ => println!("alternative without value")
    }

    // let config_max = Some(3u8);
    let config_max:Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    else {
        println!("pain");
    }

}
