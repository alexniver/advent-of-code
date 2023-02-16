use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day09.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut h = Axis::new();
    let mut t = Axis::new();

    let mut t_axis_arr = vec![t.clone()];

    for line in lines {
        let line = line.unwrap();

        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..num {
            h.move_dir(dir);
            t.follow_head(&h);
            if !t_axis_arr.contains(&t) {
                t_axis_arr.push(t.clone());
            }
        }
    }

    println!("len: {:?}", t_axis_arr.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Axis {
    x: i32,
    y: i32,
}

impl Axis {
    fn new() -> Self {
        Axis { x: 0, y: 0 }
    }

    fn move_dir(&mut self, dir: &str) {
        match dir {
            "L" => self.move_left(),
            "R" => self.move_right(),
            "U" => self.move_up(),
            "D" => self.move_down(),
            _ => {}
        }
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_up(&mut self) {
        self.y += 1;
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn follow_head(&mut self, head: &Axis) {
        if (head.x - self.x).abs() == 2 {
            if head.y != self.y {
                self.y += (head.y - self.y) / (head.y - self.y).abs();
            }
            self.x += (head.x - self.x) / 2;
        } else if (head.y - self.y).abs() == 2 {
            if head.x != self.x {
                self.x += (head.x - self.x) / (head.x - self.x).abs();
            }
            self.y += (head.y - self.y) / 2;
        }
    }
}
