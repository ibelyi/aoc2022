use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day05");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("CMZ"))],
            Step::Second => vec![("test0.txt", String::from("MCD"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut crates: Vec<Vec<char>> = vec![];
        let ncrates = (input[0].len() + 1) / 4;
        for _ in 0..ncrates {
            crates.push(vec![]);
        }
        let mut first = true;
        let mut instr = vec![];
        for l in input {
            if l.is_empty() {
                first = false;
            } else if first {
                for (i, c) in crates.iter_mut().enumerate() {
                    let mut line = l.chars().skip(i * 4);
                    if line.next().unwrap() == '[' {
                        c.push(line.next().unwrap());
                    }
                }
            } else {
                let mut split = l.split("move ");
                split = split.last().unwrap().split(" from ");
                let c = split.next().unwrap().parse::<i32>().unwrap();
                split = split.next().unwrap().split(" to ");
                let f = split.next().unwrap().parse::<usize>().unwrap() - 1;
                let t = split.next().unwrap().parse::<usize>().unwrap() - 1;
                instr.push((c, f, t));
            }
        }
        self.count(&crates, &instr, step)
    }
}

impl Solution {
    fn count(&self, crates: &Vec<Vec<char>>, instr: &[(i32, usize, usize)], step: &Step) -> String {
        let mut result = (*crates).clone();
        for (mut c, f, t) in instr {
            while c > 0 {
                c -= 1;
                let val = result[*f].remove(match step {
                    Step::First => 0,
                    Step::Second => c as usize,
                });
                result[*t].insert(0, val);
            }
        }
        result.iter().map(|v| v[0]).collect::<String>()
    }
}
