use std::fs;

#[derive(Debug)]
struct Range {
    lower_bound: i64,
    upper_bound: i64,
    shift: i64,
}
impl Range {
    fn in_bounds(&self, value: i64) -> bool {
        self.lower_bound <= value && value <= self.upper_bound
    }
}

#[derive(Debug)]
struct Category {
    ranges: Vec<Range>,
}
impl Category {
    fn transform(&self, value: &mut i64) {
        for range in &self.ranges {
            if range.in_bounds(*value) {
                *value += range.shift;
                break;
            }
        }
    }
}

fn gen_category(file_iter: &mut std::str::Lines) -> Category {
    file_iter.next();
    Category {
        ranges: file_iter
            .map_while(|line| (!line.is_empty()).then_some(line))
            .map(|line| {
                let temp: Vec<i64> = line
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                Range {
                    lower_bound: temp[1],
                    upper_bound: temp[1] + temp[2] - 1,
                    shift: temp[0] - temp[1],
                }
            })
            .collect(),
    }
}

fn pain(mut file_iter: std::str::Lines, mut seeds: Vec<i64>) -> Option<()> {
    let seed_soil = gen_category(&mut file_iter);
    let soil_fertiliser = gen_category(&mut file_iter);
    let fertiliser_water = gen_category(&mut file_iter);
    let water_light = gen_category(&mut file_iter);
    let light_temperature = gen_category(&mut file_iter);
    let temperature_humidity = gen_category(&mut file_iter);
    let humidity_location = gen_category(&mut file_iter);

    let transforms: [Category; 7] = [
        seed_soil,
        soil_fertiliser,
        fertiliser_water,
        water_light,
        light_temperature,
        temperature_humidity,
        humidity_location,
    ];

    // println!("{:?}", transforms);

    seeds.iter_mut().for_each(|x| {
        transforms.iter().for_each(|cat| {
            cat.transform(x);
            // print!(" -> {x}");
        });
        // println!("");
    });

    // println!("{:?}", seeds);
    println!("{:?}", seeds.iter().min().unwrap());

    Some(())
}

fn p1(file: &str) -> Option<()> {
    let mut file_iter = file.lines();
    let mut seeds: Vec<i64> = file_iter
        .next()?
        .get(7..)?
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    file_iter.next();
    pain(file_iter, seeds);

    Some(())
}

fn p2(file: &str) -> Option<()> {
    // takes a while
    let mut file_iter = file.lines();
    let mut seed_ranges: Vec<i64> = file_iter
        .next()?
        .get(7..)?
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut seeds = Vec::<i64>::new();
    let mut i = 0;
    while i < seed_ranges.len() - 2 {
        for j in seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1] {
            seeds.push(j);
        }
        i += 2;
    }

    println!("{:?}", seeds);

    file_iter.next();
    pain(file_iter, seeds);

    Some(())
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    p1(&file);
    p2(&file);
}
