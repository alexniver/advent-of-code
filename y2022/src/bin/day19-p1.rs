use std::fs;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::delimited, IResult,
};

fn main() {
    let input = fs::read_to_string("input-day19.txt").unwrap();
    let v = process(&input);
    println!("v: {:?}", v);
}

fn process(input: &str) -> usize {
    let mut max = 0;
    let (_, blueprints) = parse_blueprints(input).unwrap();

    for blueprint in blueprints.iter() {
        max += blueprint.id * search_state(State::new(), blueprint);
    }
    max as usize
}

fn search_state(state: State, blueprint: &Blueprint) -> u32 {
    let mut max = state.geo_final_num();
    for next_states in state.next_stats(blueprint) {
        if let Some(next_state) = next_states {
            max = max.max(search_state(next_state, blueprint));
        }
    }
    max
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(tag("\n"), parse_blueprint)(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u32, tag(":"))(input)?;

    let (input, ore_robot_cost_ore) =
        delimited(tag(" Each ore robot costs "), complete::u32, tag(" ore."))(input)?;

    let (input, clay_robot_cost_ore) =
        delimited(tag(" Each clay robot costs "), complete::u32, tag(" ore."))(input)?;

    let (input, obs_robot_cost_ore) = delimited(
        tag(" Each obsidian robot costs "),
        complete::u32,
        tag(" ore "),
    )(input)?;
    let (input, obs_robot_cost_clay) = delimited(tag("and "), complete::u32, tag(" clay."))(input)?;

    let (input, geo_robot_cost_ore) =
        delimited(tag(" Each geode robot costs "), complete::u32, tag(" ore "))(input)?;
    let (input, geo_robot_cost_obs) =
        delimited(tag("and "), complete::u32, tag(" obsidian."))(input)?;

    Ok((
        input,
        Blueprint {
            id,
            res_num_arr: [
                ResNum {
                    num_arr: [ore_robot_cost_ore, 0, 0, 0],
                },
                ResNum {
                    num_arr: [clay_robot_cost_ore, 0, 0, 0],
                },
                ResNum {
                    num_arr: [obs_robot_cost_ore, obs_robot_cost_clay, 0, 0],
                },
                ResNum {
                    num_arr: [geo_robot_cost_ore, 0, geo_robot_cost_obs, 0],
                },
            ],
            res_max_consume_rate: [
                ore_robot_cost_ore
                    .max(clay_robot_cost_ore)
                    .max(obs_robot_cost_ore)
                    .max(geo_robot_cost_ore),
                obs_robot_cost_clay,
                geo_robot_cost_obs,
                0,
            ],
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    res_num_arr: [ResNum; 4],
    res_max_consume_rate: [u32; 4],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ResNum {
    num_arr: [u32; 4],
}

#[derive(Debug, Clone, Copy)]
struct State {
    robot_num_arr: [u32; 4],
    res_num_arr: [u32; 4],
    time_left: u32,
}

impl State {
    fn new() -> Self {
        Self {
            robot_num_arr: [1, 0, 0, 0],
            res_num_arr: [0, 0, 0, 0],
            time_left: 24,
        }
    }

    fn next_stats(self, blueprint: &Blueprint) -> Vec<Option<Self>> {
        self.robot_num_arr
            .iter()
            .enumerate()
            .filter(|(i, robot_num)| {
                if *i != 3 {
                    **robot_num < blueprint.res_max_consume_rate[*i]
                        && blueprint.res_num_arr[*i]
                            .num_arr
                            .iter()
                            .enumerate()
                            .find(|&(i, &res_num)| res_num != 0 && self.robot_num_arr[i] == 0)
                            .is_none()
                } else {
                    true
                }
            })
            .map(|(i, _)| i)
            .into_iter()
            .map(|i| {
                (0..self.time_left)
                    .find(|time| {
                        self.res_num_arr
                            .iter()
                            .enumerate()
                            .filter(|&(j, _)| blueprint.res_num_arr[i].num_arr[j] > 0)
                            .find(|&(j, &res_num)| {
                                res_num + time * self.robot_num_arr[j]
                                    < blueprint.res_num_arr[i].num_arr[j]
                            })
                            .is_none()
                    })
                    .map(|mut time| {
                        time += 1;
                        let mut res_num_arr = [0; 4];
                        for (j, res_num) in res_num_arr.iter_mut().enumerate() {
                            *res_num = self.res_num_arr[j] + time * self.robot_num_arr[j]
                                - blueprint.res_num_arr[i].num_arr[j];
                        }
                        let mut robot_num_arr = self.robot_num_arr.clone();
                        robot_num_arr[i] += 1;
                        return Self {
                            res_num_arr,
                            robot_num_arr,
                            time_left: self.time_left - time,
                            ..self
                        };
                    })
            })
            .collect::<Vec<Option<Self>>>()
    }

    fn geo_final_num(&self) -> u32 {
        if self.robot_num_arr[3] != 0 {
            self.res_num_arr[3] + self.robot_num_arr[3] * self.time_left
        } else {
            self.res_num_arr[3]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_blueprint, parse_blueprints, process, Blueprint, ResNum};

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn parse_blueprint_test() {
        let res = parse_blueprint(
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        );

        assert_eq!(
            res,
            Ok((
                "",
                Blueprint {
                    id: 1,
                    res_num_arr: [
                        ResNum {
                            num_arr: [4, 0, 0, 0],
                        },
                        ResNum {
                            num_arr: [2, 0, 0, 0],
                        },
                        ResNum {
                            num_arr: [3, 14, 0, 0],
                        },
                        ResNum {
                            num_arr: [2, 0, 7, 0],
                        },
                    ],
                    res_max_consume_rate: [4, 14, 7, 0],
                },
            ))
        );
    }

    #[test]
    fn parse_blueprints_test() {
        assert_eq!(
            parse_blueprints(INPUT),
            Ok((
                "",
                vec![
                    Blueprint {
                        id: 1,
                        res_num_arr: [
                            ResNum {
                                num_arr: [4, 0, 0, 0],
                            },
                            ResNum {
                                num_arr: [2, 0, 0, 0],
                            },
                            ResNum {
                                num_arr: [3, 14, 0, 0],
                            },
                            ResNum {
                                num_arr: [2, 0, 7, 0],
                            },
                        ],
                        res_max_consume_rate: [4, 14, 7, 0],
                    },
                    Blueprint {
                        id: 2,
                        res_num_arr: [
                            ResNum {
                                num_arr: [2, 0, 0, 0],
                            },
                            ResNum {
                                num_arr: [3, 0, 0, 0],
                            },
                            ResNum {
                                num_arr: [3, 8, 0, 0],
                            },
                            ResNum {
                                num_arr: [3, 0, 12, 0],
                            },
                        ],
                        res_max_consume_rate: [3, 8, 12, 0],
                    }
                ],
            ))
        )
    }

    #[test]
    fn process_test() {
        assert_eq!(process(INPUT), 33);
    }
}
