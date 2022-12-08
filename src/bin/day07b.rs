use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day07");
}
struct Solution {}

enum Entry {
    File(u64),
    Dir(HashMap<String, Entry>),
}

fn sizes(entry: &Entry, dirs: &mut Vec<u64>) -> u64 {
    match entry {
        Entry::File(size) => *size,
        Entry::Dir(d) => {
            let result = d.iter().map(|(_, e)| sizes(e, dirs)).sum();
            dirs.push(result);
            result
        }
    }
}

fn dir<'a>(root: &'a mut Entry, path: &[&str]) -> &'a mut Entry {
    let mut curr = root;
    for d in path {
        curr = match curr {
            Entry::Dir(entries) => entries.get_mut(*d).unwrap(),
            Entry::File(_) => panic!("Can't cd into a file"),
        }
    }
    curr
}

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
        let mut root = Entry::Dir(HashMap::new());
        let mut curr = &mut Entry::Dir(HashMap::new());
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
                curr = dir(&mut root, &path);
            } else {
                let entry: Vec<&str> = l.split(' ').collect();
                match curr {
                    Entry::Dir(d) => {
                        if entry[0] == "dir" {
                            d.insert(entry[1].to_string(), Entry::Dir(HashMap::new()));
                        } else {
                            d.insert(entry[1].to_string(), Entry::File(entry[0].parse().unwrap()));
                        }
                    }
                    Entry::File(_) => panic!("File can't contains entries"),
                }
            }
        }
        let mut dirs = vec![];
        let total = sizes(&root, &mut dirs);
        match step {
            Step::First => self.count(&dirs).to_string(),
            Step::Second => self.count2(total, &dirs).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, dirs: &[u64]) -> u64 {
        let mut result = 0;
        for s in dirs {
            if *s <= 100000 {
                result += *s;
            }
        }
        result
    }

    fn count2(&self, total: u64, dirs: &[u64]) -> u64 {
        let need = total - 40000000;
        let mut min = total;
        for s in dirs {
            if *s >= need && *s < min {
                min = *s;
            }
        }
        min
    }
}
