use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day25");
}
struct Solution {}

fn to_digit(c: char) -> i32 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        o => panic!("Invalid char {}", o),
    }
}

fn to_snafu(digit: i32) -> char {
    match digit {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        o => panic!("Invalid digit {}", o),
    }
}

fn add_snafu(v1: &str, v2: &str) -> String {
    let a1 = v1.chars().rev().collect::<Vec<char>>();
    let a2 = v2.chars().rev().collect::<Vec<char>>();
    let (a1, a2) = if a2.len() > a1.len() {
        (a2, a1)
    } else {
        (a1, a2)
    };
    let mut result = vec![];
    let mut carry = 0;
    for (i, c) in a1.iter().enumerate() {
        let mut res = if i < a2.len() { to_digit(a2[i]) } else { 0 } + to_digit(*c) + carry;
        carry = if res < -2 {
            res += 5;
            -1
        } else if res > 2 {
            res -= 5;
            1
        } else {
            0
        };
        result.push(to_snafu(res));
    }
    if carry > 0 {
        result.push(to_snafu(carry));
    }
    result.iter().rev().collect::<String>()
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("2=-1=0"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, _: &Step, input: &[String]) -> String {
        let mut result = "0".to_string();
        for line in input {
            result = add_snafu(&result, line);
        }
        result
    }
}
