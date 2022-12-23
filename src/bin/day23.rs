use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day23");
}
struct Solution {}

use std::collections::{HashMap, HashSet};

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("110"))],
            Step::Second => vec![("test0.txt", String::from("20"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut elfs = HashSet::new();
        for (y, l) in input.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    elfs.insert((y as i32, x as i32));
                }
            }
        }
        self.count(&mut elfs, step).to_string()
    }
}

impl Solution {
    fn count(&self, elfs: &mut HashSet<(i32, i32)>, step: &Step) -> usize {
        let mut i = 0;
        loop {
            let mut proposal = HashMap::new();
            for (y, x) in elfs.iter() {
                let (y, x) = (*y, *x);
                let mut choice = vec![];
                for d in 0..4 {
                    match (i + d) % 4 {
                        0 => {
                            if !elfs.contains(&(y - 1, x - 1))
                                && !elfs.contains(&(y - 1, x))
                                && !elfs.contains(&(y - 1, x + 1))
                            {
                                choice.push((y - 1, x));
                            }
                        }
                        1 => {
                            if !elfs.contains(&(y + 1, x - 1))
                                && !elfs.contains(&(y + 1, x))
                                && !elfs.contains(&(y + 1, x + 1))
                            {
                                choice.push((y + 1, x));
                            }
                        }
                        2 => {
                            if !elfs.contains(&(y - 1, x - 1))
                                && !elfs.contains(&(y, x - 1))
                                && !elfs.contains(&(y + 1, x - 1))
                            {
                                choice.push((y, x - 1));
                            }
                        }
                        3 => {
                            if !elfs.contains(&(y - 1, x + 1))
                                && !elfs.contains(&(y, x + 1))
                                && !elfs.contains(&(y + 1, x + 1))
                            {
                                choice.push((y, x + 1));
                            }
                        }
                        _ => panic!("Invalid module 4"),
                    }
                }
                if !choice.is_empty() && choice.len() != 4 {
                    proposal.entry(choice[0]).or_insert(vec![]).push((y, x));
                }
            }
            let mut moved = false;
            for (new, cand) in proposal {
                if cand.len() == 1 {
                    elfs.remove(&cand[0]);
                    elfs.insert(new);
                    moved = true;
                }
            }
            i += 1;
            match step {
                Step::First => {
                    if i == 10 {
                        break;
                    }
                }
                Step::Second => {
                    if !moved {
                        return i;
                    }
                }
            }
        }
        let min_y = elfs.iter().map(|(y, _)| *y).min().unwrap();
        let max_y = elfs.iter().map(|(y, _)| *y).max().unwrap();
        let min_x = elfs.iter().map(|(_, x)| *x).min().unwrap();
        let max_x = elfs.iter().map(|(_, x)| *x).max().unwrap();
        (max_y - min_y + 1) as usize * (max_x - min_x + 1) as usize - elfs.len()
    }
}
