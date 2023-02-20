use std::{collections::VecDeque, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    IResult,
};

fn main() {
    let s = fs::read_to_string("input-day16.txt").unwrap();
    println!("v: {:?}", process(&s));
}

fn process(s: &str) -> usize {
    let v_list = parse_v_list(s).unwrap().1;
    let mut vv_list = Vec::with_capacity(v_list.len());

    let mut aa_idx = 0;
    for (i, v) in v_list.iter().enumerate() {
        let to =
            v.to.iter()
                .map(|&name| v_list.iter().position(|v| name == v.name).unwrap())
                .collect::<Vec<usize>>();

        if v.name == "AA" {
            aa_idx = i;
        }
        vv_list.push(VV {
            id: i,
            rate: v.rate,
            to,
            target_idx: 0,
        });
    }

    let mut path_time_dict = Vec::new();
    for i in 0..vv_list.len() {
        path_time_dict.push(vec![]);
        for j in 0..vv_list.len() {
            path_time_dict[i].push(bfs_time(i, j, &vv_list));
        }
    }

    // rate not zero list
    let v_target_list = vv_list
        .iter()
        .filter(|vv| vv.rate != 0 || v_list[vv.id].name == "AA")
        .map(|vv| vv.id)
        .collect::<Vec<usize>>();

    for (idx, &v) in v_target_list.iter().enumerate() {
        vv_list[v].target_idx = idx;
    }
    println!("size: {:?}", v_target_list.len());

    dfs(aa_idx, 30, 0, &vv_list, &v_target_list, &path_time_dict)
}

fn dfs(
    from_id: usize,
    mut left_time: usize,
    mut value_opened: u16,
    vv_list: &Vec<VV>,
    v_target_list: &Vec<usize>,
    path_time_dict: &Vec<Vec<usize>>,
) -> usize {
    if left_time <= 1 {
        return 0;
    }

    let bit = 1 << vv_list[from_id].target_idx;

    let mut value = 0;
    // open value if rate bigger than zero
    if value_opened & bit == 0 && vv_list[from_id].rate > 0 {
        left_time -= 1;
        value = vv_list[from_id].rate * left_time;
    }
    value_opened |= bit;

    let mut max_value = 0;
    for &to_id in v_target_list {
        let to_id_bit = 1 << vv_list[to_id].target_idx;
        if value_opened & to_id_bit != 0 {
            continue;
        }

        let t = path_time_dict[from_id][to_id];
        if left_time < t {
            continue;
        }

        max_value = max_value.max(dfs(
            to_id,
            left_time - t,
            value_opened,
            vv_list,
            v_target_list,
            &path_time_dict,
        ));
    }

    max_value + value
}

fn bfs_time(from_id: usize, to_id: usize, vv_list: &Vec<VV>) -> usize {
    let mut queue = VecDeque::new();

    let mut use_time = vec![usize::MAX; vv_list.len()];
    use_time[from_id] = 0;

    queue.push_back(from_id);

    let mut result = usize::MAX;

    while queue.len() > 0 {
        let id = queue.pop_front().unwrap();
        if id == to_id {
            result = use_time[id];
            break;
        }

        for &t_id in vv_list[id].to.iter() {
            if use_time[t_id] > use_time[id] + 1 {
                use_time[t_id] = use_time[id] + 1;
                queue.push_back(t_id);
            }
        }
    }

    result
}

fn parse_v_list(s: &str) -> IResult<&str, Vec<V>> {
    separated_list1(line_ending, parse_v)(s)
}

fn parse_v(s: &str) -> IResult<&str, V> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = alpha1(s)?;
    let (s, _) = tag(" has flow rate=")(s)?;
    let (s, rate) = complete::i32(s)?;
    let (s, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(s)?;
    let (s, to) = (separated_list1(tag(", "), alpha1))(s)?;

    Ok((
        s,
        V {
            name,
            rate: rate as usize,
            to,
        },
    ))
}

#[derive(Debug)]
struct V<'a> {
    name: &'a str,
    rate: usize,
    to: Vec<&'a str>,
}

struct VV {
    id: usize,
    rate: usize,
    to: Vec<usize>,
    target_idx: usize,
}

#[cfg(test)]
mod tests {
    use crate::{bfs_time, parse_v, parse_v_list, process, VV};

    const TEXT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn parse_v_test() {
        let v = parse_v("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
            .unwrap()
            .1;
        assert_eq!(v.name, "AA");
        assert_eq!(v.rate, 0);
        assert_eq!(v.to, vec!["DD", "II", "BB"]);
    }

    #[test]
    fn parse_v_list_test() {
        let v_list = parse_v_list(TEXT).unwrap().1;

        assert_eq!(v_list.len(), 10);
    }

    #[test]
    fn process_test() {
        assert_eq!(process(TEXT), 1651);
    }

    #[test]
    fn time_use_test() {
        let v_list = parse_v_list(TEXT).unwrap().1;
        let mut vv_list = Vec::with_capacity(v_list.len());

        for (i, v) in v_list.iter().enumerate() {
            let to =
                v.to.iter()
                    .map(|&name| v_list.iter().position(|v| name == v.name).unwrap())
                    .collect::<Vec<usize>>();

            vv_list.push(VV {
                id: i,
                rate: v.rate,
                to,
                target_idx: 0,
            });
        }

        assert_eq!(bfs_time(0, 0, &vv_list), 0);
        assert_eq!(bfs_time(0, 1, &vv_list), 1);
        assert_eq!(bfs_time(0, 2, &vv_list), 2);
        assert_eq!(bfs_time(0, 1, &vv_list), 1);
        assert_eq!(bfs_time(1, 9, &vv_list), 3);
        assert_eq!(bfs_time(9, 7, &vv_list), 7);
    }
}
