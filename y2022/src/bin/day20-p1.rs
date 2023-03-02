use std::fs;

use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input-day20.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

fn process(input: &str) -> i32 {
    let num_arr_origin = parse(input).unwrap().1;
    let len = num_arr_origin.len() as i32;
    let mut num_idx_arr = Vec::with_capacity(len as usize);
    for i in 0..len {
        num_idx_arr.push(i);
    }

    for i in 0..len {
        let idx = num_idx_arr.iter().position(|idx| *idx == i).unwrap() as i32;
        let m = num_arr_origin[i as usize];

        let new_idx = (idx + m).rem_euclid(len - 1);

        if idx == new_idx {
            continue;
        }

        let v = num_idx_arr.remove(idx as usize);
        num_idx_arr.insert(new_idx as usize, v);
    }

    let zero_idx = num_idx_arr
        .iter()
        .position(|idx| num_arr_origin[*idx as usize] == 0)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|v| num_arr_origin[num_idx_arr[(zero_idx as usize + v) % len as usize] as usize])
        .sum::<i32>()
}

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(line_ending, complete::i32)(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse, process};

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn parse_test() {
        assert_eq!(parse(INPUT), Ok(("", vec![1, 2, -3, 3, -2, 0, 4])));
    }

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 3);
    }
}
