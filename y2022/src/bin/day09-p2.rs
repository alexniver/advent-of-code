use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day09.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut arr = vec![];
    for _ in 0..10 {
        arr.push(Axis::new());
    }

    let mut t_axis_arr = vec![arr[9].clone()];

    for line in lines {
        let line = line.unwrap();

        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..num {
            arr[0].move_dir(dir);
            for i in 1..10 {
                let (x, y) = (arr[i - 1].x, arr[i - 1].y);
                arr[i].follow(x, y);
            }
            if !t_axis_arr.contains(&arr[9]) {
                t_axis_arr.push(arr[9].clone());
            }
        }
        // println!("arr: {:?}", arr);
    }

    println!("len: {:?}", t_axis_arr.len());
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    fn follow(&mut self, x: i32, y: i32) {
        if (x - self.x).abs() == 2 {
            if y != self.y {
                self.y += (y - self.y) / (y - self.y).abs();
            }
            self.x += (x - self.x) / 2;
        } else if (y - self.y).abs() == 2 {
            if x != self.x {
                self.x += (x - self.x) / (x - self.x).abs();
            }
            self.y += (y - self.y) / 2;
        }
    }
}

impl std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{:?}, {:?}]", self.x, self.y))
    }
}
