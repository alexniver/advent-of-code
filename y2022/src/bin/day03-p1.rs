use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day03.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut error_item_arr = vec![];
    for line in lines {
        let bytes = line.unwrap().into_bytes();
        let mut error_item_arr_line = vec![];

        let len = bytes.len();
        for b1 in &bytes[0..len / 2] {
            for b2 in &bytes[len / 2..] {
                if b1 == b2 && !error_item_arr_line.contains(b1) {
                    error_item_arr_line.push(*b1);
                }
            }
        }

        error_item_arr.extend_from_slice(&error_item_arr_line);
    }

    let mut sum: u32 = 0;
    for b in error_item_arr {
        sum += get_priorities(b);
    }

    println!("sum : {}", sum);
}

fn get_priorities(input: u8) -> u32 {
    if is_lower_case(input) {
        (input - b'a' + 1) as u32
    } else {
        (input - b'A' + 27) as u32
    }
}

fn is_lower_case(input: u8) -> bool {
    b'a' <= input && b'z' >= input
}

#[cfg(test)]
mod tests {
    use crate::{get_priorities, is_lower_case};

    #[test]
    fn is_lower_case_test() {
        assert_eq!(is_lower_case(b'a'), true);
        assert_eq!(is_lower_case(b'z'), true);

        assert_eq!(is_lower_case(b'A'), false);
        assert_eq!(is_lower_case(b'Z'), false);
    }

    #[test]
    fn get_priorities_test() {
        assert_eq!(get_priorities(b'a'), 1);
        assert_eq!(get_priorities(b'z'), 26);

        assert_eq!(get_priorities(b'A'), 27);
        assert_eq!(get_priorities(b'Z'), 52);
    }
}
