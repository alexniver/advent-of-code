use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn main() {
    let f = fs::read_to_string("input-day13.txt").unwrap();
    let s = process(&f);
    println!("score: {s}");
}

struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Num(b)) => a.cmp(&vec![Packet::Num(*b)]),
            (Packet::Num(a), Packet::List(b)) => vec![Packet::Num(*a)].cmp(b),
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process(input: &str) -> usize {
    if let Ok((_, pair_arr)) = pair(input) {
        pair_arr
            .iter()
            .enumerate()
            .filter_map(|(i, p)| match p.left.cmp(&p.right) {
                std::cmp::Ordering::Less => Some(i + 1),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => None,
            })
            .sum::<usize>()
    } else {
        panic!("en?")
    }
}

fn pair(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(left, right)| Pair { left, right }),
    )(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(|v| Packet::List(v)),
        nom::character::complete::u32.map(|n| Packet::Num(n)),
    ))(input)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::{packet, Packet};

    #[test]
    fn packet_test() {
        assert_eq!(
            packet("[1,2,3]"),
            Ok((
                "",
                Packet::List(vec![Packet::Num(1), Packet::Num(2), Packet::Num(3)])
            ))
        );

        assert_eq!(
            packet("[[1]]"),
            Ok(("", Packet::List(vec![Packet::List(vec![Packet::Num(1)])])))
        );
    }

    #[test]
    fn packet_cmp_test() {
        assert_eq!(
            Packet::List(vec![Packet::Num(1)]).cmp(&Packet::List(vec![Packet::Num(2)])),
            Ordering::Less
        );

        assert_eq!(
            Packet::List(vec![Packet::List(vec![]), Packet::List(vec![])])
                .cmp(&Packet::List(vec![Packet::List(vec![])])),
            Ordering::Greater
        );

        let mut v1: Vec<Vec<u32>> = Vec::new();
        v1.push(vec![]);
        v1.push(vec![]);
        let mut v2: Vec<Vec<u32>> = Vec::new();
        v2.push(vec![]);

        assert_eq!(v1.cmp(&v2), Ordering::Greater);
    }
}
