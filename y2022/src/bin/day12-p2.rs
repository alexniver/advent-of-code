use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day12.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut map = vec![];

    for line in lines {
        map.push(line.unwrap().into_bytes());
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut node_arr = Vec::with_capacity(map.len() * map[0].len());

    let mut a_arr = vec![];

    for (y, row) in map.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            if item == b'S' {
                start.0 = x;
                start.1 = y;
            }
            if item == b'E' {
                end.0 = x;
                end.1 = y;
            }

            if item == b'S' || item == b'a' {
                a_arr.push((x, y));
            }

            node_arr.push(Node::new(axis_to_id(x, y, map[0].len())));
        }
    }

    map[start.1][start.0] = b'a';
    map[end.1][end.0] = b'z';

    let mut result = usize::MAX;
    for (x, y) in a_arr {
        let path = search((x, y), end, &map, node_arr.clone());
        if path.len() > 0 && path.len() < result {
            result = path.len();
        }
    }

    println!("result len: {:?}", result - 1);
}

fn search(
    (start_x, start_y): (usize, usize),
    (end_x, end_y): (usize, usize),
    map: &Vec<Vec<u8>>,
    mut node_arr: Vec<Node>,
) -> Vec<Node> {
    let mut heap = BinaryHeap::new();
    let start_id = axis_to_id(start_x, start_y, map[0].len());
    node_arr[start_id].cost = 0;

    heap.push(node_arr[start_id].clone());

    let mut found = false;

    'out: loop {
        let node = heap.pop();

        if node.is_none() {
            break;
        }
        let node_id = node.unwrap().id;

        let (x, y) = id_to_axis(node_id, map[0].len());

        let mut dirs = vec![];
        // left
        if x > 0 {
            dirs.push((x - 1, y));
        }
        // right
        if x < map[0].len() - 1 {
            dirs.push((x + 1, y));
        }
        // up
        if y > 0 {
            dirs.push((x, y - 1));
        }
        // down
        if y < map.len() - 1 {
            dirs.push((x, y + 1));
        }

        for (new_x, new_y) in dirs {
            if map[new_y][new_x] <= map[y][x] + 1 {
                let new_node_id = axis_to_id(new_x, new_y, map[0].len());

                let node = node_arr[node_id];
                if node_arr[new_node_id].cost > node.cost + 1 {
                    node_arr[new_node_id].cost = node.cost + 1;
                    node_arr[new_node_id].pid = node.id;

                    // found node, exit
                    if new_x == end_x && new_y == end_y {
                        found = true;
                        break 'out;
                    } else {
                        // push to heap, continue search
                        heap.push(node_arr[new_node_id].clone());
                    }
                }
            }
        }
    }

    let mut result = vec![];

    if found {
        // end_node
        let end_node = node_arr[axis_to_id(end_x, end_y, map[0].len())];
        result.push(end_node);
        let mut pid = end_node.pid;
        loop {
            result.push(node_arr[pid]);

            if pid == start_id {
                break;
            }
            pid = node_arr[pid].pid;
        }
    }

    result
}

fn axis_to_id(x: usize, y: usize, row_len: usize) -> usize {
    y * row_len + x
}

fn id_to_axis(id: usize, row_len: usize) -> (usize, usize) {
    (id % row_len, id / row_len)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    id: usize,
    pid: usize,
    cost: usize,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            pid: 0,
            cost: usize::MAX,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
