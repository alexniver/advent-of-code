use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day02.txt").unwrap();

    let lines = BufReader::new(f).lines();

    let mut score = 0;
    for line in lines {
        let bytes = line.unwrap().into_bytes();
        let oppent = bytes[0];
        let mine = bytes[2];
        let mine_shape = mine_shape(oppent, mine);

        let win_lose_score = win_lose_score(mine);
        let shape_score = shape_score(mine_shape);

        score += win_lose_score;
        score += shape_score;
    }

    println!("score : {}", score);
}

fn mine_shape(oppent: u8, mine: u8) -> u8 {
    match mine {
        b'X' => lose_shape(oppent), // lose
        b'Z' => win_shape(oppent),  // win
        _ => oppent,                // draw
    }
}

fn lose_shape(oppent: u8) -> u8 {
    match oppent {
        b'A' => b'C',
        b'B' => b'A',
        _ => b'B', // b'C'
    }
}

fn win_shape(oppent: u8) -> u8 {
    match oppent {
        b'A' => b'B',
        b'B' => b'C',
        _ => b'A', // b'C'
    }
}

fn win_lose_score(mine: u8) -> i32 {
    match mine {
        b'X' => 0,
        b'Y' => 3,
        b'Z' => 6,
        _ => 0,
    }
}

fn shape_score(mine: u8) -> i32 {
    match mine {
        b'A' => 1,
        b'B' => 2,
        b'C' => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{lose_shape, win_shape};

    #[test]
    fn win_shape_test() {
        assert_eq!(win_shape(b'A'), b'B');
        assert_eq!(win_shape(b'B'), b'C');
        assert_eq!(win_shape(b'C'), b'A');
    }

    #[test]
    fn lose_shape_test() {
        assert_eq!(lose_shape(b'A'), b'C');
        assert_eq!(lose_shape(b'B'), b'A');
        assert_eq!(lose_shape(b'C'), b'B');
    }
}
