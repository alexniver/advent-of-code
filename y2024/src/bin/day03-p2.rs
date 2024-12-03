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
    let f = File::open("input-day03-p2.txt").unwrap();
    let mut input = String::new();
    let _ = BufReader::new(f).read_to_string(&mut input);

    let mut input = input.as_str();

    // let mut input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // let mut input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mut sum = 0;

    let mut is_mul = true;
    loop {
        let mul_split: IResult<&str, &str> = take_until("mul")(input);

        if let Ok((remaining, result)) = mul_split {
            if result.contains("don't") {
                is_mul = false;
            } else if result.contains("do") {
                is_mul = true;
            }
            let mul_tag: IResult<&str, &str> = tag("mul")(remaining);
            if let Ok((remaining, _result)) = mul_tag {
                if let Ok((remaining, nums)) = parse_mul(remaining) {
                    input = remaining;
                    if is_mul {
                        sum += nums.0 * nums.1;
                    }
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
