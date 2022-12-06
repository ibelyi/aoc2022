use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day06");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("7")),
                ("test1.txt", String::from("5")),
                ("test2.txt", String::from("6")),
                ("test3.txt", String::from("10")),
                ("test4.txt", String::from("11")),
            ],
            Step::Second => vec![
                ("test0.txt", String::from("19")),
                ("test1.txt", String::from("23")),
                ("test2.txt", String::from("23")),
                ("test3.txt", String::from("29")),
                ("test4.txt", String::from("26")),
            ],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        match step {
            Step::First => self.count(input[0].as_str(), 4).to_string(),
            Step::Second => self.count(input[0].as_str(), 14).to_string(),
        }
    }
}

use std::collections::VecDeque;

impl Solution {
    fn count(&self, msg: &str, size: usize) -> usize {
        let mut prev = VecDeque::new();
        for (i, c) in msg.chars().enumerate() {
            if prev.contains(&c) {
                while let Some(v) = prev.pop_front() {
                    if v == c {
                        break;
                    }
                }
            }
            if prev.len() < size - 1 {
                prev.push_back(c);
            } else {
                return i + 1;
            }
        }
        panic!("No solution");
    }
}
