use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day24");
}
struct Solution {}

#[derive(Clone, Copy, PartialEq)]
enum Wind {
    Up,
    Right,
    Down,
    Left,
}

impl Wind {
    fn new(dir: char) -> Wind {
        match dir {
            '^' => Wind::Up,
            '>' => Wind::Right,
            'v' => Wind::Down,
            '<' => Wind::Left,
            w => panic!("Unknown wind {}", w),
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("18"))],
            Step::Second => vec![("test0.txt", String::from("54"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let start_x = input[0].chars().position(|c| c == '.').unwrap() - 1;
        let end_x = input[input.len() - 1]
            .chars()
            .position(|c| c == '.')
            .unwrap()
            - 1;
        let data: Vec<Vec<Vec<Wind>>> = input
            .iter()
            .skip(1)
            .take(input.len() - 2)
            .map(|l| {
                l.chars()
                    .filter(|c| *c != '#')
                    .map(|c| if c == '.' { vec![] } else { vec![Wind::new(c)] })
                    .collect()
            })
            .collect();
        self.count(&data, start_x, end_x, step).to_string()
    }
}

fn move_wind(map: &[Vec<Vec<Wind>>]) -> Vec<Vec<Vec<Wind>>> {
    let (len_y, len_x) = (map.len(), map[0].len());
    (0..len_y)
        .map(|y| {
            (0..len_x)
                .map(|x| {
                    [
                        (-1, 0, Wind::Down),
                        (0, 1, Wind::Left),
                        (1, 0, Wind::Up),
                        (0, -1, Wind::Right),
                    ]
                    .iter()
                    .filter(|(dy, dx, w)| {
                        map[((y + len_y) as i32 + dy) as usize % len_y]
                            [((x + len_x) as i32 + dx) as usize % len_x]
                            .contains(w)
                    })
                    .map(|(_, _, w)| w)
                    .copied()
                    .collect()
                })
                .collect()
        })
        .collect()
}

const DIRS: [(i32, i32); 5] = [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];

fn make_steps(map: &mut Vec<Vec<Vec<Wind>>>, s: (i32, i32), e: (usize, usize)) -> usize {
    let (len_y, len_x) = (map.len(), map[0].len());
    let mut index = 0;
    let mut results = vec![vec![usize::MAX; len_x]; len_y];
    while results[e.0][e.1] == usize::MAX {
        *map = move_wind(map);
        index += 1;
        results = map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if c.is_empty() {
                            DIRS.iter()
                                .map(|(dy, dx)| {
                                    let (ny, nx) = (y as i32 + dy, x as i32 + dx);
                                    if ny == s.0 && nx == s.1 {
                                        index
                                    } else if ny < 0
                                        || nx < 0
                                        || ny >= len_y as i32
                                        || nx >= len_x as i32
                                        || results[ny as usize][nx as usize] == usize::MAX
                                    {
                                        usize::MAX
                                    } else {
                                        results[ny as usize][nx as usize] + 1
                                    }
                                })
                                .min()
                                .unwrap()
                        } else {
                            usize::MAX
                        }
                    })
                    .collect()
            })
            .collect();
    }
    *map = move_wind(map);
    results[e.0][e.1] + 1
}

impl Solution {
    fn count(&self, orig: &[Vec<Vec<Wind>>], sx: usize, ex: usize, step: &Step) -> usize {
        let mut map = orig.to_vec();
        let res = make_steps(&mut map, (-1, sx as i32), (orig.len() - 1, ex));
        match step {
            Step::First => res,
            Step::Second => {
                let res1 = make_steps(&mut map, (orig.len() as i32, ex as i32), (0, sx));
                let res2 = make_steps(&mut map, (-1, sx as i32), (orig.len() - 1, ex));
                res + res1 + res2
            }
        }
    }
}
