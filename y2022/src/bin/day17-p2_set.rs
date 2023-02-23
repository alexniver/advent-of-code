use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input-day17.txt").unwrap();
    let v = process(&input);
    println!("v: {}", v);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

fn get_shape(block: &Block) -> HashSet<Point> {
    let (id, x, y) = (block.id, block.x, block.y);
    match id {
        0 => {
            let mut hash = HashSet::new();
            hash.insert(Point::new(x, y));
            hash.insert(Point::new(x + 1, y));
            hash.insert(Point::new(x + 2, y));
            hash.insert(Point::new(x + 3, y));
            hash
        }
        1 => {
            let mut hash = HashSet::new();
            hash.insert(Point::new(x + 1, y));
            hash.insert(Point::new(x, y + 1));
            hash.insert(Point::new(x + 1, y + 1));
            hash.insert(Point::new(x + 2, y + 1));
            hash.insert(Point::new(x + 1, y + 2));
            hash
        }
        2 => {
            let mut hash = HashSet::new();
            hash.insert(Point::new(x, y));
            hash.insert(Point::new(x + 1, y));
            hash.insert(Point::new(x + 2, y));
            hash.insert(Point::new(x + 2, y + 1));
            hash.insert(Point::new(x + 2, y + 2));
            hash
        }
        3 => {
            let mut hash = HashSet::new();
            hash.insert(Point::new(x, y));
            hash.insert(Point::new(x, y + 1));
            hash.insert(Point::new(x, y + 2));
            hash.insert(Point::new(x, y + 3));
            hash
        }
        4 => {
            let mut hash = HashSet::new();
            hash.insert(Point::new(x, y));
            hash.insert(Point::new(x + 1, y));
            hash.insert(Point::new(x, y + 1));
            hash.insert(Point::new(x + 1, y + 1));
            hash
        }
        _ => panic!("ohhhh~"),
    }
}

fn process(input: &str) -> usize {
    let mut chamber = Chamber::new();
    let mut rock_num: u64 = 0;
    'out: loop {
        for &b in input.as_bytes() {
            let mut is_rock_down = false;
            match b {
                b'<' => is_rock_down = chamber.tick(true),
                b'>' => is_rock_down = chamber.tick(false),
                _ => {}
            }

            if is_rock_down {
                rock_num += 1;
                if rock_num == 2022 {
                    break 'out;
                }
            }
        }
    }
    // for j in (0..=chamber.max_y).rev() {
    //     let mut line = vec![];
    //     for i in 0..7 {
    //         line.push(if chamber.chamber.contains(&Point::new(i, j)) {
    //             b'#'
    //         } else {
    //             b'.'
    //         });
    //     }
    //     println!("{:?}", std::str::from_utf8(&line).unwrap());
    // }

    chamber.max_y as usize
}

struct Chamber {
    chamber: HashSet<Point>,
    block: Block,
    max_y: u64,
}

impl Chamber {
    fn new() -> Self {
        Self {
            chamber: HashSet::new(),
            block: Block::new(0, 2, 3),
            max_y: 0,
        }
    }

    fn tick(&mut self, is_left: bool) -> bool {
        // println!("left: {:?}, block: {:?}", is_left, self.block);

        // move left
        if is_left && get_shape(&self.block).iter().find(|v| v.x == 0).is_none() {
            let block_new = Block {
                x: self.block.x - 1,
                ..self.block
            };

            if self.is_shape_ok(&block_new) {
                self.block = block_new;
            }
        }

        // move right
        if !is_left && get_shape(&self.block).iter().find(|v| v.x == 6).is_none() {
            let block_new = Block {
                x: self.block.x + 1,
                ..self.block
            };

            if self.is_shape_ok(&block_new) {
                self.block = block_new;
            }
        }

        // fall down
        let mut is_rock_done = self.block.y == 0;

        if self.block.y > 0 {
            // let block_down = Block::new(self.block.id, self.block.x, self.block.y - 1);
            let block_down = Block {
                y: self.block.y - 1,
                ..self.block
            };

            if self.is_shape_ok(&block_down) {
                self.block.y = self.block.y - 1;
                is_rock_done = false;
            } else {
                is_rock_done = true;
            }
        }

        if is_rock_done {
            let shape = get_shape(&self.block);

            let mut shape_y_arr = shape.iter().map(|p| p.y).collect::<Vec<u64>>();
            shape_y_arr.sort();

            self.max_y = self.max_y.max(*shape_y_arr.last().unwrap() + 1);
            self.chamber.extend(shape);
            self.block = Block::new((self.block.id + 1) % 5, 2, self.max_y + 3);
        }

        is_rock_done
    }

    fn is_shape_ok(&self, block_new: &Block) -> bool {
        let shape_new = get_shape(&block_new);
        let diff_size = shape_new
            .difference(&self.chamber)
            .collect::<HashSet<&Point>>()
            .len();
        diff_size == shape_new.len()
    }
}

#[derive(Debug)]
struct Block {
    id: u8,
    x: u64,
    y: u64,
}

impl Block {
    fn new(idx: u8, x: u64, y: u64) -> Self {
        Self { id: idx, x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 3068);
    }
}
