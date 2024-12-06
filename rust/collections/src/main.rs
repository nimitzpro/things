use std::collections::HashMap;
use std::fs::File;
// use std::fmt;
// use std::io;
// use core::time;
use std::thread::sleep;

use std::{thread, time};
use std::io::{self, Read, Write, ErrorKind};

// #[derive(strum_macros::print)]
enum Thing {
    Number(i32),
    Float(f32),
    Text(String) 
}

impl Thing {
    fn str(&self) -> &str {
        return stringify!(self);
    }
}

fn print_thing(t: &Thing) {
    match t {
        Thing::Number(i) => print!("{i}"),
        Thing::Float(i) => print!("{i}"),
        Thing::Text(i) => print!("{i}")
    }
}

fn print_vec(list: &Vec<i32>) {
    // let N = list.len();
    // let mut i = 0;
    // while i < N {
    //     print!("{} ", list[i]);
    //     i += 1;
    // }
    for i in list {
        print!("{} ", i);
    }
    println!();
}

fn print_vec_generic(list: &Vec<Thing>) {
    for i in list {
        print_thing(i);
        print!(" ")
    }
    println!();
}

fn inc_vec_elements(list: &mut Vec<i32>) {
    for i in list {
        *i += 50;
    }
}

fn timed_print(s: &str, pause: u64) {
    for i in s.chars() {
        print!("{i}");
        io::stdout().flush().unwrap();
        sleep(time::Duration::from_millis(pause));
    }
    println!();
}

fn read_username_from_file(p: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(p);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_alt(p: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(p)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let x = [1, 2, 3, 4, 5];

    let new_boi: Vec<i32> = Vec::new();

    let mut new_boi = vec![1, 2, 3, 4, 5];
    print_vec(&new_boi);

    new_boi.push(6);
    new_boi.push(7);

    print_vec(&new_boi);
    
    let third: &i32 = &new_boi[2];
    println!("The third element is {third}");

    let third: Option<&i32> = new_boi.get(22);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // let does_not_exist = &new_boi[100]; // panics
    // let does_not_exist = new_boi.get(100); // panics
    let mut does_not_exist: &i32 = new_boi.get(30).unwrap_or(&0); // return default value of 0 if
    // non-existent
    println!("does_not_exist: {}", does_not_exist);
    
    // new_boi[3] = 69;
    new_boi.push(120);

    inc_vec_elements(&mut new_boi);

    print_vec(&new_boi);

    let mut things = vec![Thing::Number(1), Thing::Number(3), Thing::Text("hello world".to_string()), Thing::Float(9.3)];

    print_vec_generic(&things);

    things.push(Thing::Float(3.143532));
    print_vec_generic(&things);

    print_thing(&things.pop().unwrap());
    println!("\n");

    let jikushoukai = "こんにちは！アレックスと申します。21歳のアイルランド人です。".to_string();

    let pause: usize = 100;
    let pause64: u64 = pause.try_into().unwrap_or(500);

    timed_print(&jikushoukai, pause64);
    
    let duration = jikushoukai.chars().count() * pause;
    let mut jikushoukai = format!("\"{}\"", jikushoukai);
    jikushoukai.push_str(&format!("\nこの自己紹介は{duration}ミリセカンドをかかりました。"));

    println!("{jikushoukai}");

    for c in jikushoukai.bytes() {
        print!("{c} ");
    }

    println!("\n");

    let mut scores = HashMap::new();
    scores.insert("Blue", 20);
    scores.insert("Yellow", 40);

    println!("Blue team: {}", scores.get("Blue").unwrap_or(&0));

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let x = "Blue";
    if scores.contains_key(x) {
        println!("pog");
    }
    else {
        println!("cringe");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("Processed {}, count: {}", word, count);
    }
    println!("{:?}", map);

    // panic!("henlo :)");
    
    let greeting_file_result = File::open("hello.txt");
    // using match 
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // using unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // using expect or unwrap (calls panic on execution)
    // let greeting_file = File::open("hello1.txt").expect("sadge"); // .unwrap();

    println!("{:?}", read_username_from_file("hello.txt"));
    println!("{:?}", read_username_from_file_alt("pain.txt"));

}
