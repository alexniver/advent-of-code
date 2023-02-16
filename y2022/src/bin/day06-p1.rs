use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day06.txt").unwrap();
    let mut lines = BufReader::new(f).lines();

    let bytes = lines.next().unwrap().unwrap().into_bytes();

    let value = get_marker(bytes);

    println!("value: {}", value);
}

const MARKER_LEN: usize = 4;
fn get_marker(bytes: Vec<u8>) -> i32 {
    for i in 0..(bytes.len() - MARKER_LEN) {
        let mut value = 0;
        let sub_arr = &bytes[i..(i + MARKER_LEN)];
        for tmp_x in sub_arr {
            for tmp_y in sub_arr {
                if tmp_x == tmp_y {
                    value += 1;
                }
            }
        }
        if value == MARKER_LEN {
            return (i + MARKER_LEN) as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::get_marker;

    #[test]
    fn get_marker_test() {
        assert_eq!(
            get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string().into_bytes()),
            5
        );

        assert_eq!(
            get_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string().into_bytes()),
            6
        );

        assert_eq!(
            get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string().into_bytes()),
            10
        );

        assert_eq!(
            get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string().into_bytes()),
            11
        );
    }
}
