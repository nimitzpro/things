use std::fs;

fn p1(file: &str) {
    let mut total = 0;
    for line in file.lines() {
        let p: Vec<Vec<i32>> = line
            .get(9..)
            .map(|q| {
                q.split(" | ").map(|r| {
                    r.split_whitespace()
                        .map(|x| {
                            // println!("{}", x);
                            x.parse::<i32>().unwrap()
                        })
                        .collect()
                })
            })
            .expect("cring")
            .collect();

        let mut running_score = 1;
        p[1].iter()
            .filter(|x| p[0].contains(x))
            .for_each(|_| running_score *= 2);

        total += running_score / 2;
    }
    println!("total: {}", total);
}

fn p2(file: &str) {
    let mut copies = vec![1; file.lines().count()];
    let mut index = 0;
    for line in file.lines() {
        let p: Vec<Vec<&str>> = line
            .get(9..)
            .unwrap()
            .split(" | ")
            .map(|r| r.split_whitespace().collect())
            .collect();
        let mut cards_getting_copies = p[1].iter().filter(|number| p[0].contains(number)).count();
        let multiplier = copies[index];
        index += 1;

        // let mut i = index;
        copies[index..index + cards_getting_copies]
            .iter_mut()
            .for_each(|copy| *copy += multiplier);
        // while cards_getting_copies > 0 {
        //         copies[i] += multiplier;
        //         copies.push(multiplier);
        //     i += 1;
        //     cards_getting_copies -= 1;
        // }
    }

    println!("{:?}", copies);
    println!("total: {}", copies.iter().sum::<i32>());
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    // p1(&file);
    p2(&file);
}
