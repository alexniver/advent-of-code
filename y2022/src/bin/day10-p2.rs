use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day10.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut cycle = 0;
    let mut x = 1;

    let mut u8_arr = vec![];

    let mut cycle_add = |cycle: &mut i32, x: &i32| -> () {
        if *cycle != 0 && *cycle % 40 == 0 {
            u8_arr.push(b'\n');
        }

        let c = *cycle % 40;
        if c == *x - 1 || c == *x || c == *x + 1 {
            u8_arr.push(b'#')
        } else {
            u8_arr.push(b'.')
        }

        *cycle += 1;
    };

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let cmd = parts.next();
        match cmd {
            Some("addx") => {
                for _ in 0..2 {
                    cycle_add(&mut cycle, &x);
                }
                let v = parts.next().unwrap().parse::<i32>().unwrap();
                x += v;
            }
            Some("noop") => {
                cycle_add(&mut cycle, &x);
            }
            _ => {}
        }
    }

    println!("{}", String::from_utf8(u8_arr).unwrap());
}
