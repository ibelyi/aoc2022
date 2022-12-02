use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day02");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("15"))],
            Step::Second => vec![("test0.txt", String::from("12"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<i32>> = input
            .iter()
            .map(|s| {
                s.split(' ')
                    .map(|c| match c {
                        "A" | "X" => 1,
                        "B" | "Y" => 2,
                        "C" | "Z" => 3,
                        _ => panic!("{} is unknown", c),
                    })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        match step {
            Step::First => self.count(&data, game).to_string(),
            Step::Second => self.count(&data, game2).to_string(),
        }
    }
}

fn game(play: &[i32]) -> i32 {
    if play[0] == play[1] {
        play[1] + 3
    } else if play[1] == (play[0] % 3 + 1) {
        play[1] + 6
    } else {
        play[1]
    }
}

fn game2(play: &[i32]) -> i32 {
    match play[1] {
        1 => (play[0] + 1) % 3 + 1,
        2 => play[0] + 3,
        3 => (play[0] % 3) + 7,
        _ => panic!("Invalid guide {}", play[1]),
    }
}

impl Solution {
    fn count(&self, list: &[Vec<i32>], game: impl Fn(&[i32]) -> i32) -> i32 {
        list.iter().map(|v| game(v)).sum()
    }
}
