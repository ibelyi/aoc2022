use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day20");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("3"))],
            Step::Second => vec![("test0.txt", String::from("1623178306"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<i64> = input.iter().map(|l| l.parse().unwrap()).collect();
        self.count(&data, step).to_string()
    }
}

impl Solution {
    fn count(&self, file: &Vec<i64>, step: &Step) -> i64 {
        let file = match step {
            Step::First => (*file).clone(),
            Step::Second => file.iter().map(|v| *v * 811589153).collect(),
        };
        let mut indexes: Vec<usize> = (0..file.len()).collect();
        let times = match step {
            Step::First => 1,
            Step::Second => 10,
        };
        for _ in 0..times {
            for (i, v) in file.iter().enumerate() {
                if *v != 0 {
                    if let Some(n) = indexes.iter().position(|x| i == *x) {
                        let new = if *v < 0 {
                            let v = ((-*v) as usize) % (file.len() - 1);
                            if v <= n {
                                n - v
                            } else {
                                file.len() - 1 + n - v
                            }
                        } else {
                            (n + *v as usize) % (file.len() - 1)
                        };
                        if new != n {
                            indexes.remove(n);
                            if new > 0 {
                                indexes.insert(new, i);
                            } else {
                                indexes.push(i);
                            }
                        }
                    }
                }
            }
        }
        if let Some(zero) = indexes.iter().position(|v| file[*v] == 0) {
            [1000, 2000, 3000]
                .iter()
                .map(|i| file[indexes[(zero + i) % file.len()]])
                .sum()
        } else {
            panic!("No zero!");
        }
    }
}
