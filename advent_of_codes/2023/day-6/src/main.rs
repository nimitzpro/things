use std::fs;

fn p1(file: &str, line_length: usize) {
    let file: Vec<&str> = file.split_whitespace().collect();
    let times: Vec<i32> = file[1..line_length]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let distances: Vec<i32> = file[line_length + 1..]
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut margins = Vec::<i32>::new();
    let mut i = 0;
    let mut speed: i32;
    let mut time: i32;
    let mut margin: i32;
    while i < times.len() {
        margin = 0;
        speed = 1;
        time = times[i] - 1;
        while time > speed {
            if speed * time > distances[i] {
                // println!(
                //     "speed: {}, time: {}, distance: {}",
                //     speed, time, distances[i]
                // );
                margin += 1;
            }
            time -= 1;
            speed += 1;
        }
        margin *= 2;
        if speed == time && speed * time > distances[i] {
            // println!("{}, {}", speed, time);
            margin += 1;
        }
        margins.push(margin);
        i += 1;
    }

    // println!("{:?}", margins);
    let total = margins.iter().fold(1, |acc, x| acc * x);
    println!("{}", total);
}

fn p2(file: &str) {
    let vals: Vec<u64> = file
        .lines()
        .map(|line| {
            line.split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .fold(String::from(""), |acc: String, x: &str| acc + x)
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    let mut speed = 1;
    let mut margin = 0;
    let mut time = vals[0] - 1;
    while time > speed {
        if speed * time > vals[1] {
            // println!(
            //     "speed: {}, time: {}, distance: {}",
            //     speed, time, vals[i]
            // );
            margin += 1;
        }
        time -= 1;
        speed += 1;
    }
    margin *= 2;
    if speed == time && speed * time > vals[1] {
        // println!("{}, {}", speed, time);
        margin += 1;
    }

    println!("{}", margin);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    p1(&file, 5);
    p2(&file);
}
