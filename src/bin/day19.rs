use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day19");
}
struct Solution {}

struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: Vec<usize>,
    geode: Vec<usize>,
}

impl Blueprint {
    fn new(line: &str) -> Blueprint {
        let mut split = line
            .split("Blueprint ")
            .last()
            .unwrap()
            .split(": Each ore robot costs ");
        let id = split.next().unwrap().parse().unwrap();
        let mut split = split.next().unwrap().split(" ore. Each clay robot costs ");
        let ore = split.next().unwrap().parse().unwrap();
        let mut split = split
            .next()
            .unwrap()
            .split(" ore. Each obsidian robot costs ");
        let clay = split.next().unwrap().parse().unwrap();
        let mut split = split
            .next()
            .unwrap()
            .split(" clay. Each geode robot costs ");

        let obsidian = split
            .next()
            .unwrap()
            .split(" ore and ")
            .map(|v| v.parse().unwrap())
            .collect();
        let mut split = split.next().unwrap().split(" ore and ");
        let mut geode = vec![split.next().unwrap().parse().unwrap()];
        let mut split = split.next().unwrap().split(" obsidian.");
        geode.push(split.next().unwrap().parse().unwrap());
        Blueprint {
            id,
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("33"))],
            Step::Second => vec![("test0.txt", String::from("3472"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Blueprint> = input.iter().map(|l| Blueprint::new(l)).collect();
        self.count(&data, step).to_string()
    }
}

#[derive(Clone, Copy, Debug)]
struct Stock {
    ore: usize,
    clay: usize,
    obsidian: usize,
    ore_r: usize,
    clay_r: usize,
    obs_r: usize,
    geo_r: usize,
}

#[derive(Clone, Copy, Debug)]
enum Robots {
    None,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn choices(blue: &Blueprint, stock: &Stock, time: usize) -> usize {
    if time == 1 {
        return stock.geo_r;
    }
    let mut select = vec![];
    if blue.geode[0] <= stock.ore && blue.geode[1] <= stock.obsidian {
        select.push(Robots::Geode);
    } else {
        let obsidian_needed = if stock.obs_r != 0 && blue.geode[1] > stock.obsidian {
            ((blue.geode[1] - stock.obsidian + stock.obs_r - 1) / stock.obs_r) * stock.ore_r
        } else {
            0
        } + stock.ore;
        if blue.clay <= stock.ore
            && (stock.obs_r == 0 || obsidian_needed >= blue.geode[0] + blue.clay)
        {
            select.push(Robots::Clay);
        }
        if blue.obsidian[0] <= stock.ore
            && blue.obsidian[1] <= stock.clay
            && (stock.obs_r == 0 || obsidian_needed >= blue.geode[0] + blue.obsidian[0])
        {
            select.push(Robots::Obsidian);
        } else {
            let clay_needed = if stock.clay_r != 0 && blue.obsidian[1] > stock.clay {
                ((blue.obsidian[1] - stock.clay + stock.clay_r - 1) / stock.clay_r) * stock.ore_r
            } else {
                0
            } + stock.ore;
            if blue.ore <= stock.ore
                && (stock.clay_r == 0 || clay_needed >= blue.obsidian[0] + blue.ore)
            {
                select.push(Robots::Ore);
            } else {
                select.push(Robots::None);
            }
        }
    }
    let mut result = 0;
    for r in select {
        let mut stock = *stock;
        stock.ore += stock.ore_r;
        stock.clay += stock.clay_r;
        stock.obsidian += stock.obs_r;
        match r {
            Robots::Geode => {
                stock.ore -= blue.geode[0];
                stock.obsidian -= blue.geode[1];
                stock.geo_r += 1;
            }
            Robots::Obsidian => {
                stock.ore -= blue.obsidian[0];
                stock.clay -= blue.obsidian[1];
                stock.obs_r += 1;
            }
            Robots::Clay => {
                stock.ore -= blue.clay;
                stock.clay_r += 1;
            }
            Robots::Ore => {
                stock.ore -= blue.ore;
                stock.ore_r += 1;
            }
            Robots::None => {}
        }
        let res = choices(blue, &stock, time - 1);
        if res > result {
            result = res;
        }
    }
    result + stock.geo_r
}

fn analyze(blue: &Blueprint, time: usize) -> usize {
    let stock = Stock {
        ore: 0,
        clay: 0,
        obsidian: 0,
        ore_r: 1,
        clay_r: 0,
        obs_r: 0,
        geo_r: 0,
    };
    choices(blue, &stock, time)
}

impl Solution {
    fn count(&self, list: &[Blueprint], step: &Step) -> usize {
        match step {
            Step::First => list.iter().map(|b| b.id * analyze(b, 24)).sum(),
            Step::Second => list.iter().take(3).map(|b| analyze(b, 32)).product(),
        }
    }
}
