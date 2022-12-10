use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day10");
}
struct Solution {}

enum OPER {
    Noop,
    AddX(i32),
}

impl OPER {
    fn new(line: &str) -> OPER {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "noop" {
            OPER::Noop
        } else {
            OPER::AddX(parts[1].parse().unwrap())
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("13140"))],
            Step::Second => vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<OPER> = input.iter().map(|l| OPER::new(l)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, list: &[OPER]) -> i32 {
        let mut x = 1;
        let mut cycle = 0;
        let mut result = 0;
        for op in list {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                result += cycle * x;
            }
            if let OPER::AddX(v) = op {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    result += cycle * x;
                }
                x += v;
            };
        }
        result
    }

    fn count2(&self, list: &[OPER]) -> i32 {
        let mut x = 1i32;
        let mut cycle = 0;
        let mut crt = vec![];
        for op in list {
            crt.push(if (cycle % 40 - x).abs() < 2 { '#' } else { '.' });
            cycle += 1;
            if let OPER::AddX(v) = op {
                crt.push(if (cycle % 40 - x).abs() < 2 { '#' } else { '.' });
                cycle += 1;
                x += *v;
            }
        }
        println!("   --- Result ---   ");
        for i in 0..crt.len() / 40 {
            println!("{}", crt.iter().skip(i * 40).take(40).collect::<String>());
        }
        0
    }
}
