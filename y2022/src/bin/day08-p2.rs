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

    let mut max_score = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, b) in row.iter().enumerate() {
            let mut search_arr = vec![];

            // up
            let mut search_item_arr = vec![];
            for x in (0..i).rev() {
                search_item_arr.push((x, j));
            }
            search_arr.push(search_item_arr);

            // down
            let mut search_item_arr = vec![];
            for x in i + 1..grid.len() {
                search_item_arr.push((x, j));
            }
            search_arr.push(search_item_arr);

            // left
            let mut search_item_arr = vec![];
            for x in (0..j).rev() {
                search_item_arr.push((i, x));
            }
            search_arr.push(search_item_arr);

            // right
            let mut search_item_arr = vec![];
            for x in j + 1..row.len() {
                search_item_arr.push((i, x));
            }
            search_arr.push(search_item_arr);

            let mut score = 1;
            for item_arr in search_arr {
                let mut s = 0;
                for item in item_arr {
                    let tmp_b = grid[item.0][item.1];
                    s += 1;
                    if tmp_b >= *b {
                        break;
                    }
                }
                score = score * s;
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("max_score : {:?}", max_score);
}
