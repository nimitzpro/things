fn main() {
    println!("Zamn");
    let y = {
        let x = 3;
        x + 1
    };
    test_func(plus_one(five()), y);

    let str_literal = "i am literally a string";
    println!("{}", str_literal);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    let s2 = s; // moves the string to a non-mutable variable, invalidating s
                // s2.push_str(" >:)"); // fails
    println!("{}", s2);

    use_string(s2.clone());

    println!("try using the string now: {s2}");

    let (x1, x2) = calculate_lengths(&s2);
    println!("{}, {}", x1, x2);
    println!("string is still available, pointer is passed by reference: {s2}");

    let mut s3 = s2; // replace immutable reference with mutable reference
    modify(&mut s3);

    println!("{s3}");

    let a: &str = first_word(&s3); // get reference to first word
    println!("a (pointer): {a}");
    let mut a: String = String::from(a); // make a copy of the first word
    s3 = String::from("goodbye, world");
    println!("a (new string): {}", a);
    a.push_str(" 🔥 🔥 🔥");
    println!("first word: {a}");
    println!("s3: {}", s3);

    let mut a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    a[1] = 5;
    a[2] = 5;
    assert_eq!(&a[1..3], &[5, 5]);
}

fn test_func(x: i32, y: i32) {
    let result = x * y;
    println!("x * y = {x} * {y} = {result}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn use_string(string: String) {
    println!("string cloned from s2 into string: {string}");
}

fn calculate_lengths(s: &String) -> (usize, usize) {
    (s.len(), s.chars().count())
}

fn modify(s: &mut String) {
    s.push_str(", >_<");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
