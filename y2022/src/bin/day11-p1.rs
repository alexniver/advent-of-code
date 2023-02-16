use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input-day11.txt").unwrap();
    let mut lines = BufReader::new(f).lines();

    let mut system = MonkeySystem::new();
    loop {
        if let Some(line) = lines.next() {
            if let Ok(line) = line {
                let mut parts = line.split_whitespace();
                if let Some("Monkey") = parts.next() {
                    // one monkey
                    let mut monkey = Monkey::new();

                    let id = parts.next().unwrap();
                    monkey.id = id[0..id.len() - 1].parse::<usize>().unwrap();

                    // Starting items: 79, 98
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace().skip(2);

                    loop {
                        let p = parts.next();
                        if p.is_none() {
                            break;
                        }

                        let p = p.unwrap().replace(",", "");
                        monkey.add_item_worry_level(p[0..p.len()].parse::<i32>().unwrap());
                    }

                    // Operation: new = old * 19
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace().skip(4);
                    let op_str = parts.next(); // *
                    monkey.op = match op_str {
                        Some("+") => match parts.next() {
                            Some("old") => Op::AddOld,
                            Some(num_str) => Op::Add(num_str.parse().unwrap()),
                            _ => {
                                panic!("error op");
                            }
                        },
                        Some("*") => match parts.next() {
                            Some("old") => Op::MultOld,
                            Some(num_str) => Op::Mult(num_str.parse().unwrap()),
                            _ => {
                                panic!("error op");
                            }
                        },
                        _ => {
                            panic!("invalid op");
                        }
                    };

                    // Test: divisible by 23
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace().skip(3);
                    monkey.div = parts.next().unwrap().parse().unwrap();

                    // If true: throw to monkey 2
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace().skip(5);
                    monkey.div_true = parts.next().unwrap().parse().unwrap();

                    // If false: throw to monkey 3
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace().skip(5);
                    monkey.div_false = parts.next().unwrap().parse().unwrap();

                    system.add_monkey(monkey);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    for _ in 0..20 {
        system.operation();
    }

    let mut max_arr = [0; 2];
    for m in system.monkey_arr {
        let mut min_num = max_arr[0];
        let mut min_idx = 0;
        for (i, num) in max_arr.iter_mut().enumerate() {
            if min_num > *num {
                min_num = *num;
                min_idx = i;
            }
        }

        if m.inspect_times > min_num {
            max_arr[min_idx] = m.inspect_times;
        }
    }

    let score: usize = max_arr.iter().product();
    println!("score : {:?}", score);
}

#[derive(Debug)]
enum Op {
    Add(i32),
    Mult(i32),
    AddOld,
    MultOld,
}

struct Monkey {
    id: usize,
    item_worry_level_arr: Vec<i32>,
    op: Op,
    div: i32,
    div_true: usize,
    div_false: usize,
    inspect_times: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            item_worry_level_arr: vec![],
            op: Op::AddOld,
            div: 0,
            div_true: 0,
            div_false: 0,
            inspect_times: 0,
        }
    }

    fn inspect(&mut self) {
        self.inspect_times += 1;
    }

    fn add_item_worry_level(&mut self, worry_level: i32) {
        self.item_worry_level_arr.push(worry_level);
    }
}

struct MonkeySystem {
    monkey_arr: Vec<Monkey>,
}

impl MonkeySystem {
    fn new() -> Self {
        MonkeySystem { monkey_arr: vec![] }
    }

    fn add_monkey(&mut self, m: Monkey) {
        self.monkey_arr.push(m);
    }

    fn operation(&mut self) {
        for i in 0..self.monkey_arr.len() {
            let mut remove_arr = vec![];
            let m = &mut self.monkey_arr[i];
            for _ in 0..m.item_worry_level_arr.len() {
                let worry_level = m.item_worry_level_arr.remove(0);
                m.inspect();

                let worry_level = match m.op {
                    Op::Add(op_num) => worry_level + op_num,
                    Op::Mult(op_num) => worry_level * op_num,
                    Op::AddOld => worry_level + worry_level,
                    Op::MultOld => worry_level * worry_level,
                };

                let worry_level = worry_level / 3;

                remove_arr.push((
                    if worry_level % m.div == 0 {
                        m.div_true
                    } else {
                        m.div_false
                    },
                    worry_level,
                ));
            }

            for remove in remove_arr {
                self.monkey_arr[remove.0].add_item_worry_level(remove.1);
            }
        }
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("M:")
            .field("id", &self.id)
            .field("items", &self.item_worry_level_arr)
            .field("op", &self.op)
            .field("div", &self.div)
            .field("div_true", &self.div_true)
            .field("div_false", &self.div_false)
            .field("inspect_times", &self.inspect_times)
            .finish()
    }
}
