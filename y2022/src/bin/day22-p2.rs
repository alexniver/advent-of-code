use std::{collections::HashMap, fs};

use nom::{branch::alt, character::complete, multi::many1, IResult, Parser};

fn main() {
    let input = fs::read_to_string("input-day22.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

#[derive(Debug)]
struct Tile {
    t: TileType,
    out_dir_arr: Vec<Dir>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn next_in_tile(&self, x: usize, y: usize, map: &Vec<Vec<Tile>>) -> (usize, usize, bool) {
        let (tmp_x, tmp_y, mut is_out) = self.next_in_map(x, y, map);

        if !is_out {
            is_out = map[tmp_y][tmp_x].t == TileType::None;
        }
        if is_out {
            (x, y, is_out)
        } else {
            (tmp_x, tmp_y, is_out)
        }
    }

    fn next_in_map(&self, x: usize, y: usize, map: &Vec<Vec<Tile>>) -> (usize, usize, bool) {
        self.next(x, y, map[0].len() - 1, map.len() - 1)
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Relation {
    x: usize,
    y: usize,
    dir_when_edge: Dir,
}

impl Relation {
    fn new(x: usize, y: usize, dir_when_edge: Dir) -> Self {
        Self {
            x,
            y,
            dir_when_edge,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Corner {
    x: usize,
    y: usize,
    dir0: Dir,
    dir1: Dir,
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

    fn next(
        &mut self,
        map: &Vec<Vec<Tile>>,
        relation: &HashMap<Dir, HashMap<Point, Relation>>,
    ) -> bool {
        let (x, y, is_out) = self.dir.next_in_tile(self.x, self.y, map);

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
            let new_point = relation
                .get(&self.dir)
                .unwrap()
                .get(&Point::new(self.x, self.y))
                .unwrap();
            match map[new_point.y][new_point.x].t {
                TileType::Tile => {
                    self.x = new_point.x;
                    self.y = new_point.y;

                    if map[new_point.y][new_point.x].out_dir_arr[0] == new_point.dir_when_edge {
                        self.dir = map[new_point.y][new_point.x].out_dir_arr[1].oppo();
                    } else {
                        self.dir = map[new_point.y][new_point.x].out_dir_arr[0].oppo();
                    }
                }
                TileType::Wall => return false,
                TileType::None => {
                    panic!("can't happen")
                }
            }
        }

        true
    }
}

fn process(input: &str) -> usize {
    let (map, password) = parse(input);

    let (start_x, start_y) = get_start(&map);

    let mut me = Me::new(start_x, start_y);

    let relation = get_edge_relation(&map);

    for p in password.iter() {
        match p {
            Password::Num(num) => {
                for _ in 0..*num {
                    if !me.next(&map, &relation) {
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

fn get_edge_relation(map: &Vec<Vec<Tile>>) -> HashMap<Dir, HashMap<Point, Relation>> {
    let mut result = HashMap::new();

    let corner_arr = get_corner_arr(map);
    for corner in corner_arr {
        let (x0, y0, _) = corner.dir0.next_in_map(corner.x, corner.y, map);
        let (x1, y1, _) = corner.dir1.next_in_map(corner.x, corner.y, map);

        let mut dir0 = corner.dir0;
        let mut dir1 = corner.dir1;

        let mut edge0 = get_edge(x0, y0, dir0, map);
        let mut edge1 = get_edge(x1, y1, dir1, map);
        let mut idx = 0;

        loop {
            // create relation
            let min_len = edge0.len().min(edge1.len());
            for (i, p0) in edge0[idx..min_len].iter().enumerate() {
                let p1 = &edge1[idx + i];

                let t0 = &map[p0.y][p0.x];
                let out_dir0 = if t0.out_dir_arr[0] == dir0 {
                    t0.out_dir_arr[1]
                } else {
                    t0.out_dir_arr[0]
                };

                let t1 = &map[p1.y][p1.x];
                let out_dir1 = if t1.out_dir_arr[0] == dir1 {
                    t1.out_dir_arr[1]
                } else {
                    t1.out_dir_arr[0]
                };

                if result.get(&out_dir0).is_none() {
                    result.insert(out_dir0, HashMap::new());
                }
                if result.get(&out_dir1).is_none() {
                    result.insert(out_dir1, HashMap::new());
                }

                let out_dir_map0 = result.get_mut(&out_dir0).unwrap();
                out_dir_map0.insert(*p0, Relation::new(p1.x, p1.y, dir1));

                let out_dir_map1 = result.get_mut(&out_dir1).unwrap();
                out_dir_map1.insert(*p1, Relation::new(p0.x, p0.y, dir0));
            }
            idx += min_len - idx;

            if edge0.len() == edge1.len() {
                break;
            }

            let last0 = edge0.last().unwrap();
            let last1 = edge1.last().unwrap();

            let tile0 = &map[last0.y][last0.x];
            let tile1 = &map[last1.y][last1.x];

            if edge0.len() < edge1.len() {
                dir0 = if tile0.out_dir_arr[0] == dir0 {
                    tile0.out_dir_arr[1].oppo()
                } else {
                    tile0.out_dir_arr[0].oppo()
                };
                edge0.extend(get_edge(last0.x, last0.y, dir0, map));
            } else if edge0.len() > edge1.len() {
                dir1 = if tile1.out_dir_arr[0] == dir1 {
                    tile1.out_dir_arr[1].oppo()
                } else {
                    tile1.out_dir_arr[0].oppo()
                };
                edge1.extend(get_edge(last1.x, last1.y, dir1, map));
            }
        }

        // for (i, p0) in edge0.iter().enumerate() {
        //     let p1 = &edge1[i];
        //     for out_dir0 in &map[p0.y][p0.x].out_dir_arr {
        //         if result.get(out_dir0).is_none() {
        //             result.insert(*out_dir0, HashMap::new());
        //         }
        //         let relation = result.get_mut(out_dir0).unwrap();
        //         relation.insert(*p0, *p1);
        //     }
        // }
    }

    result
}

fn get_start(map: &Vec<Vec<Tile>>) -> (usize, usize) {
    map.iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, v)| (i, j, v)))
        .find(|(_, _, v)| v.t == TileType::Tile)
        .map(|(i, j, _)| (j, i))
        .unwrap()
}

fn get_edge(x: usize, y: usize, dir: Dir, map: &Vec<Vec<Tile>>) -> Vec<Point> {
    let mut result = vec![];

    let (mut tmp_x, mut tmp_y) = (x, y);
    loop {
        result.push(Point { x: tmp_x, y: tmp_y });
        let (new_x, new_y, is_out) = dir.next_in_map(tmp_x, tmp_y, &map);
        if is_out || map[new_y][new_x].out_dir_arr.is_empty() {
            break;
        }
        (tmp_x, tmp_y) = (new_x, new_y);
    }

    result
}

fn get_corner_arr(map: &Vec<Vec<Tile>>) -> Vec<Corner> {
    let mut result = vec![];

    let (start_x, start_y) = get_start(map);
    let (mut x, mut y) = (start_x, start_y);

    let tile = &map[y][x];
    let mut dir = tile.out_dir_arr[0].oppo();

    loop {
        let Point { x: tmp_x, y: tmp_y } = get_edge(x, y, dir, map).into_iter().last().unwrap();
        if tmp_x == start_x && tmp_y == start_y {
            break;
        }

        let tmp_tile = &map[tmp_y][tmp_x];

        // out_dir_arr len is 1 for useful corner, len 2 for useless corner
        if tmp_tile.out_dir_arr.len() == 1 {
            let (new_x, new_y, _) = dir.next_in_map(tmp_x, tmp_y, map);
            result.push(Corner {
                x: new_x,
                y: new_y,
                dir0: dir.oppo(),
                dir1: tmp_tile.out_dir_arr[0],
            });

            // rotate
            dir = tmp_tile.out_dir_arr[0];

            let (new_x, new_y, _) = dir.next_in_tile(new_x, new_y, map);
            (x, y) = (new_x, new_y);
        } else if tmp_tile.out_dir_arr.len() == 2 {
            (x, y) = (tmp_x, tmp_y);
            // rotate, not change position
            for tmp_dir in tmp_tile.out_dir_arr.iter() {
                if dir != *tmp_dir {
                    dir = tmp_dir.oppo();
                    break;
                }
            }
        }
    }

    result
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
                        if is_out && map[y][x] != TileType::None {
                            out_dir_arr.push(*dir);
                        }
                    }
                    Tile { t: *v, out_dir_arr }
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
    use crate::{
        get_corner_arr, get_edge, get_edge_relation, get_start, parse, parse_map, parse_password,
        process, Corner, Dir, Password, Point, Relation,
    };

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
    fn parse_test() {
        let (map, password) = parse(INPUT);

        let (x, y) = get_start(&map);
        assert_eq!(map[y][x].out_dir_arr, vec![Dir::Up, Dir::Left]);
        assert_eq!(map[4][8].out_dir_arr, vec![]);

        assert_eq!(password.len(), 13);
        assert_eq!(password[0], Password::Num(10));
        assert_eq!(password[1], Password::RotRight);
        assert_eq!(password[2], Password::Num(5));
        assert_eq!(password[3], Password::RotLeft);
    }

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
    fn get_start_test() {
        let map = parse(INPUT).0;
        let start = get_start(&map);
        assert_eq!(start, (8, 0));
    }

    #[test]
    fn get_edge_test() {
        let map = parse(INPUT).0;

        let start = get_start(&map);

        let edge = get_edge(start.0, start.1, Dir::Down, &map);
        assert_eq!(edge.len(), 4);
        assert_eq!(edge[0], Point { x: 8, y: 0 });
        assert_eq!(edge[1], Point { x: 8, y: 1 });
        assert_eq!(edge[2], Point { x: 8, y: 2 });
        assert_eq!(edge[3], Point { x: 8, y: 3 });

        let start = (7, 4);
        let edge = get_edge(start.0, start.1, Dir::Left, &map);
        assert_eq!(edge.len(), 8);
        assert_eq!(edge[0], Point::new(7, 4));
        assert_eq!(edge[1], Point::new(6, 4));
        assert_eq!(edge[2], Point::new(5, 4));
        assert_eq!(edge[3], Point::new(4, 4));
        assert_eq!(edge[4], Point::new(3, 4));
        assert_eq!(edge[5], Point::new(2, 4));
        assert_eq!(edge[6], Point::new(1, 4));
        assert_eq!(edge[7], Point::new(0, 4));
    }

    #[test]
    fn get_corner_test() {
        let map = parse(INPUT).0;
        let corner_arr = get_corner_arr(&map);
        assert_eq!(corner_arr.len(), 3);
        assert_eq!(
            corner_arr[0],
            Corner {
                x: 8,
                y: 4,
                dir0: Dir::Up,
                dir1: Dir::Left
            }
        );

        assert_eq!(
            corner_arr[1],
            Corner {
                x: 8,
                y: 7,
                dir0: Dir::Left,
                dir1: Dir::Down,
            }
        );

        assert_eq!(
            corner_arr[2],
            Corner {
                x: 11,
                y: 8,
                dir0: Dir::Right,
                dir1: Dir::Up,
            }
        );
    }

    #[test]
    fn get_edge_relation_test() {
        let map = parse(INPUT).0;
        let relation = get_edge_relation(&map);

        assert_eq!(
            relation.get(&Dir::Up).unwrap().get(&Point::new(8, 0)),
            Some(&Relation::new(3, 4, Dir::Left))
        );
        assert_eq!(
            relation.get(&Dir::Left).unwrap().get(&Point::new(8, 0)),
            Some(&Relation::new(4, 4, Dir::Left))
        );
    }

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 5031);
    }
}
