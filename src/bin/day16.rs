use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day16");
}
struct Solution {}

struct Valve {
    name: String,
    rate: usize,
    next: Vec<String>,
}

impl Valve {
    fn new(line: &str) -> Valve {
        let mut split = line
            .split("Valve ")
            .last()
            .unwrap()
            .split(" has flow rate=");
        let name = split.next().unwrap().to_string();
        let rest = split.next().unwrap();
        split = if rest.contains("tunnels lead") {
            rest.split("; tunnels lead to valves ")
        } else {
            rest.split("; tunnel leads to valve ")
        };
        let rate = split.next().unwrap().parse().unwrap();
        let next = split
            .next()
            .unwrap()
            .split(", ")
            .map(|v| v.to_string())
            .collect();
        Valve { name, rate, next }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("1651"))],
            Step::Second => vec![("test0.txt", String::from("1707"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Valve> = input.iter().map(|l| Valve::new(l)).collect();
        self.count(&data, step).to_string()
    }
}

use std::collections::HashMap;

fn solve(
    v: usize,
    list: &[Valve],
    closed: &[usize],
    shortest: &[Vec<usize>],
    time: usize,
) -> usize {
    let mut result = 0;
    let left = if list[v].rate > 0 { time - 1 } else { time };
    for n in closed.iter() {
        let t = shortest[v][*n];
        if t + 1 < left {
            let closed = closed
                .iter()
                .filter(|v| **v != *n)
                .copied()
                .collect::<Vec<usize>>();
            let r = solve(*n, list, &closed, shortest, left - t);
            if r > result {
                result = r;
            }
        }
    }
    result + list[v].rate * left
}

impl Solution {
    fn count(&self, list: &[Valve], step: &Step) -> usize {
        let mut map = HashMap::new();
        let mut closed = vec![];
        for (i, v) in list.iter().enumerate() {
            map.insert(v.name.to_string(), i);
            if v.rate > 0 {
                closed.push(i);
            }
        }
        let mut shortest = vec![vec![list.len(); list.len()]; list.len()];
        for (v, item) in shortest.iter_mut().enumerate() {
            item[v] = 0
        }
        let mut changed = true;
        while changed {
            changed = false;
            for v in 0..list.len() {
                for (n, nv) in list.iter().enumerate() {
                    if shortest[v][n] < list.len() {
                        for nn in nv.next.iter().map(|v| map.get(v).unwrap()) {
                            if shortest[v][*nn] > shortest[v][n] + 1 {
                                shortest[v][*nn] = shortest[v][n] + 1;
                                shortest[*nn][v] = shortest[n][v] + 1;
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        let start = *map.get("AA").unwrap();
        match step {
            Step::First => solve(start, list, &closed, &shortest, 30),
            Step::Second => {
                let mut result = 0;
                for i in 1..=closed.len() / 2 {
                    let mut stack: Vec<usize> = (0..i).collect();
                    'done: loop {
                        let human: Vec<usize> = closed
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| stack.contains(i))
                            .map(|(_, v)| *v)
                            .collect();
                        let elephant: Vec<usize> = closed
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| !stack.contains(i))
                            .map(|(_, v)| *v)
                            .collect();
                        let r = solve(start, list, &human, &shortest, 26)
                            + solve(start, list, &elephant, &shortest, 26);
                        if r > result {
                            result = r;
                        }
                        let mut curr = stack.len() - 1;
                        loop {
                            stack[curr] += 1;
                            if stack[curr] <= closed.len() - stack.len() + curr {
                                break;
                            }
                            if curr > 0 {
                                curr -= 1;
                            } else {
                                break 'done;
                            }
                        }
                        for i in curr + 1..stack.len() {
                            stack[i] = stack[i - 1] + 1;
                        }
                    }
                }
                result
            }
        }
    }
}
