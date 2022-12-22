use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day22");
}
struct Solution {}

enum Path {
    Number(usize),
    Left,
    Right,
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("6032"))],
            Step::Second => vec![("test0.txt", String::from("5031"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data = input.iter();
        let mut map = vec![];
        let mut path = vec![];
        loop {
            if let Some(line) = data.next() {
                if !line.is_empty() {
                    map.push(line.chars().collect());
                } else if let Some(p) = data.next() {
                    let mut number = vec![];
                    for c in p.chars() {
                        if c.is_digit(10) {
                            number.push(c);
                        } else {
                            if !number.is_empty() {
                                path.push(Path::Number(
                                    number.iter().collect::<String>().parse().unwrap(),
                                ));
                                number.clear();
                            }
                            if c == 'L' {
                                path.push(Path::Left);
                            } else if c == 'R' {
                                path.push(Path::Right);
                            } else {
                                panic!("Unknown step {}", c);
                            }
                        }
                    }
                    if !number.is_empty() {
                        path.push(Path::Number(
                            number.iter().collect::<String>().parse().unwrap(),
                        ));
                    }
                    break;
                } else {
                    panic!("No password!");
                }
            } else {
                panic!("Ended too early!");
            }
        }
        match step {
            Step::First => self.count(&map, &path).to_string(),
            Step::Second => self.count2(&map, &path).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, map: &[Vec<char>], path: &[Path]) -> usize {
        let mut curr = (0, 0, 0);
        for c in &map[0] {
            if *c == ' ' {
                curr.1 += 1;
            } else if *c == '.' {
                break;
            } else {
                panic!("Invalid starting entry: {}", c);
            }
        }
        for s in path {
            match s {
                Path::Left => curr.2 = (curr.2 + 3) % 4,
                Path::Right => curr.2 = (curr.2 + 1) % 4,
                Path::Number(n) => {
                    for _ in 1..=*n {
                        let next = if curr.2 & 1 == 0 {
                            let mv = if curr.2 == 0 {
                                1
                            } else {
                                map[curr.0].len() - 1
                            };
                            let mut x = curr.1;
                            loop {
                                x = (x + mv) % map[curr.0].len();
                                if map[curr.0][x] != ' ' {
                                    break;
                                }
                            }
                            (curr.0, x)
                        } else {
                            let mv = if curr.2 == 1 { 1 } else { map.len() - 1 };
                            let mut y = curr.0;
                            loop {
                                loop {
                                    y = (y + mv) % map.len();
                                    if curr.1 < map[y].len() {
                                        break;
                                    }
                                }
                                if map[y][curr.1] != ' ' {
                                    break;
                                }
                            }
                            (y, curr.1)
                        };
                        if map[next.0][next.1] == '#' {
                            break;
                        } else {
                            curr.0 = next.0;
                            curr.1 = next.1;
                        }
                    }
                }
            }
        }
        1000 * (curr.0 + 1) + 4 * (curr.1 + 1) + curr.2
    }

    fn count2(&self, map: &[Vec<char>], path: &[Path]) -> usize {
        let side = if map.len() < 20 { 4 } else { 50 };
        let mut curr = (0, 0, 0);
        for c in &map[0] {
            if *c == ' ' {
                curr.1 += 1;
            } else if *c == '.' {
                break;
            } else {
                panic!("Invalid starting entry: {}", c);
            }
        }
        for s in path {
            match s {
                Path::Left => curr.2 = (curr.2 + 3) % 4,
                Path::Right => curr.2 = (curr.2 + 1) % 4,
                Path::Number(n) => {
                    for _ in 1..=*n {
                        let next = if curr.2 == 0 {
                            if curr.1 == map[curr.0].len() - 1 {
                                if side == 4 {
                                    if curr.0 < 4 {
                                        (11 - curr.0, 15, 2)
                                    } else if curr.0 < 8 {
                                        (8, 19 - curr.0, 1)
                                    } else {
                                        (11 - curr.0, 11, 0)
                                    }
                                } else if curr.0 < 50 {
                                    (149 - curr.0, 99, 2)
                                } else if curr.0 < 100 {
                                    (49, 50 + curr.0, 3)
                                } else if curr.0 < 150 {
                                    (149 - curr.0, 149, 2)
                                } else {
                                    (149, curr.0 - 100, 3)
                                }
                            } else {
                                (curr.0, curr.1 + 1, curr.2)
                            }
                        } else if curr.2 == 1 {
                            if curr.0 == map.len() - 1
                                || curr.1 > map[curr.0 + 1].len() - 1
                                || map[curr.0 + 1][curr.1] == ' '
                            {
                                if side == 4 {
                                    if curr.1 < 4 {
                                        (11, 11 - curr.1, 3)
                                    } else if curr.1 < 8 {
                                        (15 - curr.1, 8, 0)
                                    } else if curr.1 < 12 {
                                        (7, 11 - curr.1, 3)
                                    } else {
                                        (19 - curr.1, 0, 0)
                                    }
                                } else if curr.1 < 50 {
                                    (0, curr.1 + 100, 1)
                                } else if curr.1 < 100 {
                                    (curr.1 + 100, 49, 2)
                                } else {
                                    (curr.1 - 50, 99, 2)
                                }
                            } else {
                                (curr.0 + 1, curr.1, curr.2)
                            }
                        } else if curr.2 == 2 {
                            if curr.1 == 0 || map[curr.0][curr.1 - 1] == ' ' {
                                if side == 4 {
                                    if curr.0 < 4 {
                                        (4, 4 + curr.0, 1)
                                    } else if curr.0 < 8 {
                                        (11, 19 - curr.0, 3)
                                    } else {
                                        (7, 15 - curr.0, 3)
                                    }
                                } else if curr.0 < 50 {
                                    (149 - curr.0, 0, 0)
                                } else if curr.0 < 100 {
                                    (100, curr.0 - 50, 1)
                                } else if curr.0 < 150 {
                                    (149 - curr.0, 50, 0)
                                } else {
                                    (0, curr.0 - 100, 1)
                                }
                            } else {
                                (curr.0, curr.1 - 1, curr.2)
                            }
                        } else if curr.0 == 0
                            || curr.1 > map[curr.0 - 1].len()
                            || map[curr.0 - 1][curr.1] == ' '
                        {
                            if side == 4 {
                                if curr.1 < 4 {
                                    (0, 11 - curr.1, 1)
                                } else if curr.1 < 8 {
                                    (curr.1 - 4, 8, 0)
                                } else if curr.1 < 12 {
                                    (4, 11 - curr.1, 1)
                                } else {
                                    (19 - curr.1, 11, 2)
                                }
                            } else if curr.1 < 50 {
                                (curr.1 + 50, 50, 0)
                            } else if curr.1 < 100 {
                                (curr.1 + 100, 0, 0)
                            } else {
                                (199, curr.1 - 100, 3)
                            }
                        } else {
                            (curr.0 - 1, curr.1, curr.2)
                        };
                        if map[next.0][next.1] == '#' {
                            break;
                        } else {
                            curr = next;
                        }
                    }
                }
            }
        }
        1000 * (curr.0 + 1) + 4 * (curr.1 + 1) + curr.2
    }
}
