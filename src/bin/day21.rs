use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day21");
}
struct Solution {}

enum Oper {
    Number(i64),
    Mult(String, String),
    Div(String, String),
    Add(String, String),
    Sub(String, String),
}

struct Monkey {
    name: String,
    oper: Oper,
}

impl Monkey {
    fn new(line: &str) -> Monkey {
        let mut split = line.split(": ");
        let name = split.next().unwrap().to_string();
        let op_line = split.last().unwrap().split(' ').collect::<Vec<&str>>();
        let oper = if op_line.len() == 1 {
            Oper::Number(op_line[0].parse().unwrap())
        } else {
            let (op1, op2) = (op_line[0].to_string(), op_line[2].to_string());
            match op_line[1] {
                "*" => Oper::Mult(op1, op2),
                "/" => Oper::Div(op1, op2),
                "+" => Oper::Add(op1, op2),
                "-" => Oper::Sub(op1, op2),
                _ => panic!("Unknown operation!"),
            }
        };
        Monkey { name, oper }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("152"))],
            Step::Second => vec![("test0.txt", String::from("301"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Monkey> = input.iter().map(|l| Monkey::new(l)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

use std::collections::HashMap;

fn calc<'a>(monkeys: &'a HashMap<&String, &Monkey>, name: &'a String) -> i64 {
    if let Some(m) = monkeys.get(name) {
        match &m.oper {
            Oper::Number(n) => *n,
            Oper::Add(a, b) => calc(monkeys, a) + calc(monkeys, b),
            Oper::Sub(a, b) => calc(monkeys, a) - calc(monkeys, b),
            Oper::Mult(a, b) => calc(monkeys, a) * calc(monkeys, b),
            Oper::Div(a, b) => calc(monkeys, a) / calc(monkeys, b),
        }
    } else {
        panic!("Unknown monkey {}!", name);
    }
}

fn check<'a>(
    monkeys: &'a HashMap<&String, &Monkey>,
    name: &'a String,
    other: &str,
    humn: &mut HashMap<String, bool>,
) -> bool {
    if name == other {
        return true;
    };
    if let Some(m) = monkeys.get(name) {
        let res = match &m.oper {
            Oper::Number(_) => (false, false),
            Oper::Add(a, b) => (
                check(monkeys, a, other, humn),
                check(monkeys, b, other, humn),
            ),
            Oper::Sub(a, b) => (
                check(monkeys, a, other, humn),
                check(monkeys, b, other, humn),
            ),
            Oper::Mult(a, b) => (
                check(monkeys, a, other, humn),
                check(monkeys, b, other, humn),
            ),
            Oper::Div(a, b) => (
                check(monkeys, a, other, humn),
                check(monkeys, b, other, humn),
            ),
        };
        if res.0 != res.1 {
            humn.insert(name.to_owned(), res.0);
        }
        res.0 || res.1
    } else {
        panic!("Unknown monkey {}", name);
    }
}

fn reverse<'a>(
    monkeys: &'a HashMap<&String, &Monkey>,
    humn: &HashMap<String, bool>,
    val: i64,
    name: &String,
) -> i64 {
    if let Some(m) = monkeys.get(name) {
        if let Some(left) = humn.get(name) {
            match &m.oper {
                Oper::Number(_) => panic!("Invalid path for {}!", name),
                Oper::Add(a, b) => {
                    if *left {
                        reverse(monkeys, humn, val - calc(monkeys, b), a)
                    } else {
                        reverse(monkeys, humn, val - calc(monkeys, a), b)
                    }
                }
                Oper::Sub(a, b) => {
                    if *left {
                        reverse(monkeys, humn, val + calc(monkeys, b), a)
                    } else {
                        reverse(monkeys, humn, calc(monkeys, a) - val, b)
                    }
                }
                Oper::Mult(a, b) => {
                    if *left {
                        reverse(monkeys, humn, val / calc(monkeys, b), a)
                    } else {
                        reverse(monkeys, humn, val / calc(monkeys, a), b)
                    }
                }
                Oper::Div(a, b) => {
                    if *left {
                        reverse(monkeys, humn, val * calc(monkeys, b), a)
                    } else {
                        reverse(monkeys, humn, calc(monkeys, a) / val, b)
                    }
                }
            }
        } else {
            val
        }
    } else {
        panic!("Unknown monkey {}", name);
    }
}

impl Solution {
    fn count(&self, list: &[Monkey]) -> i64 {
        let mut monkeys = HashMap::new();
        for m in list {
            monkeys.insert(&m.name, m);
        }
        calc(&monkeys, &"root".to_string())
    }
    fn count2(&self, list: &[Monkey]) -> i64 {
        let mut monkeys = HashMap::new();
        for m in list {
            monkeys.insert(&m.name, m);
        }
        let mut humn = HashMap::new();
        check(&monkeys, &"root".to_string(), "humn", &mut humn);
        if let Some(root) = monkeys.get(&"root".to_string()) {
            let left = *humn.get(&"root".to_string()).unwrap();
            let test = match &root.oper {
                Oper::Number(_) => panic!("Root can't be a number!"),
                Oper::Add(a, b) => {
                    if left {
                        (calc(&monkeys, b), a.to_owned())
                    } else {
                        (calc(&monkeys, a), a.to_owned())
                    }
                }
                Oper::Sub(a, b) => {
                    if left {
                        (calc(&monkeys, b), a.to_owned())
                    } else {
                        (calc(&monkeys, a), a.to_owned())
                    }
                }
                Oper::Mult(a, b) => {
                    if left {
                        (calc(&monkeys, b), a.to_owned())
                    } else {
                        (calc(&monkeys, a), a.to_owned())
                    }
                }
                Oper::Div(a, b) => {
                    if left {
                        (calc(&monkeys, b), a.to_owned())
                    } else {
                        (calc(&monkeys, a), a.to_owned())
                    }
                }
            };
            reverse(&monkeys, &humn, test.0, &test.1)
        } else {
            panic!("No root monkey!");
        }
    }
}
