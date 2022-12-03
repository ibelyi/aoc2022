use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day03");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("157"))],
            Step::Second => vec![("test0.txt", String::from("70"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn convert(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        c.to_digit(36).unwrap() + 17
    } else {
        c.to_digit(36).unwrap() - 9
    }
}

use std::collections::HashSet;
impl Solution {
    fn count(&self, list: &[Vec<char>]) -> u32 {
        let mut result = 0;
        for l in list {
            let mut items: HashSet<char> = HashSet::new();
            for c in l.iter().take(l.len() / 2) {
                items.insert(*c);
            }
            for c in l.iter().skip(l.len() / 2) {
                if items.contains(c) {
                    result += convert(c);
                    break;
                }
            }
        }
        result
    }

    fn count2(&self, list: &[Vec<char>]) -> u32 {
        let mut result = vec![];
        for g in 0..list.len() / 3 {
            let mut sacks = [HashSet::new(), HashSet::new(), HashSet::new()];
            for (i, l) in list.iter().skip(g * 3).take(3).enumerate() {
                for c in l {
                    sacks[i].insert(c);
                }
            }
            let mut val: HashSet<&char> = sacks[0].intersection(&sacks[1]).copied().collect();
            val = val.intersection(&sacks[2]).copied().collect();
            if val.len() == 1 {
                result.push(convert(val.iter().next().unwrap()));
            } else {
                panic!("Invalid number of common items: {}", val.len());
            }
        }
        result.iter().sum()
    }
}
