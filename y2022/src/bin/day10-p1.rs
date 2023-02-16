use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day10.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut cycle = 0;
    let mut x = 1;

    let mut sum = 0;

    let mut target_cycle_iter = vec![20, 60, 100, 140, 180, 220].into_iter();
    let mut target_cycle = target_cycle_iter.next().unwrap();

    let mut target_cycle_x_arr = vec![];

    let cycle_add = |cycle: &mut i32,
                     x: &i32,
                     target_cycle_iter: &mut std::vec::IntoIter<i32>,
                     target_cycle: &mut i32,
                     target_cycle_x_arr: &mut Vec<i32>,
                     sum: &mut i32|
     -> () {
        *cycle += 1;
        if *cycle == *target_cycle {
            *sum += *cycle * *x;
            target_cycle_x_arr.push(*x);
            if let Some(v) = target_cycle_iter.next() {
                *target_cycle = v;
            }
        }
    };

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let cmd = parts.next();
        match cmd {
            Some("addx") => {
                for _ in 0..2 {
                    cycle_add(
                        &mut cycle,
                        &x,
                        &mut target_cycle_iter,
                        &mut target_cycle,
                        &mut target_cycle_x_arr,
                        &mut sum,
                    );
                }
                let v = parts.next().unwrap().parse::<i32>().unwrap();
                x += v;
            }
            Some("noop") => {
                cycle_add(
                    &mut cycle,
                    &x,
                    &mut target_cycle_iter,
                    &mut target_cycle,
                    &mut target_cycle_x_arr,
                    &mut sum,
                );
            }
            _ => {}
        }
    }
    println!("arr: {:?}, sum : {:?}", target_cycle_x_arr, sum);
}
