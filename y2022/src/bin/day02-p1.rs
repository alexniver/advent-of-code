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

        let win_lose_score = win_lose_score(oppent, mine);
        let shape_score = shape_score(mine);

        score += win_lose_score;
        score += shape_score;
    }

    println!("score : {}", score);
}

const DIFF: u8 = b'X' - b'A';
fn win_lose_score(opponent: u8, mine: u8) -> i32 {
    let mine = mine - DIFF;
    if opponent == mine {
        // draw
        return 3;
    } else {
        if mine == b'C' && opponent == b'A' {
            // lose
            return 0;
        }
        if (mine == b'A' && opponent == b'C') || (mine > opponent) {
            // win
            return 6;
        }
    }
    return 0; // lose
}

fn shape_score(mine: u8) -> i32 {
    match mine {
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::win_lose_score;

    #[test]
    fn test_win_lose_score() {
        assert_eq!(win_lose_score(b'B', b'X'), 0);
        assert_eq!(win_lose_score(b'C', b'X'), 6);
        assert_eq!(win_lose_score(b'A', b'X'), 3);

        assert_eq!(win_lose_score(b'B', b'Y'), 3);
        assert_eq!(win_lose_score(b'C', b'Y'), 0);
        assert_eq!(win_lose_score(b'A', b'Y'), 6);

        assert_eq!(win_lose_score(b'B', b'Z'), 6);
        assert_eq!(win_lose_score(b'C', b'Z'), 3);
        assert_eq!(win_lose_score(b'A', b'Z'), 0);
    }
}
