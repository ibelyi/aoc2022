use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day14");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("24"))],
            Step::Second => vec![("test0.txt", String::from("93"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<Vec<usize>>> = input
            .iter()
            .map(|l| {
                l.split(" -> ")
                    .map(|d| d.split(',').map(|v| v.parse().unwrap()).collect())
                    .collect()
            })
            .collect();
        let mut max = vec![0usize; 2];
        let mut min = vec![1000usize; 2];
        for l in data.iter().flatten() {
            for i in 0..2 {
                if l[i] < min[i] {
                    min[i] = l[i];
                }
                if l[i] > max[i] {
                    max[i] = l[i];
                }
            }
        }
        if min[0] > 498 - max[1] {
            min[0] = 498 - max[1];
        }
        if max[0] < 502 + max[1] {
            max[0] = 502 + max[1];
        }
        let mut map = vec![vec!['.'; max[1] + 3]; max[0] - min[0] + 3];
        for l in data {
            let mut prev = &l[0];
            for d in l.iter().skip(1) {
                if prev[0] == d[0] {
                    for y in if prev[1] < d[1] {
                        prev[1]..=d[1]
                    } else {
                        d[1]..=prev[1]
                    } {
                        map[d[0] - min[0] + 1][y] = '#';
                    }
                } else {
                    for x in if prev[0] < d[0] {
                        prev[0]..=d[0]
                    } else {
                        d[0]..=prev[0]
                    } {
                        map[x - min[0] + 1][d[1]] = '#';
                    }
                }
                prev = d;
            }
        }
        if let Step::Second = step {
            let last = map[0].len() - 1;
            for l in map.iter_mut() {
                l[last] = '#';
            }
        }
        self.count(&mut map, 501 - min[0]).to_string()
    }
}

impl Solution {
    fn count(&self, map: &mut [Vec<char>], src: usize) -> usize {
        let mut sand = vec![src, 0];
        loop {
            sand[1] += 1;
            if map[sand[0]][sand[1]] != '.' {
                if map[sand[0] - 1][sand[1]] == '.' {
                    sand[0] -= 1;
                } else if map[sand[0] + 1][sand[1]] == '.' {
                    sand[0] += 1;
                } else {
                    map[sand[0]][sand[1] - 1] = 'o';
                    if sand[1] == 1 {
                        break;
                    }
                    sand = vec![src, 0];
                }
            }
            if sand[1] + 1 >= map[0].len() {
                break;
            }
        }
        map.iter().flatten().filter(|c| **c == 'o').count()
    }
}
