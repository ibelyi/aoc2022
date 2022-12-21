use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution::new(&[]);
    solver.solve("day21");
}

enum Oper {
    Mult,
    Div,
    Add,
    Sub,
}

enum Monkey {
    Number(i64),
    Oper(Oper, String, String),
}

impl Monkey {
    fn new(line: &str) -> (String, Monkey) {
        let mut split = line.split(": ");
        let name = split.next().unwrap().to_string();
        let op_line = split.last().unwrap().split(' ').collect::<Vec<&str>>();
        (
            name,
            if op_line.len() == 1 {
                Monkey::Number(op_line[0].parse().unwrap())
            } else {
                Monkey::Oper(
                    match op_line[1] {
                        "*" => Oper::Mult,
                        "/" => Oper::Div,
                        "+" => Oper::Add,
                        "-" => Oper::Sub,
                        _ => panic!("Unknown operation!"),
                    },
                    op_line[0].to_owned(),
                    op_line[2].to_owned(),
                )
            },
        )
    }
}

use std::collections::HashMap;

struct Solution {
    monkeys: HashMap<String, Monkey>,
}

impl Solution {
    fn new(list: &[String]) -> Solution {
        Solution {
            monkeys: list.iter().map(|l| Monkey::new(l)).collect(),
        }
    }
    fn calc(&self, name: &str) -> i64 {
        if let Some(m) = self.monkeys.get(name) {
            match m {
                Monkey::Number(n) => *n,
                Monkey::Oper(o, a, b) => match o {
                    Oper::Add => self.calc(a) + self.calc(b),
                    Oper::Sub => self.calc(a) - self.calc(b),
                    Oper::Mult => self.calc(a) * self.calc(b),
                    Oper::Div => self.calc(a) / self.calc(b),
                },
            }
        } else {
            panic!("Unknown monkey {}!", name);
        }
    }
    fn check(&self, name: &str, other: &str, humn: &mut HashMap<String, bool>) -> bool {
        if name == other {
            return true;
        };
        if let Some(m) = self.monkeys.get(name) {
            let res = match m {
                Monkey::Number(_) => (false, false),
                Monkey::Oper(_, a, b) => (self.check(a, other, humn), self.check(b, other, humn)),
            };
            if res.0 != res.1 {
                humn.insert(name.to_owned(), res.0);
            }
            res.0 || res.1
        } else {
            panic!("Unknown monkey {}", name);
        }
    }

    fn reverse(&self, humn: &HashMap<String, bool>, val: i64, name: &str) -> i64 {
        if let Some(m) = self.monkeys.get(name) {
            if let Some(left) = humn.get(name) {
                match m {
                    Monkey::Number(_) => panic!("Invalid path for {}!", name),
                    Monkey::Oper(o, a, b) => match o {
                        Oper::Add => {
                            if *left {
                                self.reverse(humn, val - self.calc(b), a)
                            } else {
                                self.reverse(humn, val - self.calc(a), b)
                            }
                        }
                        Oper::Sub => {
                            if *left {
                                self.reverse(humn, val + self.calc(b), a)
                            } else {
                                self.reverse(humn, self.calc(a) - val, b)
                            }
                        }
                        Oper::Mult => {
                            if *left {
                                self.reverse(humn, val / self.calc(b), a)
                            } else {
                                self.reverse(humn, val / self.calc(a), b)
                            }
                        }
                        Oper::Div => {
                            if *left {
                                self.reverse(humn, val * self.calc(b), a)
                            } else {
                                self.reverse(humn, self.calc(a) / val, b)
                            }
                        }
                    },
                }
            } else {
                val
            }
        } else {
            panic!("Unknown monkey {}", name);
        }
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
        match step {
            Step::First => Solution::new(input).count().to_string(),
            Step::Second => Solution::new(input).count2().to_string(),
        }
    }
}

impl Solution {
    fn count(&self) -> i64 {
        self.calc("root")
    }
    fn count2(&self) -> i64 {
        let mut humn = HashMap::new();
        self.check("root", "humn", &mut humn);
        if let Some(root) = self.monkeys.get("root") {
            let left = *humn.get("root").unwrap();
            let (val, name) = match root {
                Monkey::Number(_) => panic!("Root can't be a number!"),
                Monkey::Oper(_, a, b) => {
                    if left {
                        (self.calc(b), a)
                    } else {
                        (self.calc(a), b)
                    }
                }
            };
            self.reverse(&humn, val, name)
        } else {
            panic!("No root monkey!");
        }
    }
}
