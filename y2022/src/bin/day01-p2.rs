use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day01.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut max = [0; 3];
    let mut current = 0;
    for line in lines {
        let line = line.unwrap();
        if line.trim().is_empty() {
            max.sort();
            for max in &mut max {
                if current > *max {
                    println!(
                        "current bigger than max, max: {}, current: {}",
                        max, current
                    );
                    *max = current;
                    break;
                }
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    let sum: i32 = max.iter().sum();
    println!("sum is : {}, {:?}", sum, max);
}
