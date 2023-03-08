use std::{collections::VecDeque, fs, u64};

fn main() {
    let input = fs::read_to_string("input-day25.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

fn process(input: &str) -> String {
    let sum = parse(input).iter().sum();
    qui2sna(dec2qui(sum))
}

fn parse(input: &str) -> Vec<u64> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(qui2dec(sna2qui(line)));
    }
    result
}

fn dec2qui(mut dec: u64) -> u64 {
    let mut result = 0;
    loop {
        let mut len: u32 = 0;
        while dec >= u64::pow(5, len as u32) {
            len += 1;
        }

        if len == 0 {
            result += dec;
            break;
        } else {
            let p = u64::pow(5, len - 1);
            let v = dec / p;
            dec -= p * v;
            result += u64::pow(10, len - 1) * v;
        }
    }

    result
}

fn qui2dec(mut qui: u64) -> u64 {
    let mut result = 0;
    let mut len: u32 = 0;
    loop {
        result += (qui % 5) * u64::pow(5, len);
        qui /= 10;
        len += 1;
        if qui == 0 {
            break;
        }
    }
    result
}

fn qui2sna(mut qui: u64) -> String {
    let mut result = VecDeque::new();

    let mut len: u32 = 0;

    let mut is_carry = false;

    loop {
        let mut v = (qui % 5);
        if is_carry {
            v += 1;
        }

        match v {
            3 => {
                result.push_front(b'=');
                is_carry = true;
            }
            4 => {
                result.push_front(b'-');
                is_carry = true;
            }
            mut n => {
                if n >= 5 {
                    is_carry = true;
                    n %= 5;
                } else {
                    is_carry = false;
                }
                result.push_front((n + 48) as u8);
            }
            _ => {
                panic!("can't happen")
            }
        }

        qui /= 10;
        len += 1;
        if qui == 0 && !is_carry {
            break;
        }
    }
    String::from_utf8(result.into()).unwrap()
}

fn sna2qui(sna: &str) -> u64 {
    let mut result = 0;
    let mut len: u32 = 0;

    let mut is_borrow = false;
    loop {
        let b = sna.as_bytes()[sna.len() - 1 - len as usize];
        let p = u64::pow(10, len);
        match b {
            b'=' => {
                let n = if is_borrow { 2 } else { 3 };
                result += n * p;
                is_borrow = true;
            }
            b'-' => {
                let n = if is_borrow { 3 } else { 4 };
                result += n * p;
                is_borrow = true;
            }
            mut n => {
                if is_borrow {
                    if n == 48 {
                        n = 48 + 4;
                        is_borrow = true;
                    } else {
                        n -= 1;
                        is_borrow = false;
                    }
                }
                result += (n as u64 - 48) * p;
            }
        }
        len += 1;
        if len as usize == sna.len() {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{dec2qui, parse, process, qui2dec, qui2sna, sna2qui};

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn dec2qui_test() {
        assert_eq!(dec2qui(1), 1);
        assert_eq!(dec2qui(2), 2);
        assert_eq!(dec2qui(3), 3);
        assert_eq!(dec2qui(4), 4);
        assert_eq!(dec2qui(5), 10);
        assert_eq!(dec2qui(6), 11);
        assert_eq!(dec2qui(7), 12);
        assert_eq!(dec2qui(8), 13);
        assert_eq!(dec2qui(9), 14);
        assert_eq!(dec2qui(10), 20);
        assert_eq!(dec2qui(11), 21);
        assert_eq!(dec2qui(12), 22);
        assert_eq!(dec2qui(125), 1000);
        assert_eq!(dec2qui(126), 1001);
    }

    #[test]
    fn qui2dec_test() {
        assert_eq!(qui2dec(1), 1);
        assert_eq!(qui2dec(2), 2);
        assert_eq!(qui2dec(3), 3);
        assert_eq!(qui2dec(4), 4);
        assert_eq!(qui2dec(10), 5);
        assert_eq!(qui2dec(20), 10);
        assert_eq!(qui2dec(1000), 125);
        assert_eq!(qui2dec(1001), 126);
    }

    #[test]
    fn qui2sna_test() {
        assert_eq!(qui2sna(1), "1");
        assert_eq!(qui2sna(2), "2");
        assert_eq!(qui2sna(3), "1=");
        assert_eq!(qui2sna(4), "1-");
        assert_eq!(qui2sna(10), "10");
        assert_eq!(qui2sna(11), "11");
        assert_eq!(qui2sna(12), "12");
        assert_eq!(qui2sna(13), "2=");
        assert_eq!(qui2sna(14), "2-");
        assert_eq!(qui2sna(20), "20");
    }

    #[test]
    fn sna2qui_test() {
        assert_eq!(sna2qui("1"), 1);
        assert_eq!(sna2qui("2"), 2);
        assert_eq!(sna2qui("1="), 3);
        assert_eq!(sna2qui("1-"), 4);
        assert_eq!(sna2qui("10"), 10);
        assert_eq!(sna2qui("2="), 13);
        assert_eq!(sna2qui("2-"), 14);
        assert_eq!(sna2qui("20"), 20);
    }

    #[test]
    fn parse_test() {
        let arr = parse(INPUT);
        assert_eq!(arr.len(), 13);

        assert_eq!(arr[0], 1747);
        assert_eq!(arr[1], 906);
        assert_eq!(arr[2], 198);
        assert_eq!(arr[3], 11);
        assert_eq!(arr[4], 201);
        assert_eq!(arr[5], 31);
        assert_eq!(arr[6], 1257);
        assert_eq!(arr[7], 32);
        assert_eq!(arr[8], 353);
        assert_eq!(arr[9], 107);
        assert_eq!(arr[10], 7);
        assert_eq!(arr[11], 3);
        assert_eq!(arr[12], 37);
    }

    #[test]
    fn process_test() {
        let v = process(INPUT);
        assert_eq!(v, String::from("2=-1=0"));
    }
}
