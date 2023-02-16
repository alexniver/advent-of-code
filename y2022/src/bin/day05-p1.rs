use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day05.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut stack_line_arr = vec![];
    let mut move_line_arr = vec![];

    let mut stack_inited = false;

    for line in lines {
        let bytes = line.unwrap();
        if bytes.is_empty() {
            stack_inited = true;
            continue;
        }

        if !stack_inited {
            stack_line_arr.push(bytes);
        } else {
            move_line_arr.push(bytes);
        }
    }

    println!(
        "stack len: {}, move len: {}",
        stack_line_arr.len(),
        move_line_arr.len()
    );

    let stack_arr = build_stack_arr(stack_line_arr);

    let move_arr = build_move_arr(move_line_arr);

    let stack_arr = apply_move(stack_arr, move_arr);

    let result = stack_arr
        .iter()
        .map(|u| (u.last().unwrap()).clone())
        .collect::<Vec<u8>>();
    println!("result : {:?}", String::from_utf8(result));
    // for item in stack_arr {
    //     println!(
    //         "item : {:?}",
    //         item.iter()
    //             .map(|&a| char::from_u32(a as u32).unwrap())
    //             .collect::<Vec<char>>()
    //     );
    // }
}

#[derive(Debug, PartialEq)]
struct Move {
    num: u32,
    from: u32,
    to: u32,
}

fn build_stack_arr(stack_line_arr: Vec<String>) -> Vec<Vec<u8>> {
    let stack_num = 1 + (stack_line_arr[0].len() - 3) / 4;
    let mut stack_arr = Vec::with_capacity(stack_num);
    for _ in 0..stack_num {
        stack_arr.push(vec![]);
    }
    for i in (0..stack_line_arr.len() - 1).rev() {
        let bytes = stack_line_arr[i].as_bytes();
        if bytes[1] != b' ' {
            stack_arr[0].push(bytes[1]);
        }
        for j in 1..stack_num {
            let b = bytes[1 + 4 * j];
            if b != b' ' {
                stack_arr[j].push(b);
            }
        }
    }

    stack_arr
}

fn build_move_arr(move_line_arr: Vec<String>) -> Vec<Move> {
    let mut result = Vec::with_capacity(move_line_arr.len());

    for line in move_line_arr {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let m = Move {
            num: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        };
        result.push(m);
    }

    result
}

fn apply_move(mut stack_arr: Vec<Vec<u8>>, move_arr: Vec<Move>) -> Vec<Vec<u8>> {
    for m in move_arr {
        for _ in 0..m.num {
            let pop = stack_arr[(m.from - 1) as usize].pop().unwrap();
            stack_arr[(m.to - 1) as usize].push(pop);
        }
    }
    stack_arr
}

#[cfg(test)]
mod tests {

    use crate::{build_move_arr, Move};

    #[test]
    fn build_move_arr_test() {
        let move_line_arr = vec![
            String::from("move 8 from 7 to 1"),
            String::from("move 9 from 1 to 9"),
        ];
        let move_arr = build_move_arr(move_line_arr);
        assert_eq!(
            move_arr[0],
            Move {
                num: 8,
                from: 7,
                to: 1
            }
        );
    }
}
