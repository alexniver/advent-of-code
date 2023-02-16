use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day04.txt").unwrap();

    let lines = BufReader::new(f).lines();

    let mut counter = 0;
    for line in lines {
        let line = line.unwrap();

        let mut pairs = line.split(",");
        let (min1, max1) = parse_min_max(pairs.next().unwrap());
        let (min2, max2) = parse_min_max(pairs.next().unwrap());

        if (min1 <= min2 && max1 >= min2) || (min2 <= min1 && max2 >= min1) {
            counter += 1;
        }
    }

    println!("counter: {}", counter);
}

fn parse_min_max(s: &str) -> (u32, u32) {
    let mut pairs = s.split("-");
    (
        pairs.next().unwrap().parse::<u32>().unwrap(),
        pairs.next().unwrap().parse::<u32>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::parse_min_max;

    #[test]
    fn parse_min_max_test() {
        assert_eq!(parse_min_max("2-3"), (2, 3));
    }
}
