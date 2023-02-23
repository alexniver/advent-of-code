use std::{
    collections::{HashSet, VecDeque},
    fs,
};

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
    let mut faces = 0;

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;

    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;

    for p in point_list.iter() {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        min_z = min_z.min(p.z);

        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
        max_z = max_z.max(p.z);
    }

    min_x -= 1;
    min_y -= 1;
    min_z -= 1;

    max_x += 1;
    max_y += 1;
    max_z += 1;
    let mut queue = VecDeque::new();
    queue.push_back(Point::new(min_x, min_y, min_z));

    let dirs = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let tmp_p = queue.pop_front().unwrap();
        if visited.contains(&tmp_p) {
            continue;
        }

        if point_list.contains(&tmp_p) {
            faces += 1;
            continue;
        }

        for &dir in dirs.iter() {
            let neighbour = Point::new(tmp_p.x + dir.0, tmp_p.y + dir.1, tmp_p.z + dir.2);
            if neighbour.x >= min_x
                && neighbour.x <= max_x
                && neighbour.y >= min_y
                && neighbour.y <= max_y
                && neighbour.z >= min_z
                && neighbour.z <= max_z
            {
                queue.push_back(neighbour);
            }
        }

        visited.insert(tmp_p);
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
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
        assert_eq!(process(INPUT), 58);
    }
}
