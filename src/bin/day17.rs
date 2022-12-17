use aoc2022::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day17");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("3068"))],
            Step::Second => vec![("test0.txt", String::from("1514285714288"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<char> = input[0].chars().collect();
        match step {
            Step::First => self.count(&data, step).to_string(),
            Step::Second => self.count(&data, step).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, wind: &[char], step: &Step) -> u64 {
        let rocks = vec![
            vec![['.', '.', '@', '@', '@', '@', '.']],
            vec![
                ['.', '.', '.', '@', '.', '.', '.'],
                ['.', '.', '@', '@', '@', '.', '.'],
                ['.', '.', '.', '@', '.', '.', '.'],
            ],
            vec![
                ['.', '.', '@', '@', '@', '.', '.'],
                ['.', '.', '.', '.', '@', '.', '.'],
                ['.', '.', '.', '.', '@', '.', '.'],
            ],
            vec![
                ['.', '.', '@', '.', '.', '.', '.'],
                ['.', '.', '@', '.', '.', '.', '.'],
                ['.', '.', '@', '.', '.', '.', '.'],
                ['.', '.', '@', '.', '.', '.', '.'],
            ],
            vec![
                ['.', '.', '@', '@', '.', '.', '.'],
                ['.', '.', '@', '@', '.', '.', '.'],
            ],
        ];
        let mut field = vec![];
        let mut currw = 0;
        // let mut prev = 0;
        let end = match step {
            Step::First => 2022,
            Step::Second => 1000000000000,
        };
        let mut stat = vec![vec![0; rocks.len()]; wind.len()];
        let mut prev = vec![vec![(0, 0); rocks.len()]; wind.len()];
        let mut extra = 0;
        let mut i = 0;
        while i < end {
            let curr = i % rocks.len();
            stat[currw][curr] += 1;
            if extra == 0 && stat[currw][curr] > 2 {
                extra = (field.len() - prev[currw][curr].1) as u64
                    * ((end - i) / (i - prev[currw][curr].0)) as u64;
                i = end - (end - i) % (i - prev[currw][curr].0);
                if i == end {
                    break;
                }
            }
            prev[currw][curr] = (i, field.len());
            i += 1;
            for _ in 0..3 {
                field.push(vec!['.'; 7]);
            }
            for l in &rocks[curr] {
                field.push(l.to_vec());
            }
            let mut hight = field.len();
            loop {
                let mut can_move = true;
                match wind[currw] {
                    '>' => {
                        'checkR: for line in field
                            .iter()
                            .skip(hight - rocks[curr].len())
                            .take(rocks[curr].len())
                        {
                            for x in (0usize..7).rev() {
                                if line[x] == '@' {
                                    if x == 6 || line[x + 1] == '#' {
                                        can_move = false;
                                        break 'checkR;
                                    }
                                    break;
                                }
                            }
                        }
                        if can_move {
                            for line in field
                                .iter_mut()
                                .skip(hight - rocks[curr].len())
                                .take(rocks[curr].len())
                            {
                                for x in (0usize..6).rev() {
                                    if line[x] == '@' {
                                        line[x + 1] = '@';
                                        line[x] = '.';
                                    }
                                }
                            }
                        }
                    }
                    '<' => {
                        'checkL: for line in field
                            .iter()
                            .skip(hight - rocks[curr].len())
                            .take(rocks[curr].len())
                        {
                            for x in 0usize..7 {
                                if line[x] == '@' {
                                    if x == 0 || line[x - 1] == '#' {
                                        can_move = false;
                                        break 'checkL;
                                    }
                                    break;
                                }
                            }
                        }
                        if can_move {
                            for line in field
                                .iter_mut()
                                .skip(hight - rocks[curr].len())
                                .take(rocks[curr].len())
                            {
                                for x in 1usize..7 {
                                    if line[x] == '@' {
                                        line[x - 1] = '@';
                                        line[x] = '.';
                                    }
                                }
                            }
                        }
                    }
                    _ => panic!("Unknown wind!"),
                }
                currw = (currw + 1) % wind.len();

                can_move = true;
                'checkB: for y in hight - rocks[curr].len()..hight {
                    for x in 0usize..7 {
                        if field[y][x] == '@' && (y == 0 || field[y - 1][x] == '#') {
                            can_move = false;
                            break 'checkB;
                        }
                    }
                }
                if can_move {
                    for y in hight - rocks[curr].len()..hight {
                        for x in 0usize..7 {
                            if field[y][x] == '@' {
                                field[y - 1][x] = '@';
                                field[y][x] = '.';
                            }
                        }
                    }
                    let mut cut = 0usize;
                    for line in field.iter().rev() {
                        if line.iter().any(|v| *v != '.') {
                            break;
                        }
                        cut += 1;
                    }
                    for _ in 0..cut {
                        field.pop();
                    }
                    hight -= 1;
                } else {
                    for line in field
                        .iter_mut()
                        .skip(hight - rocks[curr].len())
                        .take(rocks[curr].len())
                    {
                        for v in line.iter_mut() {
                            if *v == '@' {
                                *v = '#';
                            }
                        }
                    }
                    break;
                }
            }
        }
        extra + field.len() as u64
    }
}
