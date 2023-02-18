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

    let coord_max = 4000000;
    // let coord_max = 20;

    let mut scan_range_arr: Vec<Vec<(i64, i64)>> = Vec::with_capacity(coord_max);
    for y in 0..=coord_max {
        scan_range_arr.push(vec![]);
        for s in sensor_arr.iter() {
            if let Some(scan_range) = s.scan_range_in_y(y as i64) {
                scan_range_arr[y].push(scan_range);
            }
        }
    }

    for y in 0..=coord_max {
        scan_range_arr[y].sort_by(|a, b| match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            res => res,
        });

        let mut max_left_x = scan_range_arr[y][0].1;
        for i in 0..(scan_range_arr[y].len() - 1) {
            max_left_x = max_left_x.max(scan_range_arr[y][i].1);
            if max_left_x < scan_range_arr[y][i + 1].0 - 1 {
                println!(
                    "x: {:?}, y: {:?}, result: {:?}",
                    scan_range_arr[y][i].1,
                    y,
                    (scan_range_arr[y][i].1 + 1) * coord_max as i64 + y as i64
                );
            }
        }
    }
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

impl Sensor {
    fn scan_range_in_y(&self, y: i64) -> Option<(i64, i64)> {
        if distance(self.x, self.y, self.x, y) > self.closest_distance {
            None
        } else {
            let diff = self.closest_distance - (y - self.y).abs();
            Some((self.x - diff, self.x + diff))
        }
    }
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
