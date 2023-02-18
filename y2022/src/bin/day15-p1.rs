use std::{collections::HashSet, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let s = fs::read_to_string("input-day15.txt").unwrap();
    process(&s);
}

fn process(s: &str) {
    let mut occipued_set = HashSet::new();

    let sensor_arr = line(s)
        .unwrap()
        .1
        .iter()
        .map(|&((x, y), (x2, y2))| {
            occipued_set.insert((x, y));
            occipued_set.insert((x2, y2));
            Sensor {
                x,
                y,
                closest_distance: distance(x, y, x2, y2),
            }
        })
        .collect::<Vec<Sensor>>();

    let mut no_beacon_set = HashSet::new();
    let row = 2000000;
    // let row = 10;
    for s in sensor_arr.iter() {
        let mut i = 0;
        while distance(s.x + i, row, s.x, s.y) <= s.closest_distance {
            for x in [s.x + i, s.x - i] {
                if occipued_set.get(&(x, row)).is_none() {
                    no_beacon_set.insert((x, row));
                }
            }
            i += 1;
        }
    }

    println!("sum : {:?}", no_beacon_set.len());
}

fn line(s: &str) -> IResult<&str, Vec<((i64, i64), (i64, i64))>> {
    separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(position, tag(": closest beacon is at "), position),
        ),
    )(s)
}

fn position(s: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(s)
}

struct Sensor {
    x: i64,
    y: i64,
    // closest_beacon: Beacon,
    closest_distance: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon {
    x: i64,
    y: i64,
}

fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[cfg(test)]
mod tests {
    use crate::{distance, line, position};

    #[test]
    fn position_test() {
        assert_eq!(position("x=2, y=3"), Ok(("", (2, 3))));
    }

    #[test]
    fn distance_test() {
        assert_eq!(distance(0, 0, 0, 1), 1);
        assert_eq!(distance(0, 0, 0, 2), 2);
        assert_eq!(distance(0, 0, 2, 0), 2);
        assert_eq!(distance(0, 0, 2, 2), 4);
    }

    #[test]
    fn line_test() {
        assert_eq!(
            line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok(("", vec![((2, 18), (-2, 15))]))
        )
    }
}
