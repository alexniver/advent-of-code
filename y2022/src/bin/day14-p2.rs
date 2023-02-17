use std::{collections::BTreeSet, fs};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let s = fs::read_to_string("input-day14.txt").unwrap();
    let set = parse(&s);
    process(set.unwrap().1);
}

fn process(mut set: BTreeSet<(u32, u32)>) {
    let sand_origin = (500, 0);

    let mut current = sand_origin;

    let rock_len = set.len();

    let mut rock_vec = set.iter().collect::<Vec<&(u32, u32)>>();
    rock_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = **rock_vec.last().unwrap();

    loop {
        if set.get(&sand_origin).is_some() {
            break;
        }
        let down = (current.0, current.1 + 1);
        let down_left = (current.0 - 1, current.1 + 1);
        let down_right = (current.0 + 1, current.1 + 1);

        match (
            set.get(&down).or_else(|| {
                if down.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            set.get(&down_left).or_else(|| {
                if down_left.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            set.get(&down_right).or_else(|| {
                if down_right.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (None, _, _) => current = down,
            (_, None, _) => current = down_left,
            (_, _, None) => current = down_right,
            (Some(_), Some(_), Some(_)) => {
                set.insert(current);
                current = sand_origin;
            }
        }
    }

    println!("value: {:?}", set.len() - rock_len);
}

fn parse(s: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (s, pairs) = separated_list1(line_ending, line)(s).unwrap();
    let set = pairs
        .into_iter()
        .flatten()
        .collect::<BTreeSet<(u32, u32)>>();
    Ok((s, set))
}

fn line(s: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (s, pairs) = separated_list1(tag(" -> "), pair)(s)?;
    let iter = pairs
        .into_iter()
        .tuple_windows()
        .flat_map(|((x1, y1), (x2, y2))| {
            let x_min = x1.min(x2);
            let y_min = y1.min(y2);

            let x_max = x1.max(x2);
            let y_max = y1.max(y2);

            (x_min..=x_max).cartesian_product(y_min..=y_max)
        });

    Ok((s, iter))
}

fn pair(s: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, tag(","), complete::u32)(s)
}

#[cfg(test)]
mod tests {
    use crate::{line, pair};

    #[test]
    fn pair_test() {
        let s = "12,21";
        assert_eq!(pair(s), Ok(("", (12, 21))));
    }

    #[test]
    fn line_test() {
        let s = "12,21 -> 12,22 -> 12,23";
        let mut iter = line(s).unwrap().1;
        assert_eq!(Some((12, 21)), iter.next());
        assert_eq!(Some((12, 22)), iter.next());
        assert_eq!(Some((12, 22)), iter.next());
        assert_eq!(Some((12, 23)), iter.next());
        assert_eq!(None, iter.next());
    }
}
