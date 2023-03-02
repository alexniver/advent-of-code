use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};

fn main() {
    let input = fs::read_to_string("input-day21.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

fn process(input: &str) -> isize {
    let map = parse(input).unwrap().1;
    calculate("root", &map)
}

fn calculate<'a>(name: &'a str, map: &HashMap<&str, Job<'a>>) -> isize {
    match map.get(name).unwrap() {
        Job::Opt(left, right, opt) => match opt {
            Math::Add => calculate(left, map) + calculate(right, map),
            Math::Sub => calculate(left, map) - calculate(right, map),
            Math::Multi => calculate(left, map) * calculate(right, map),
            Math::Div => calculate(left, map) / calculate(right, map),
        },
        Job::Num(v) => *v as isize,
    }
}

fn parse<'a>(input: &'a str) -> IResult<&str, HashMap<&str, Job<'a>>> {
    let mut result = HashMap::new();

    let (input, list) = separated_list1(line_ending, tuple((alpha1, tag(": "), parse_job)))(input)?;
    for item in list {
        result.insert(item.0, item.2);
    }

    Ok((input, result))
}

fn parse_job<'a>(input: &'a str) -> IResult<&str, Job<'a>> {
    alt((parse_opt, parse_num))(input)
}

fn parse_num<'a>(input: &str) -> IResult<&str, Job<'a>> {
    let (input, num) = complete::i32(input)?;
    Ok((input, Job::Num(num)))
}

fn parse_opt<'a>(input: &'a str) -> IResult<&str, Job<'a>> {
    let (input, left) = alpha1(input)?;
    let (input, opt) = delimited(
        tag(" "),
        one_of("+-*/").map(|opt| match opt {
            '+' => Math::Add,
            '-' => Math::Sub,
            '*' => Math::Multi,
            '/' => Math::Div,
            _ => panic!(""),
        }),
        tag(" "),
    )(input)?;
    let (input, right) = alpha1(input)?;

    Ok((input, Job::Opt(left, right, opt)))
}

enum Job<'a> {
    Opt(&'a str, &'a str, Math),
    Num(i32),
}

enum Math {
    Add,
    Sub,
    Multi,
    Div,
}

#[cfg(test)]
mod tests {
    use crate::{parse, process};

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn parse_test() {
        assert_eq!(parse(INPUT).unwrap().1.len(), 15);
    }
    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 152);
    }
}
