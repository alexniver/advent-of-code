use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day03.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut common_item_arr = vec![];
    let mut group_item_arr = vec![];
    let mut seq = 0;
    for line in lines {
        let bytes = line.unwrap().into_bytes();
        if seq % 3 == 0 {
            group_item_arr = bytes;
        } else {
            group_item_arr = group_item_arr
                .into_iter()
                .filter(|b| bytes.contains(b))
                .collect();
        }

        if (seq + 1) % 3 == 0 {
            group_item_arr.dedup();
            common_item_arr.extend_from_slice(&group_item_arr);
        }

        seq += 1;
    }

    let mut sum: u32 = 0;
    for b in common_item_arr {
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
