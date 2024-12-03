use std::{
    fs::File,
    io::{BufReader, Read},
};

use nom::{
    bytes::complete::{tag, take_until},
    sequence::{delimited, separated_pair},
    IResult,
};

// 定义一个解析整数的函数
fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("("),
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        tag(")"),
    )(input)
}

fn main() {
    let f = File::open("input-day03-p1.txt").unwrap();
    let mut input = String::new();
    let _ = BufReader::new(f).read_to_string(&mut input);

    let mut input = input.as_str();

    let mut sum = 0;
    loop {
        let mul_split: IResult<&str, &str> = take_until("mul")(input);

        if let Ok((remaining, _result)) = mul_split {
            let mul_tag: IResult<&str, &str> = tag("mul")(remaining);
            if let Ok((remaining, _result)) = mul_tag {
                if let Ok((remaining, nums)) = parse_mul(remaining) {
                    input = remaining;
                    sum += nums.0 * nums.1;
                } else {
                    input = remaining;
                }
            }
        } else {
            break;
        }
    }

    println!("sum: {}", sum);
}
