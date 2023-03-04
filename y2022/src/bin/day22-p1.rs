use std::fs;

use nom::{branch::alt, character::complete, multi::many1, IResult, Parser};

fn main() {
    let input = fs::read_to_string("input-day22.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

struct Tile {
    t: TileType,
    _out_dir_arr: Vec<Dir>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Tile,
    Wall,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Password {
    Num(u32),
    RotLeft,
    RotRight,
}

const ALL_DIR: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn oppo(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    fn next_in_map(&self, x: usize, y: usize, map: &Vec<Vec<Tile>>) -> (usize, usize, bool) {
        let (tmp_x, tmp_y, mut is_out) = self.next(x, y, map[0].len() - 1, map.len() - 1);

        if !is_out {
            is_out = map[tmp_y][tmp_x].t == TileType::None;
        }
        if is_out {
            (x, y, is_out)
        } else {
            (tmp_x, tmp_y, is_out)
        }
    }

    fn next(&self, mut x: usize, mut y: usize, max_x: usize, max_y: usize) -> (usize, usize, bool) {
        let mut is_out = false;
        match self {
            Dir::Up => {
                if y == 0 {
                    is_out = true;
                } else {
                    y -= 1;
                }
            }
            Dir::Down => {
                if y == max_y {
                    is_out = true;
                } else {
                    y += 1;
                }
            }
            Dir::Left => {
                if x == 0 {
                    is_out = true;
                } else {
                    x -= 1;
                }
            }
            Dir::Right => {
                if x == max_x {
                    is_out = true;
                } else {
                    x += 1;
                }
            }
        }

        (x, y, is_out)
    }

    fn rot_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    fn rot_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn val(&self) -> usize {
        match self {
            Dir::Up => 3,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 0,
        }
    }
}

#[derive(Debug)]
struct Me {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Me {
    fn new(x: usize, y: usize) -> Self {
        Me {
            x,
            y,
            dir: Dir::Right,
        }
    }

    fn next(&mut self, map: &Vec<Vec<Tile>>) -> bool {
        let (x, y, is_out) = self.dir.next_in_map(self.x, self.y, map);

        if !is_out {
            match map[y][x].t {
                TileType::Tile => {
                    self.x = x;
                    self.y = y;
                }
                TileType::Wall => return false,
                _ => {}
            }
        } else {
            let oppo_dir = self.dir.oppo();
            let (mut tmp_x, mut tmp_y) = (x, y);
            loop {
                let (new_x, new_y, is_out) = oppo_dir.next_in_map(tmp_x, tmp_y, map);
                if !is_out {
                    (tmp_x, tmp_y) = (new_x, new_y);
                }

                if is_out {
                    if map[tmp_y][tmp_x].t == TileType::Tile {
                        self.x = tmp_x;
                        self.y = tmp_y;
                    }
                    break;
                }
            }
        }

        true
    }
}

fn process(input: &str) -> usize {
    let (map, password) = parse(input);

    let (start_y, start_x) = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, v)| (i, j, v)))
        .find(|(_, _, v)| v.t == TileType::Tile)
        .map(|(i, j, _)| (i, j))
        .unwrap();

    let mut me = Me::new(start_x, start_y);

    for p in password.iter() {
        match p {
            Password::Num(num) => {
                for _ in 0..*num {
                    if !me.next(&map) {
                        break;
                    }
                }
            }
            Password::RotLeft => me.dir = me.dir.rot_left(),
            Password::RotRight => me.dir = me.dir.rot_right(),
        }
    }

    println!("me: {:?}", me);

    1000 * (me.y + 1) + 4 * (me.x + 1) + me.dir.val()
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<Password>) {
    let mut split = input.split("\n\n");
    let map = split.next().unwrap();
    let pass = split.next().unwrap();

    let map = parse_map(map);
    let (max_x, max_y) = (map[0].len() - 1, map.len() - 1);
    let map = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, v)| {
                    let mut out_dir_arr = vec![];
                    for dir in ALL_DIR.iter() {
                        let (new_x, new_y, mut is_out) = dir.next(x, y, max_x, max_y);
                        if !is_out {
                            if map[new_y][new_x] == TileType::None {
                                is_out = true;
                            }
                        }
                        if is_out {
                            out_dir_arr.push(*dir);
                        }
                    }
                    Tile {
                        t: *v,
                        _out_dir_arr: out_dir_arr,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (map, parse_password(pass).unwrap().1)
}

fn parse_map(input: &str) -> Vec<Vec<TileType>> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len();

    let mut result = Vec::with_capacity(height);

    for line in lines {
        let mut row = vec![TileType::None; width];
        for (i, v) in line.bytes().enumerate() {
            match v {
                b'.' => row[i] = TileType::Tile,
                b'#' => row[i] = TileType::Wall,
                _ => {}
            }
        }
        result.push(row);
    }
    result
}

fn parse_password(input: &str) -> IResult<&str, Vec<Password>> {
    many1(alt((
        complete::u32.map(|num| Password::Num(num)),
        alt((
            complete::char('L').map(|_| Password::RotLeft),
            complete::char('R').map(|_| Password::RotRight),
        )),
    )))(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse_map, parse_password, process, Password};

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn parse_map_test() {
        let mut split = INPUT.split("\n\n");
        let map = parse_map(split.next().unwrap());

        assert_eq!(map.len(), 12);
        assert_eq!(map[0].len(), 16);
    }

    #[test]
    fn parse_password_test() {
        let mut split = INPUT.split("\n\n");
        split.next();

        let password = parse_password(split.next().unwrap()).unwrap().1;

        assert_eq!(password[0], Password::Num(10));
        assert_eq!(password[1], Password::RotRight);
        assert_eq!(password[2], Password::Num(5));
        assert_eq!(password[3], Password::RotLeft);
    }

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 6032);
    }
}
