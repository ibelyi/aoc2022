use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day13");
}
struct Solution {}

#[derive(Clone)]
enum Item {
    Digit(i32),
    Array(Vec<Item>),
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("13"))],
            Step::Second => vec![("test0.txt", String::from("140"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Vec<Item>> = vec![];
        for l in input {
            if l.is_empty() {
                data.push(vec![]);
            } else {
                if data.is_empty() {
                    data.push(vec![]);
                }
                let mut stack = vec![];
                let mut digits = vec![];
                for c in l.chars() {
                    match c {
                        '[' => {
                            stack.push(vec![]);
                        }
                        ']' | ',' => {
                            if !digits.is_empty() {
                                let number = digits.iter().collect::<String>().parse().unwrap();
                                digits.clear();
                                let last = stack.len() - 1;
                                stack[last].push(Item::Digit(number));
                            }
                            if c == ']' {
                                let item = Item::Array(stack.pop().unwrap());
                                if stack.is_empty() {
                                    let last = data.len() - 1;
                                    data[last].push(item);
                                    break;
                                } else {
                                    let last = stack.len() - 1;
                                    stack[last].push(item);
                                }
                            }
                        }
                        _ => {
                            digits.push(c);
                        }
                    }
                }
                if !digits.is_empty() {
                    let number = digits.iter().collect::<String>().parse().unwrap();
                    let last = data.len() - 1;
                    data[last].push(Item::Digit(number));
                }
                if !stack.is_empty() {
                    panic!("Mismatched brackets!");
                }
            }
        }
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

use std::cmp::Ordering;

fn cmp(sig1: &Item, sig2: &Item) -> Ordering {
    match (sig1, sig2) {
        (Item::Digit(v1), Item::Digit(v2)) => v1.cmp(v2),
        (Item::Digit(_), Item::Array(_)) => cmp(&Item::Array(vec![sig1.clone()]), sig2),
        (Item::Array(_), Item::Digit(_)) => cmp(sig1, &Item::Array(vec![sig2.clone()])),
        (Item::Array(a1), Item::Array(a2)) => {
            for (i, v1) in a1.iter().enumerate() {
                if i >= a2.len() {
                    return Ordering::Greater;
                }
                let res = cmp(v1, &a2[i]);
                if res != Ordering::Equal {
                    return res;
                }
            }
            a1.len().cmp(&a2.len())
        }
    }
}

impl Solution {
    fn count(&self, list: &[Vec<Item>]) -> usize {
        let mut result = 0;
        for (i, s) in list.iter().enumerate() {
            if cmp(&s[0], &s[1]) != Ordering::Greater {
                result += i + 1;
            }
        }
        result
    }
    fn count2(&self, list: &[Vec<Item>]) -> usize {
        let extra = [
            Item::Array(vec![Item::Array(vec![Item::Digit(2)])]),
            Item::Array(vec![Item::Array(vec![Item::Digit(6)])]),
        ];
        let mut list = list.iter().flatten().collect::<Vec<&Item>>();
        list.push(&extra[0]);
        list.push(&extra[1]);
        list.sort_unstable_by(|s1, s2| cmp(s1, s2));
        list.into_iter()
            .enumerate()
            .filter(|(_, v)| extra.iter().any(|e| cmp(v, e) == Ordering::Equal))
            .map(|(i, _)| i + 1)
            .product()
    }
}
