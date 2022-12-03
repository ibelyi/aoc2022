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
        list.iter()
            .map(|l| {
                let mut sack: HashSet<char> = l.iter().take(l.len() / 2).copied().collect();
                sack = l
                    .iter()
                    .skip(l.len() / 2)
                    .filter(|c| sack.contains(c))
                    .copied()
                    .collect();
                if sack.len() != 1 {
                    panic!("Invalid number of matching objects: {}", sack.len());
                }
                convert(sack.iter().next().unwrap())
            })
            .sum()
    }

    fn count2(&self, list: &[Vec<char>]) -> u32 {
        let mut result = 0;
        let mut sack: HashSet<char> = HashSet::new();
        for (i, l) in list.iter().enumerate() {
            match i % 3 {
                0 => sack = l.iter().copied().collect(),
                1 => sack = l.iter().filter(|c| sack.contains(*c)).copied().collect(),
                2 => {
                    sack = l.iter().filter(|c| sack.contains(*c)).copied().collect();
                    if sack.len() != 1 {
                        panic!("Invalid number of matching objects: {}", sack.len());
                    }
                    result += convert(sack.iter().next().unwrap());
                }
                _ => panic!("Impossible!"),
            }
        }
        result
    }
}
