use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input-day18.txt").unwrap();
    let v = process(&input);
    println!("v: {}", v);
}

fn process(input: &str) -> usize {
    let point_list = separated_list1(line_ending, parse_point)(input).unwrap().1;
    let dirs = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut faces = 0;

    for Point { x, y, z } in point_list.iter() {
        for (dx, dy, dz) in dirs.iter() {
            let p = Point {
                x: x + dx,
                y: y + dy,
                z: z + dz,
            };
            if point_list.iter().find(|tmp_p| **tmp_p == p).is_none() {
                faces += 1;
            }
        }
    }
    faces
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, v) = separated_list1(tag(","), complete::i32)(input)?;
    Ok((
        input,
        Point {
            x: v[0],
            y: v[1],
            z: v[2],
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg(test)]
mod tests {
    use crate::process;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 64);
    }
}
