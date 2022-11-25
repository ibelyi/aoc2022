#!/bin/bash -e

if [ -z "$1" ] || [[ $1 != day* ]]; then
  echo "Usage: $0 <day>"
  exit 1
fi

DAY=$1
ROOT="."

if [ -e $ROOT/input/$DAY ] || [ -e $ROOT/src/bin/$DAY.rs ]; then
  echo "Day $DAY already exists"
  exit 1
fi

mkdir -p $ROOT/input/$DAY
touch $ROOT/input/$DAY/input.txt
mkdir -p $ROOT/src/bin
cat > $ROOT/src/bin/$1.rs <<TEMPLATE
use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("$1");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![], //vec![("test0.txt", String::from("0"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<&String> = input.iter().collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, _: &[&String]) -> i32 {
        0
    }
}
TEMPLATE
