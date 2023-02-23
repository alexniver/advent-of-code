use std::fs;

fn main() {
    let input = fs::read_to_string("input-day17.txt").unwrap();
    let v = process(&input);
    println!("v: {}", v);
}

const SHAPES: [[[bool; 4]; 4]; 5] = [
    [
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
    ],
    [
        [true, true, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false, false, false, false],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [true, true, false, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
];

fn process(input: &str) -> usize {
    let mut chamber = Chamber::new();
    let mut rock_num = 0;
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
    // for line in chamber.chamber.iter().rev() {
    //     println!(
    //         "line: {:?}",
    //         std::str::from_utf8(
    //             &line
    //                 .iter()
    //                 .map(|&v| if v { b'#' } else { b'.' })
    //                 .collect::<Vec<u8>>()
    //         )
    //         .unwrap()
    //     );
    // }
    chamber.chamber.len()
}

struct Chamber {
    chamber: Vec<[bool; 7]>,
    block: Block,
}

impl Chamber {
    fn new() -> Self {
        Self {
            chamber: vec![],
            block: Block::new(0, 2, 3),
        }
    }

    fn tick(&mut self, is_left: bool) -> bool {
        // move
        if (is_left && self.block.x > 0) || (!is_left && self.block.x < 6) {
            let new_block = Block::new(
                self.block.idx,
                if is_left {
                    self.block.x - 1
                } else {
                    self.block.x + 1
                },
                self.block.y,
            );

            // test can block move
            if self.is_new_block_ok(&new_block) {
                self.block = new_block;
            }
        }

        // fall down
        let mut is_rock_down = self.block.y == 0;
        if self.block.y > 0 {
            let new_block = Block::new(self.block.idx, self.block.x, self.block.y - 1);
            if self.is_new_block_ok(&new_block) {
                self.block = new_block;
            } else {
                is_rock_down = true;
            }
        }

        if is_rock_down {
            // add to chamber
            let shape = SHAPES[self.block.idx as usize];
            for (y, line) in shape.iter().enumerate() {
                let is_need_new_line = !line.iter().find(|&&v| v).is_none();
                if is_need_new_line {
                    if self.chamber.get(y + self.block.y as usize).is_none() {
                        self.chamber.push([false; 7]);
                    }
                }

                for (x, &v) in line.iter().enumerate() {
                    if v {
                        self.chamber[y + self.block.y as usize][x + self.block.x as usize] = v;
                    }
                }
            }

            // spawn new block
            self.block = Block::new((self.block.idx + 1) % 5, 2, (self.chamber.len() + 3) as u32);
        }

        is_rock_down
    }

    fn is_new_block_ok(&self, new_block: &Block) -> bool {
        let shape = SHAPES[new_block.idx as usize];
        for (y, line) in shape.iter().enumerate() {
            for (x, &v) in line.iter().enumerate() {
                if v {
                    let new_x = x + new_block.x as usize;
                    if new_x > 6 {
                        return false;
                    }
                    let new_y = y + new_block.y as usize;
                    if let Some(c_line) = self.chamber.get(new_y) {
                        if let Some(&c_v) = c_line.get(new_x) {
                            if c_v {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[derive(Debug)]
struct Block {
    idx: u8,
    x: u32,
    y: u32,
}

impl Block {
    fn new(idx: u8, x: u32, y: u32) -> Self {
        Self { idx, x, y }
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
