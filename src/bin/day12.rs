use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day12");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("31"))],
            Step::Second => vec![("test0.txt", String::from("29"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut start = (0, 0);
        let mut finish = (0, 0);
        let data: Vec<Vec<u32>> = input
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (y, x);
                            0
                        } else if c == 'E' {
                            finish = (y, x);
                            25
                        } else {
                            c as u32 - 'a' as u32
                        }
                    })
                    .collect()
            })
            .collect();
        self.count(&data, &start, &finish, step).to_string()
    }
}

impl Solution {
    fn count(
        &self,
        map: &[Vec<u32>],
        start: &(usize, usize),
        finish: &(usize, usize),
        step: &Step,
    ) -> i32 {
        let mut steps = vec![vec![-1i32; map[0].len()]; map.len()];
        match step {
            Step::First => {
                steps[start.0][start.1] = 0;
            }
            Step::Second => {
                for (y, l) in map.iter().enumerate() {
                    for (x, v) in l.iter().enumerate() {
                        if *v == 0 {
                            steps[y][x] = 0;
                        }
                    }
                }
            }
        }

        let mut changed = true;
        while steps[finish.0][finish.1] == -1 || changed {
            changed = false;
            for y in 0..steps.len() {
                for x in 0..steps[0].len() {
                    if steps[y][x] != -1 {
                        if x > 0
                            && map[y][x - 1] < map[y][x] + 2
                            && (steps[y][x - 1] == -1 || steps[y][x - 1] > steps[y][x] + 1)
                        {
                            steps[y][x - 1] = steps[y][x] + 1;
                            changed = true;
                        }
                        if x < map[0].len() - 1
                            && map[y][x + 1] < map[y][x] + 2
                            && (steps[y][x + 1] == -1 || steps[y][x + 1] > steps[y][x] + 1)
                        {
                            steps[y][x + 1] = steps[y][x] + 1;
                            changed = true;
                        }
                        if y > 0
                            && map[y - 1][x] < map[y][x] + 2
                            && (steps[y - 1][x] == -1 || steps[y - 1][x] > steps[y][x] + 1)
                        {
                            steps[y - 1][x] = steps[y][x] + 1;
                            changed = true;
                        }
                        if y < map.len() - 1
                            && map[y + 1][x] < map[y][x] + 2
                            && (steps[y + 1][x] == -1 || steps[y + 1][x] > steps[y][x] + 1)
                        {
                            steps[y + 1][x] = steps[y][x] + 1;
                            changed = true;
                        }
                    }
                }
            }
        }
        steps[finish.0][finish.1]
    }
}
