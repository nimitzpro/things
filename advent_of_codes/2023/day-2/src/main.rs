use std::fs;

static CONSTRAINTS: [i32; 3] = [12, 13, 14];

fn p1(file: &str) {
    let mut total = 0;
    let mut valid: bool;
    for line in file.lines() {
        valid = true;
        let mut section_iter = line.split(":");
        let id: i32 = section_iter.next().unwrap()[5..].parse().unwrap();
        let game_iter = section_iter.next().unwrap().split(";");
        'outer: for round in game_iter {
            for colour in round.trim().split(", ") {
                // println!("worb: {}", colour.trim());
                let vals: Vec<&str> = colour.split(" ").collect();
                // println!("{:?}", vals);
                match vals[1] {
                    "red" => {
                        if vals[0].parse::<i32>().unwrap() > CONSTRAINTS[0] {
                            valid = false;
                            break 'outer;
                        }
                    }
                    "green" => {
                        if vals[0].parse::<i32>().unwrap() > CONSTRAINTS[1] {
                            valid = false;
                            break 'outer;
                        }
                    }
                    "blue" => {
                        if vals[0].parse::<i32>().unwrap() > CONSTRAINTS[2] {
                            valid = false;
                            break 'outer;
                        }
                    }
                    _ => (),
                }
            }
        }
        if valid {
            // println!("id: {}", id);
            total += id;
        }
    }
    println!("total: {}", total);
}

fn p2(file: &str) {
    let mut total = 0;
    let (mut min_red, mut min_green, mut min_blue);
    for line in file.lines() {
        let mut section_iter = line.split(":");
        section_iter.next();
        let game_iter = section_iter.next().unwrap().split(";");
        (min_red, min_green, min_blue) = (0, 0, 0);
        for round in game_iter {
            for colour in round.trim().split(", ") {
                // println!("worb: {}", colour.trim());
                let vals: Vec<&str> = colour.split(" ").collect();
                // println!("{:?}", vals);
                match vals[1] {
                    "red" => {
                        let temp = vals[0].parse::<i32>().unwrap();
                        if temp > min_red {
                            min_red = temp;
                        }
                    }
                    "green" => {
                        let temp = vals[0].parse::<i32>().unwrap();
                        if temp > min_green {
                            min_green = temp;
                        }
                    }
                    "blue" => {
                        let temp = vals[0].parse::<i32>().unwrap();
                        if temp > min_blue {
                            min_blue = temp;
                        }
                    }
                    _ => (),
                }
            }
        }
        // println!("{}, {}, {}", min_red, min_green, min_blue);
        // println!("id: {}", id);
        total += min_red * min_green * min_blue;
    }
    println!("total: {}", total);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    p1(&file);
    p2(&file);
}
