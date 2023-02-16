use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day01.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut max = 0;
    let mut current = 0;
    for line in lines {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("max is : {}", max);
}
