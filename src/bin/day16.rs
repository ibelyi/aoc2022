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
    map: &HashMap<String, usize>,
    closed: &[usize],
    shortest: &HashMap<(usize, usize), usize>,
    time: usize,
) -> usize {
    let mut result = 0;
    let left = if list[v].rate > 0 { time - 1 } else { time };
    for n in closed.iter() {
        let r = if let Some(t) = shortest.get(&(v, *n)) {
            if t + 1 < left {
                let closed = closed
                    .iter()
                    .filter(|v| **v != *n)
                    .copied()
                    .collect::<Vec<usize>>();
                solve(*n, list, map, &closed, shortest, left - t)
            } else {
                0
            }
        } else {
            0
        };
        if r > result {
            result = r;
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
        let mut shortest = HashMap::new();
        for v in 0..list.len() {
            shortest.insert((v, v), 0usize);
        }
        let mut changed = true;
        while changed {
            changed = false;
            for v in 0..list.len() {
                for (n, nv) in list.iter().enumerate() {
                    let mut new = vec![];
                    if let Some(p) = shortest.get(&(v, n)) {
                        for nn in nv.next.iter().map(|v| map.get(v).unwrap()) {
                            if let Some(vv) = shortest.get(&(v, *nn)) {
                                if *vv > p + 1 {
                                    new.push((v, *nn, p + 1));
                                    changed = true;
                                }
                            } else {
                                new.push((v, *nn, p + 1));
                                changed = true;
                            }
                        }
                    }
                    for (x, y, z) in new {
                        shortest.insert((x, y), z);
                        shortest.insert((y, x), z);
                    }
                }
            }
        }
        let start = *map.get("AA").unwrap();
        match step {
            Step::First => solve(start, list, &map, &closed, &shortest, 30),
            Step::Second => {
                let mut result = 0;
                for i in 1..=closed.len() / 2 {
                    let mut stack: Vec<usize> = (0..i).collect();
                    loop {
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
                        let h = solve(start, list, &map, &human, &shortest, 26);
                        let e = solve(start, list, &map, &elephant, &shortest, 26);
                        if h + e > result {
                            result = h + e;
                        }
                        let mut done = false;
                        let mut curr = stack.len() - 1;
                        loop {
                            stack[curr] += 1;
                            if stack[curr] <= closed.len() - stack.len() + curr {
                                break;
                            }
                            if curr > 0 {
                                curr -= 1;
                            } else {
                                done = true;
                                break;
                            }
                        }
                        if done {
                            break;
                        } else {
                            while curr + 1 < stack.len() {
                                stack[curr + 1] = stack[curr] + 1;
                                curr += 1;
                            }
                        }
                    }
                }
                result
            }
        }
    }
}
