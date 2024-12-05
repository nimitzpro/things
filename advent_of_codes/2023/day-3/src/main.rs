use std::fs;
fn f(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn p1(file_matrix: &Vec<Vec<char>>) {
    let mut total: u64 = 0;
    let mut temp_number: u64 = 0;
    let mut valid = false;
    // println!("{}", file_matrix[1][34]);
    for (line_i, line) in file_matrix.iter().enumerate() {
        if valid {
            total += temp_number;
        }
        temp_number = 0;
        valid = false;
        for (i, c) in line.iter().enumerate() {
            // println!("{},{} -> {}", line_i, i, c);
            if *c == '.' {
                if valid && temp_number != 0 {
                    // println!("@{}", temp_number);
                    total += temp_number;
                    temp_number = 0;
                    valid = false;
                } else {
                    temp_number = 0;
                    valid = false;
                }
            } else if let Some(x) = c.to_digit(10) {
                temp_number = (temp_number * 10) + x as u64;
                if !valid {
                    // let mut above @ mut
                    // above_left @ mut above_right @ mut below @ mut below_left @ mut
                    // below_right = false;
                    if line_i > 0 {
                        if f(file_matrix[line_i - 1][i]) {
                            valid = true;
                            continue;
                        }
                        if i > 0 && f(file_matrix[line_i - 1][i - 1]) {
                            valid = true;
                            continue;
                        }
                        if i < line.len() - 1 && f(file_matrix[line_i - 1][i + 1]) {
                            valid = true;
                            continue;
                        }
                    }
                    if line_i < file_matrix.len() - 1 {
                        if f(file_matrix[line_i + 1][i]) {
                            valid = true;
                            continue;
                        }
                        if i > 0 && f(file_matrix[line_i + 1][i - 1]) {
                            valid = true;
                            continue;
                        }
                        if i < line.len() - 1 && f(file_matrix[line_i + 1][i + 1]) {
                            valid = true;
                            continue;
                        }
                    }
                }
            } else {
                valid = true; // still true for next number
                if temp_number != 0 {
                    // println!("/{}", temp_number);
                    total += temp_number;
                    temp_number = 0;
                }
            }
        }
    }
    println!("total: {}", total);
}

fn construct_number(file_matrix: &Vec<Vec<char>>, row: usize, col: usize) -> (u32, i32) {
    let mut number = file_matrix[row][col].to_digit(10).unwrap();
    let mut power = 1;
    for c in file_matrix[row][..col].iter().rev() {
        match c.to_digit(10) {
            Some(x) => {
                number = x * (10 as u32).pow(power) + number;
                power += 1;
            }
            None => break,
        }
    }
    let mut used = 0;
    for c in file_matrix[row][col + 1..].iter() {
        match c.to_digit(10) {
            Some(x) => {
                number = number * 10 + x;
                used += 1;
            }
            None => break,
        }
    }
    println!("grabbed number: {}", number);
    (number, used)
}

fn p2(file_matrix: &Vec<Vec<char>>) {
    let mut total = 0;
    // let mut a: [i32; 2] = [0, 0];
    let mut temp: u32;
    let mut used: i32;

    for (line_i, line) in file_matrix.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            temp = 0;
            used = 0;
            if *c == '*' {
                if line_i > 0 {
                    if file_matrix[line_i][i - 1].is_digit(10) {
                        // left
                        if temp == 0 {
                            temp = construct_number(file_matrix, line_i, i - 1).0;
                        } else {
                            total += temp * construct_number(file_matrix, line_i, i - 1).0;
                            continue;
                        }
                    }
                    if i > 0 && file_matrix[line_i - 1][i - 1].is_digit(10) {
                        // above + left
                        if temp == 0 {
                            (temp, used) = construct_number(file_matrix, line_i - 1, i - 1);
                        } else {
                            total += temp * construct_number(file_matrix, line_i - 1, i - 1).0;
                            continue;
                        }
                    }
                    if file_matrix[line_i - 1][i].is_digit(10) && used == 0 {
                        // above
                        if temp == 0 {
                            (temp, used) = construct_number(file_matrix, line_i - 1, i);
                        } else {
                            total += temp * construct_number(file_matrix, line_i - 1, i).0;
                            continue;
                        }
                    }
                    used -= std::cmp::max(0, used - 1);
                    if i < line.len() - 1 {
                        if file_matrix[line_i - 1][i + 1].is_digit(10) && used == 0 {
                            // above right
                            if temp == 0 {
                                temp = construct_number(file_matrix, line_i - 1, i + 1).0;
                            } else {
                                total += temp * construct_number(file_matrix, line_i - 1, i + 1).0;
                                continue;
                            }
                        }
                        if file_matrix[line_i][i + 1].is_digit(10) {
                            // right
                            if temp == 0 {
                                temp = construct_number(file_matrix, line_i, i + 1).0;
                            } else {
                                total += temp * construct_number(file_matrix, line_i, i + 1).0;
                                continue;
                            }
                        }
                    }
                }
                used = 0;
                if line_i < file_matrix.len() - 1 {
                    if i > 0 && file_matrix[line_i + 1][i - 1].is_digit(10) {
                        // bottom left
                        if temp == 0 {
                            (temp, used) = construct_number(file_matrix, line_i + 1, i - 1);
                        } else {
                            total += temp * construct_number(file_matrix, line_i + 1, i - 1).0;
                            continue;
                        }
                    }
                    if file_matrix[line_i + 1][i].is_digit(10) && used == 0 {
                        // bottom
                        if temp == 0 {
                            (temp, used) = construct_number(file_matrix, line_i + 1, i);
                        } else {
                            total += temp * construct_number(file_matrix, line_i + 1, i).0;
                            continue;
                        }
                    }
                    used -= std::cmp::max(0, used - 1);
                    if i < line.len() - 1
                        && file_matrix[line_i + 1][i + 1].is_digit(10)
                        && used == 0
                        && temp != 0
                    {
                        total += temp * construct_number(file_matrix, line_i + 1, i + 1).0;
                    }
                }
            }
        }
    }

    println!("total: {}", total);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let file_matrix: Vec<Vec<char>> = file.lines().map(|s| s.chars().collect()).collect();
    p1(&file_matrix);

    p2(&file_matrix);
}
