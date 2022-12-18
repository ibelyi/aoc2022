use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day18");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("64"))],
            Step::Second => vec![("test0.txt", String::from("58"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: HashSet<Vec<usize>> = input
            .iter()
            .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

use std::collections::HashSet;

impl Solution {
    fn count(&self, set: &HashSet<Vec<usize>>) -> usize {
        let mut result = 6 * set.len();
        for c in set {
            for d in 0..3 {
                let mut n = (*c).clone();
                n[d] += 1;
                if set.contains(&n) {
                    result -= 1;
                }
                if n[d] > 1 {
                    n[d] -= 2;
                    if set.contains(&n) {
                        result -= 1;
                    }
                }
            }
        }
        result
    }
    fn count2(&self, set: &HashSet<Vec<usize>>) -> usize {
        let mut max = vec![0; 3];
        let mut min = vec![100; 3];
        for c in set {
            for d in 0..3 {
                if max[d] < c[d] {
                    max[d] = c[d];
                }
                if min[d] > c[d] {
                    min[d] = c[d];
                }
            }
        }
        let mut outside = HashSet::new();
        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                let n = vec![x, y, min[2]];
                if !set.contains(&n) {
                    outside.insert(n);
                }
                let n = vec![x, y, max[2]];
                if !set.contains(&n) {
                    outside.insert(n);
                }
            }
        }
        for x in min[0]..=max[0] {
            for z in min[2]..=max[2] {
                let n = vec![x, min[1], z];
                if !set.contains(&n) {
                    outside.insert(n);
                }
                let n = vec![x, max[1], z];
                if !set.contains(&n) {
                    outside.insert(n);
                }
            }
        }
        for y in min[1]..=max[1] {
            for z in min[2]..=max[2] {
                let n = vec![min[0], y, z];
                if !set.contains(&n) {
                    outside.insert(n);
                }
                let n = vec![max[0], y, z];
                if !set.contains(&n) {
                    outside.insert(n);
                }
            }
        }
        loop {
            let mut new = HashSet::new();
            for c in &outside {
                for d in 0..3 {
                    if c[d] < max[d] {
                        let mut n = c.clone();
                        n[d] += 1;
                        if !set.contains(&n) && !outside.contains(&n) {
                            new.insert(n);
                        }
                    }
                    if c[d] > min[d] {
                        let mut n = c.clone();
                        n[d] -= 1;
                        if !set.contains(&n) && !outside.contains(&n) {
                            new.insert(n);
                        }
                    }
                }
            }
            if new.is_empty() {
                break;
            }
            for c in new {
                outside.insert(c);
            }
        }
        let mut result = 0;
        for c in set {
            for d in 0..3 {
                let mut n = (*c).clone();
                if c[d] == max[d] {
                    result += 1;
                } else {
                    n[d] += 1;
                    if outside.contains(&n) {
                        result += 1;
                    }
                }
                if c[d] == min[d] {
                    result += 1;
                } else {
                    n[d] -= 2;
                    if outside.contains(&n) {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
