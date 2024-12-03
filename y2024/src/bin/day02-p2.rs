use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input-day02-p2.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut safe_num = 0;
    for line in lines.map_while(Result::ok) {
        let arr = line
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if check_safe(&arr) {
            safe_num += 1;
        } else {
            for idx in 0..arr.len() {
                let mut arr_clone = arr.clone();
                arr_clone.remove(idx);
                if check_safe(&arr_clone) {
                    safe_num += 1;
                    break;
                }
            }
        }
    }
    println!("{}", safe_num);
}

fn check_safe(arr: &[i32]) -> bool {
    for (idx, &num) in arr.iter().enumerate() {
        if idx + 2 < arr.len() {
            let v1 = arr[idx + 1] - num;
            let v2 = arr[idx + 2] - arr[idx + 1];

            if v1 * v2 < 0 {
                return false;
            }

            if !(1..=3).contains(&v1.abs()) || !(1..=3).contains(&v2.abs()) {
                return false;
            }
        }
    }

    true
}
