use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input-day24.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

fn process(input: &str) -> usize {
    let (entry, exit, storms, max_x, max_y) = parse(input);
    let map = gen_map(storms, max_x, max_y);

    let mut result = usize::MAX;

    let state = State::new(entry, 0, 0);

    let mut set: HashSet<(usize, Pos)> = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(state);
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        if set.contains(&(state.z, state.pos)) {
            continue;
        }
        set.insert((state.z, state.pos));

        if state.pos.x == exit.x && state.pos.y + 1 == exit.y {
            result = result.min(state.time + 1);
        }

        let z = (state.z + 1) % map.len();

        for dir in DIRS.iter() {
            if let Some(pos) = state.pos.try_walk(dir, max_x, max_y) {
                if pos == entry || map[z][pos.y - 1][pos.x - 1] {
                    queue.push_back(State::new(pos, state.time + 1, z))
                }
            }
        }
        if state.pos == entry || map[z][state.pos.y - 1][state.pos.x - 1] {
            queue.push_back(State::new(state.pos.clone(), state.time + 1, z))
        }
    }

    result
}

// true can walk, false is storm
fn gen_map(mut storms: Vec<Storm>, max_x: usize, max_y: usize) -> Vec<Vec<Vec<bool>>> {
    // gcd
    let mut a = (max_x - 1).min(max_y - 1);
    let mut b = (max_x - 1).max(max_y - 1);
    let mut m = b % a;
    while m != 0 {
        b = a;
        a = m;
        m = b % a;
    }
    // lcm
    let len = (max_x - 1) * (max_y - 1) / a;

    let mut result = vec![vec![vec![true; max_x - 1]; max_y - 1]; len];
    for z in 0..len {
        for y in 0..max_y - 1 {
            for x in 0..max_x - 1 {
                if storms
                    .iter()
                    .find(|s| s.pos.x - 1 == x && s.pos.y - 1 == y)
                    .is_some()
                {
                    result[z][y][x] = false;
                }
            }
        }

        // storm move
        for storm in storms.iter_mut() {
            match storm.dir {
                Dir::Left => {
                    if storm.pos.x == 1 {
                        storm.pos.x = max_x - 1;
                    } else {
                        storm.pos.x -= 1;
                    }
                }
                Dir::Down => {
                    if storm.pos.y == max_y - 1 {
                        storm.pos.y = 1;
                    } else {
                        storm.pos.y += 1;
                    }
                }
                Dir::Right => {
                    if storm.pos.x == max_x - 1 {
                        storm.pos.x = 1;
                    } else {
                        storm.pos.x += 1;
                    }
                }
                Dir::Up => {
                    if storm.pos.y == 1 {
                        storm.pos.y = max_y - 1;
                    } else {
                        storm.pos.y -= 1;
                    }
                }
            }
        }
    }

    result
}

// entry, exit, storms, max_x, max_y
fn parse(input: &str) -> (Pos, Pos, Vec<Storm>, usize, usize) {
    let mut entry = Pos::new(0, 0);
    let mut exit = Pos::new(0, 0);
    let mut storms = vec![];

    let lines = input.lines().collect::<Vec<_>>();
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;

    for (y, l) in lines.iter().enumerate() {
        for (x, v) in l.bytes().enumerate() {
            match v {
                b'.' => {
                    if y == 0 {
                        entry.x = x;
                        entry.y = 0;
                    } else if y == max_y {
                        exit.x = x;
                        exit.y = max_y;
                    }
                }
                b'<' => storms.push(Storm::new(Pos::new(x, y), Dir::Left)),
                b'v' => storms.push(Storm::new(Pos::new(x, y), Dir::Down)),
                b'>' => storms.push(Storm::new(Pos::new(x, y), Dir::Right)),
                b'^' => storms.push(Storm::new(Pos::new(x, y), Dir::Up)),
                b'#' => {}
                _ => {
                    panic!("can't happen")
                }
            }
        }
    }

    (entry, exit, storms, max_x, max_y)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Storm {
    pos: Pos,
    dir: Dir,
}

impl Storm {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn try_walk(&self, dir: &Dir, max_x: usize, max_y: usize) -> Option<Pos> {
        let mut result = self.clone();

        match dir {
            Dir::Left => {
                if result.x <= 1 {
                    return None;
                }
                result.x -= 1;
            }
            Dir::Down => {
                if result.y >= max_y - 1 {
                    return None;
                }
                result.y += 1;
            }
            Dir::Right => {
                if result.x >= max_x - 1 {
                    return None;
                }
                result.x += 1;
            }
            Dir::Up => {
                if result.y <= 1 {
                    return None;
                }
                result.y -= 1;
            }
        }

        if result.x == 0 || result.y == 0 || result.x == max_x || result.y == max_y {
            None
        } else {
            Some(result)
        }
    }
}

const DIRS: [Dir; 4] = [Dir::Left, Dir::Down, Dir::Right, Dir::Up];
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Down,
    Right,
    Up,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    z: usize,
    time: usize,
    pos: Pos,
}
impl State {
    fn new(pos: Pos, time: usize, z: usize) -> Self {
        Self { pos, time, z }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::{gen_map, parse, process, Pos};

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn parse_test() {
        let (entry, exit, storms, max_x, max_y) = parse(INPUT);
        assert_eq!(entry, Pos::new(1, 0));
        assert_eq!(exit, Pos::new(6, 5));
        assert_eq!(storms.len(), 19);
        assert_eq!(max_x, 7);
        assert_eq!(max_y, 5);
    }

    #[test]
    fn gen_map_test() {
        let (entry, exit, storms, max_x, max_y) = parse(INPUT);

        assert_eq!(max_x, 7);
        assert_eq!(max_y, 5);

        let map = gen_map(storms, max_x, max_y);

        assert_eq!(entry, Pos::new(1, 0));
        assert_eq!(exit, Pos::new(6, 5));

        assert_eq!(map.len(), 12);
        assert_eq!(map[0][0][0], false);
        assert_eq!(map[1][0][0], true);
        assert_eq!(map[2][0][0], true);
        assert_eq!(map[3][0][0], false);
        assert_eq!(map[4][0][0], true);
        assert_eq!(map[5][0][0], false);

        assert_eq!(map[0][3][5], false);
        assert_eq!(map[1][3][5], false);
        assert_eq!(map[2][3][5], true);
        assert_eq!(map[3][3][5], true);
        assert_eq!(map[4][3][5], true);
        assert_eq!(map[5][3][5], true);
    }

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 18);
    }
}
