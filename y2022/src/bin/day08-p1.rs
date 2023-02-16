use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day08.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut grid = vec![];

    for line in lines {
        let line = line.unwrap().into_bytes();
        grid.push(line);
    }

    let mut sum = 0;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            sum += row.len();
            continue;
        }

        for (j, b) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                sum += 1;
                continue;
            }

            let mut dir_arr = Vec::with_capacity(4);
            let mut dir_item_arr = vec![];
            for col in 0..grid.len() {
                if col == i {
                    dir_arr.push(dir_item_arr);
                    dir_item_arr = vec![];
                } else {
                    dir_item_arr.push((col, j));
                }
            }
            dir_arr.push(dir_item_arr);

            dir_item_arr = vec![];
            for row in 0..row.len() {
                if row == j {
                    dir_arr.push(dir_item_arr);
                    dir_item_arr = vec![];
                } else {
                    dir_item_arr.push((i, row));
                }
            }
            dir_arr.push(dir_item_arr);

            for dir_item_arr in dir_arr {
                let mut is_visable_dir = true;
                for item in dir_item_arr {
                    if grid[item.0][item.1] >= *b {
                        is_visable_dir = false;
                        break;
                    }
                }

                if is_visable_dir {
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("sum : {:?}", sum);
}
