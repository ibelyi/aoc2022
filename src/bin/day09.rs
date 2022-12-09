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

fn abs(v: i32) -> i32 {
    if v < 0 {
        -v
    } else {
        v
    }
}

fn act(a: &str, curr: (i32, i32)) -> (i32, i32) {
    match a {
        "U" => (curr.0 + 1, curr.1),
        "R" => (curr.0, curr.1 + 1),
        "D" => (curr.0 - 1, curr.1),
        "L" => (curr.0, curr.1 - 1),
        _ => panic!("Invalid action"),
    }
}

fn mv(curr: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if abs(curr.0 - tail.0) > 1 && abs(curr.1 - tail.1) > 1 {
        if curr.0 > tail.0 + 1 {
            if curr.1 > tail.1 + 1 {
                (tail.0 + 1, tail.1 + 1)
            } else {
                (tail.0 + 1, tail.1 - 1)
            }
        } else if curr.1 > tail.1 + 1 {
            (tail.0 - 1, tail.1 + 1)
        } else {
            (tail.0 - 1, tail.1 - 1)
        }
    } else if abs(curr.0 - tail.0) > 1 || abs(curr.1 - tail.1) > 1 {
        if curr.0 > tail.0 + 1 {
            (tail.0 + 1, curr.1)
        } else if curr.0 < tail.0 - 1 {
            (tail.0 - 1, curr.1)
        } else if curr.1 > tail.1 + 1 {
            (curr.0, tail.1 + 1)
        } else if curr.1 < tail.1 - 1 {
            (curr.0, tail.1 - 1)
        } else {
            panic!("Invalid condition for tail");
        }
    } else {
        (tail.0, tail.1)
    }
}

fn debug(visited: &HashSet<(i32, i32)>) {
    let mut min = (0, 0);
    let mut max = (0, 0);
    for (y, x) in visited {
        if *y < min.0 {
            min.0 = *y;
        }
        if *y > max.0 {
            max.0 = *y;
        }
        if *x < min.1 {
            min.1 = *x;
        }
        if *x > max.1 {
            max.1 = *x;
        }
    }
    let mut field = vec![];
    for _ in min.0..=max.0 {
        field.push(vec!['.'; (max.1 - min.1 + 1) as usize]);
    }
    for (y, x) in visited {
        field[(y - min.0) as usize][(x - min.1) as usize] = '#';
    }
    println!("Visited");
    for l in field.iter().rev() {
        println!("{}", l.iter().collect::<String>());
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
        if visited.len() < 40 {
            debug(&visited);
        }
        visited.len()
    }
}
