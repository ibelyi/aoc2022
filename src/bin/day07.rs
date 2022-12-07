use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day07");
}
struct Solution {}

use std::collections::HashMap;

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("95437"))],
            Step::Second => vec![("test0.txt", String::from("24933642"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut path = vec![];
        let mut prefix = "".to_string();
        let mut files = HashMap::new();
        for l in input {
            if l.starts_with("$ cd ") {
                match l.split("$ cd ").last().unwrap() {
                    "/" => {
                        path.clear();
                    }
                    ".." => {
                        path.pop();
                    }
                    dir => path.push(dir),
                }
            } else if l.starts_with("$ ls") {
                prefix = if path.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{}/", path.join("/"))
                }
            } else {
                let entry: Vec<&str> = l.split(' ').collect();
                if entry[0] != "dir" {
                    files.insert(
                        format!("{}{}", prefix, entry[1]),
                        entry[0].parse::<u64>().unwrap(),
                    );
                }
            }
        }
        let mut dirs: HashMap<String, u64> = HashMap::new();
        for (f, s) in files {
            let parts = f.split('/').count();
            for i in 1..parts {
                let path = f.split('/').take(i).collect::<Vec<&str>>().join("/");
                let count = dirs.entry(path).or_insert(0);
                *count += s;
            }
        }
        match step {
            Step::First => self.count(&dirs).to_string(),
            Step::Second => self.count2(&dirs).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, dirs: &HashMap<String, u64>) -> u64 {
        let mut result = 0;
        for s in dirs.values() {
            if *s <= 100000 {
                result += *s;
            }
        }
        result
    }

    fn count2(&self, dirs: &HashMap<String, u64>) -> u64 {
        let need = dirs[""] - 40000000;
        let mut min = dirs[""];
        for s in dirs.values() {
            if *s >= need && *s < min {
                min = *s;
            }
        }
        min
    }
}
