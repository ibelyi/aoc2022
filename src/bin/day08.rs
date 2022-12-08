use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day08");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("21"))],
            Step::Second => vec![("test0.txt", String::from("8"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<i32>> = input
            .iter()
            .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

use std::collections::HashSet;

impl Solution {
    fn count(&self, forest: &[Vec<i32>]) -> usize {
        let mut result = HashSet::new();
        let mut maxs = vec![-1; forest[0].len()];
        for (y, l) in forest.iter().enumerate() {
            let mut max = -1;
            // Visible from left
            for (x, t) in l.iter().enumerate() {
                if *t > max {
                    result.insert((y, x));
                    max = *t;
                }
                // Visible from top
                if *t > maxs[x] {
                    result.insert((y, x));
                    maxs[x] = *t;
                }
            }
            max = -1;
            // Visible from right
            for (x, t) in l.iter().rev().enumerate() {
                if *t > max {
                    result.insert((y, l.len() - x - 1));
                    max = *t;
                    if max == 9 {
                        break;
                    }
                }
            }
        }
        maxs = vec![-1; forest[0].len()];
        for (y, l) in forest.iter().rev().enumerate() {
            for (x, t) in l.iter().enumerate() {
                // Visible from bottom
                if *t > maxs[x] {
                    result.insert((forest.len() - y - 1, x));
                    maxs[x] = *t;
                }
            }
        }
        result.len()
    }

    fn count2(&self, forest: &[Vec<i32>]) -> i32 {
        let mut max = 0;
        for (y, l) in forest.iter().enumerate().skip(1).take(forest.len() - 2) {
            for (x, t) in l.iter().enumerate().skip(1).take(l.len() - 2) {
                let mut c = 0;
                for i in 1..=x {
                    c += 1;
                    if *t <= l[x - i] {
                        break;
                    }
                }
                let mut val = c;
                c = 0;
                for i in 1..l.len() - x {
                    c += 1;
                    if *t <= l[x + i] {
                        break;
                    }
                }
                val *= c;
                c = 0;
                for i in 1..=y {
                    c += 1;
                    if *t <= forest[y - i][x] {
                        break;
                    }
                }
                val *= c;
                c = 0;
                for i in 1..forest.len() - y {
                    c += 1;
                    if *t <= forest[y + i][x] {
                        break;
                    }
                }
                val *= c;
                if val > max {
                    max = val;
                }
            }
        }
        max
    }
}
