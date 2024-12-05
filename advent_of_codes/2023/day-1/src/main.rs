use std::thread::current;
use std::{fs, ascii::AsciiExt};
use std::str::{self, Chars};

fn p1(contents: &str) {
    let mut total = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }
        println!("processing {}", line);

        let mut number: u32 = 0;
        for character in line.chars() {
            if let Some(x) = character.to_digit(10) {
                number = x * 10;
                println!("Added to number: {}", number);
                break;
            }
        }
        let a: u32 = char::from(line.as_bytes()[line.rfind(|x: char| x.is_digit(10)).unwrap_or_default()]).to_digit(10).unwrap();
        println!("AAAAA: {}", a);
        number += a;
        println!("number now: {}\n", number);
        total += number;
        }
    println!("{}", total);
}

static DIGITS: &'static [&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

fn matcher(c: &str) -> Option<u32> {
    for (index, number) in DIGITS.iter().enumerate() {
        if let Some(_) = c.find(number) {
            return Some(index as u32);
        }
    }
    None
}

fn check_line(line: Chars) -> u32 {
    let mut number: u32 = 0;
    let mut current_str = String::new();
    let mut line_iter = line.into_iter();
    for c in line_iter.by_ref() {
        if let Some(x) = c.to_digit(10) {
            number = x*10;
            current_str = "".into();
            break;
        }
        current_str.push(c);
        if let Some(x) = matcher(&current_str) {
            number = (x+1) * 10;
            current_str = "".into();
            break;
        }
    }

    for c in line_iter.rev() {
        if let Some(x) = c.to_digit(10) {
            number += x;
            current_str = "".into();
            break;
        }
        current_str = format!("{}{}", c, current_str);
        if let Some(x) = matcher(&current_str) {
            number += (x+1);
            current_str = "".into();
            break;
        }
    }
    number
}

fn p2(contents: &str) {
    let mut total: u32 = 0;
    for line in contents.lines() {
        let number = check_line(line.chars());
        println!("number: {}", number);
        total += number;
    }
    println!("Total: {}", total);
}

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./input.txt").unwrap();
    // p1(&contents);
    p2(&contents);
 }
