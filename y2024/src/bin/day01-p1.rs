use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let f = File::open("input-day01-p1.txt").unwrap();
    let lines = BufReader::new(f).lines();
    let mut v1 = vec![];
    let mut v2 = vec![];
    for line in lines.map_while(Result::ok) {
        let mut s = line.split("   ");
        let s1 = s.next().unwrap();
        let s2 = s.next().unwrap();
        v1.push(s1.parse::<u32>().unwrap());
        v2.push(s2.parse::<u32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let zip = zip(v1, v2);
    let result = zip.map(|(a, b)| a.abs_diff(b)).sum::<u32>();
    println!("{}", result);
}