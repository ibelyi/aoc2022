use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day01");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("24000"))],
            Step::Second => vec![("test0.txt", String::from("45000"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data = vec![];
        let mut elf = 0;
        for v in input {
            if v.is_empty() {
                data.push(elf);
                elf = 0;
            } else {
                elf += v.parse::<i32>().unwrap();
            }
        }
        data.push(elf);
        data.sort_unstable();
        data.reverse();
        match step {
            Step::First => self.count(&data, 1).to_string(),
            Step::Second => self.count(&data, 3).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, list: &[i32], count: usize) -> i32 {
        list.iter().take(count).sum()
    }
}
