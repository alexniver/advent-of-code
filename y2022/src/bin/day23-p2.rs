use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input-day23.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

const DIR_N: [Pos; 3] = [
    Pos { x: -1, y: -1 },
    Pos { x: 0, y: -1 },
    Pos { x: 1, y: -1 },
];
const DIR_S: [Pos; 3] = [Pos { x: -1, y: 1 }, Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }];
const DIR_W: [Pos; 3] = [
    Pos { x: -1, y: -1 },
    Pos { x: -1, y: 0 },
    Pos { x: -1, y: 1 },
];
const DIR_E: [Pos; 3] = [Pos { x: 1, y: -1 }, Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }];

fn process(input: &str) -> isize {
    let mut dirs_arr = [DIR_N, DIR_S, DIR_W, DIR_E];
    let mut tile_set = parse_tile_map(input);

    let mut round = 0;
    loop {
        let mut move_arr = vec![];
        for v in tile_set.iter() {
            let mut all_dir_empty = true;
            for dir in dirs_arr.iter().flatten() {
                let pos_dir = Pos::new(v.x + dir.x, v.y + dir.y);
                if tile_set.contains(&pos_dir) {
                    all_dir_empty = false;
                    break;
                }
            }

            // move
            if !all_dir_empty {
                for dirs in dirs_arr.iter() {
                    let mut is_empty = true;
                    for dir in dirs.iter() {
                        let pos_dir = Pos::new(v.x + dir.x, v.y + dir.y);
                        if tile_set.contains(&pos_dir) {
                            is_empty = false;
                            break;
                        }
                    }

                    // move to this dir
                    if is_empty {
                        let dir = &dirs[1];
                        move_arr.push(Move::new(
                            Pos::new(v.x, v.y),
                            Pos::new(v.x + dir.x, v.y + dir.y),
                        ));
                        break;
                    }
                }
            }
        }

        let mut new_pos_arr = vec![];
        let mut confict_idx_arr = vec![];
        for k in 0..move_arr.len() {
            let f = new_pos_arr.iter().find(|(_, pos)| *pos == move_arr[k].new);
            if f.is_some() {
                let f = &f.unwrap().0;
                if !confict_idx_arr.contains(f) {
                    confict_idx_arr.push(*f);
                }
                confict_idx_arr.push(k);
            }
            new_pos_arr.push((k, move_arr[k].new));
        }

        for (idx, m) in move_arr.iter().enumerate() {
            if !confict_idx_arr.contains(&idx) {
                tile_set.remove(&m.old);
                tile_set.insert(m.new);
            }
        }

        let tmp = dirs_arr[0];
        for k in 0..dirs_arr.len() - 1 {
            dirs_arr[k] = dirs_arr[k + 1];
        }
        dirs_arr[dirs_arr.len() - 1] = tmp;

        // print_tile_map(&tile_map);
        round += 1;
        if move_arr.is_empty() {
            break;
        }
    }

    round
}

fn parse_tile_map(input: &str) -> HashSet<Pos> {
    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.bytes().enumerate() {
            match v {
                b'#' => {
                    result.insert(Pos::new(x as isize, y as isize));
                }
                _ => {}
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::process;

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    //     const INPUT: &str = ".....
    // ..##.
    // ..#..
    // .....
    // ..##.
    // .....";

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 20);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct Move {
    old: Pos,
    new: Pos,
}

impl Move {
    fn new(old: Pos, new: Pos) -> Self {
        Move { old, new }
    }
}
