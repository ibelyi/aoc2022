use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day09");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("13"))],
            Step::Second => vec![
                ("test0.txt", String::from("1")),
                ("test1.txt", String::from("36")),
            ],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<(&str, i32)> = input
            .iter()
            .map(|l| {
                let mut s = l.split(' ');
                (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
            })
            .collect();
        match step {
            Step::First => self.count(&data, 2).to_string(),
            Step::Second => self.count(&data, 10).to_string(),
        }
    }
}

use std::collections::HashSet;

fn act(a: &str, curr: (i32, i32)) -> (i32, i32) {
    match a {
        "U" => (curr.0 + 1, curr.1),
        "R" => (curr.0, curr.1 + 1),
        "D" => (curr.0 - 1, curr.1),
        "L" => (curr.0, curr.1 - 1),
        _ => panic!("Invalid action"),
    }
}

use std::cmp::Ordering;

fn mv(curr: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if (curr.0 - tail.0).abs() > 1 || (curr.1 - tail.1).abs() > 1 {
        (
            tail.0
                + match curr.0.cmp(&tail.0) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                },
            tail.1
                + match curr.1.cmp(&tail.1) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                },
        )
    } else {
        (tail.0, tail.1)
    }
}

impl Solution {
    fn count(&self, list: &[(&str, i32)], size: usize) -> usize {
        let mut rope = vec![(0, 0); size];
        let mut visited = HashSet::new();
        for (a, s) in list {
            for _ in 0..*s {
                rope[0] = act(*a, rope[0]);
                for i in 1..rope.len() {
                    rope[i] = mv(rope[i - 1], rope[i]);
                }
                visited.insert(rope[rope.len() - 1]);
            }
        }
        visited.len()
    }
}
