use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day04");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("2"))],
            Step::Second => vec![("test0.txt", String::from("4"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<Vec<i32>>> = input
            .iter()
            .map(|s| {
                s.split(',')
                    .map(|r| {
                        r.split('-')
                            .map(|v| v.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, list: &[Vec<Vec<i32>>]) -> usize {
        list.iter()
            .filter(|v| {
                (v[0][0] <= v[1][0] && v[0][1] >= v[1][1])
                    || (v[0][0] >= v[1][0] && v[0][1] <= v[1][1])
            })
            .count()
    }
    fn count2(&self, list: &[Vec<Vec<i32>>]) -> usize {
        list.iter()
            .filter(|v| v[0][1] >= v[1][0] && v[0][0] <= v[1][1])
            .count()
    }
}
