use iterators::*;

pub mod iterators;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn test_closure() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(format!("{}: {}", &value, r.width));
        r.width
    });
    println!("{:#?}", list);
    println!("{:?}", sort_operations);
}

fn test_iterators() {
    let vector = [1, 2, 3];

    let mut vector_iterator = vector.iter();
    assert_eq!(Some(&1), vector_iterator.next());
    println!("{}", vector_iterator.next().unwrap());
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn fibbing() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array: [u32; 4] = [1, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

fn main() {
    test_closure();
    test_iterators();
    // let mut fib = fibonacci();
    // for i in 0..10 { 
    //     print!("{:?}", fib.next());
    // }
    
    fibbing();
    
    println!("{}", fibonacci().skip(100).take(1).next().unwrap());

    let mut simp = make_simple_iter(vec![12,24,36,48,60,72,84,96,108]);

    simp.reset();
    println!("reset value: {}", simp.next().unwrap());

    for x in simp {
        println!("{}", x);
    }

}
