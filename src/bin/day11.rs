use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day11");
}
struct Solution {}

#[derive(Clone, Copy)]
enum OPER {
    Add(u64),
    Mul(u64),
    Square,
}

struct Monkey {
    items: Vec<u64>,
    oper: OPER,
    test: u64,
    switch: [usize; 2],
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("10605"))],
            Step::Second => vec![("test0.txt", String::from("2713310158"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Monkey> = vec![];
        let mut items = vec![];
        let mut oper = OPER::Square;
        let mut test = 0;
        let mut switch = [0, 0];
        for l in input {
            if l.is_empty() {
                data.push(Monkey {
                    items: items.clone(),
                    oper,
                    test,
                    switch,
                });
            } else if l.starts_with("  Starting items: ") {
                items = l
                    .split(": ")
                    .last()
                    .unwrap()
                    .split(", ")
                    .map(|v| v.parse().unwrap())
                    .collect();
            } else if l.starts_with("  Operation: ") {
                let parts: Vec<&str> = l.split(" new = old ").last().unwrap().split(' ').collect();
                oper = if parts[1] == "old" {
                    OPER::Square
                } else {
                    let val = parts[1].parse().unwrap();
                    if parts[0] == "*" {
                        OPER::Mul(val)
                    } else {
                        OPER::Add(val)
                    }
                }
            } else if l.starts_with("  Test: ") {
                test = l.split(' ').last().unwrap().parse().unwrap();
            } else if l.starts_with("    If true: ") {
                switch[0] = l.split(' ').last().unwrap().parse().unwrap();
            } else if l.starts_with("    If false: ") {
                switch[1] = l.split(' ').last().unwrap().parse().unwrap();
            }
        }
        data.push(Monkey {
            items,
            oper,
            test,
            switch,
        });
        self.count(&mut data, step).to_string()
    }
}

impl Solution {
    fn count(&self, list: &mut Vec<Monkey>, step: &Step) -> usize {
        let iter = match step {
            Step::First => 20,
            Step::Second => 10000,
        };
        let mut result = vec![0; list.len()];
        let mut total = 1u64;
        for m in list.iter() {
            total *= m.test;
        }
        for _ in 0..iter {
            for m in 0..list.len() {
                let mut monkeys = vec![vec![]; list.len()];
                result[m] += list[m].items.len();
                for i in &list[m].items {
                    let mut val = match list[m].oper {
                        OPER::Square => i * i,
                        OPER::Add(v) => i + v,
                        OPER::Mul(v) => i * v,
                    };
                    if let Step::First = step {
                        val /= 3;
                    }
                    val %= total;
                    monkeys[list[m].switch[if val % list[m].test == 0 { 0 } else { 1 }]].push(val);
                }
                list[m].items.clear();
                for (i, items) in monkeys.into_iter().enumerate() {
                    items.into_iter().for_each(|v| list[i].items.push(v));
                }
            }
        }
        result.sort_unstable();
        result.iter().rev().take(2).product()
    }
}
