use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day15");
}
struct Solution {}

use std::collections::HashSet;

struct Couple {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl Couple {
    fn new(line: &str) -> Couple {
        let vals: Vec<Vec<i32>> = line
            .split("Sensor at ")
            .last()
            .unwrap()
            .split(": closest beacon is at ")
            .map(|v| {
                v.split("x=")
                    .last()
                    .unwrap()
                    .split(", y=")
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();
        Couple {
            sensor: (vals[0][0], vals[0][1]),
            beacon: (vals[1][0], vals[1][1]),
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("26"))],
            Step::Second => vec![("test0.txt", String::from("56000011"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Couple> = input.iter().map(|l| Couple::new(l)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn ranges(list: &[Couple], line: i32) -> Vec<(i32, i32)> {
    let mut nobody = vec![];
    for c in list {
        let d = (c.sensor.0 - c.beacon.0).abs() + (c.sensor.1 - c.beacon.1).abs()
            - (c.sensor.1 - line).abs();
        if d < 0 {
            continue;
        }
        nobody.push((c.sensor.0 - d, c.sensor.0 + d));
    }
    nobody.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut result = vec![];
    let mut prev: Option<(i32, i32)> = None;
    for v in nobody {
        prev = if let Some(p) = prev {
            if v.0 <= p.1 {
                if p.1 < v.1 {
                    Some((p.0, v.1))
                } else {
                    Some((p.0, p.1))
                }
            } else {
                result.push(p);
                None
            }
        } else {
            Some(v)
        }
    }
    if let Some(prev) = prev {
        result.push(prev);
    }
    result
}

impl Solution {
    fn count(&self, list: &[Couple]) -> usize {
        let line = if list.len() < 20 { 10 } else { 2000000 };
        let mut ontheline = HashSet::new();
        for c in list {
            if c.beacon.1 == line {
                ontheline.insert(c.beacon.0);
            }
        }
        ranges(list, line)
            .iter()
            .map(|(s, e)| (e - s + 1) as usize)
            .sum::<usize>()
            - ontheline.len()
    }

    fn count2(&self, list: &[Couple]) -> i64 {
        let possible = if list.len() < 20 { 0..20 } else { 0..4000000 };
        for line in possible {
            let r = ranges(list, line);
            if r.len() == 2 {
                return (r[0].1 + 1) as i64 * 4000000i64 + line as i64;
            }
        }
        panic!("No solution!");
    }
}
