use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

fn main() {
    let f = File::open("input-day13.txt").unwrap();
    let mut lines = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let mut i = 0;
    let line_len = lines.len();
    loop {
        let line1 = lines.pop().unwrap();
        let line2 = lines.pop().unwrap();
        let _ = lines.pop(); // space

        i += 3;
        if i >= line_len {
            break;
        }
    }
}

fn parse_packet(line: String) -> Packet {
    let mut packet = Packet::Arr(vec![]);
    let mut stack = vec![];
    stack.push(&packet);
    // let mut current_packet = &mut packet;

    for b in line.into_bytes() {
        match b {
            b'[' => match &mut stack.last_mut() {
                Some(Packet::Arr(v)) => {
                    let mut packet = Packet::Arr(vec![]);
                    v.push(packet);
                    let packet = &mut v.last_mut().unwrap();
                    stack.push(packet);
                }
                _ => {}
            },
            b']' => match stack.last_mut() {
                Some(Packet::Arr(v)) => {
                    stack.pop();
                }
                _ => {}
            },
            n => match stack.last_mut() {
                Some(Packet::Arr(v)) => v.push(Packet::Num(n)),
                _ => {}
            },
            b',' => {}
            _ => {}
        }
    }

    packet
}

enum Packet {
    Arr(Vec<Packet>),
    Num(u8),
}
