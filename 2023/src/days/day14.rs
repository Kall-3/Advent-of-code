use crate::problem::Problem;

pub struct DayFourteen;

use std::collections::HashMap;

fn move_rocks(platform: &mut Vec<Vec<char>>) {
    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                // move rock (if possible)
                platform[i][j] = '.';

                let mut n = i;
                while n > 0 && platform[n - 1][j] == '.' {
                    n -= 1;
                }

                platform[n][j] = 'O';
            }
        }
    }
}

fn cycle_rocks(platform: &mut Vec<Vec<char>>) {
    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                platform[i][j] = '.';
                let mut n = i;
                while n > 0 && platform[n - 1][j] == '.' { n -= 1; }
                platform[n][j] = 'O';
            }
        }
    }

    for j in 0..platform[0].len() {
        for i in 0..platform.len() {
            if platform[i][j] == 'O' {
                platform[i][j] = '.';
                let mut n = j;
                while n > 0 && platform[i][n - 1] == '.' { n -= 1; }
                platform[i][n] = 'O';
            }
        }
    }

    for i in (0..platform.len()).rev() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                platform[i][j] = '.';
                let mut n = i;
                while n + 1 < platform.len() && platform[n + 1][j] == '.' { n += 1; }
                platform[n][j] = 'O';
            }
        }
    }

    for j in (0..platform[0].len()).rev() {
        for i in 0..platform.len() {
            if platform[i][j] == 'O' {
                platform[i][j] = '.';
                let mut n = j;
                while n + 1 < platform[i].len() && platform[i][n + 1] == '.' { n += 1; }
                platform[i][n] = 'O';
            }
        }
    }
}

fn get_key(platform: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    platform.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'O')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let mut platform: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        move_rocks(&mut platform);

        let mut total = 0;

        for (i, row) in platform.iter().rev().enumerate() {
            total += row.iter().fold(0, |acc, &c| if c == 'O' { acc + 1 + i } else { acc });
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let mut platform: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut seen: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();

        for i in 1..=1_000_000_000 {
            cycle_rocks(&mut platform);
            let key = get_key(&platform);

            // detect cycles
            if let Some(&cycle_length) = seen.get(&key) {
                let remaining = (1_000_000_000 - i) % (i - cycle_length);
                for _ in 0..remaining {
                    cycle_rocks(&mut platform);
                }
                break;
            } else {
                seen.insert(key, i);
            }
        }

        let mut total = 0;

        for (i, row) in platform.iter().rev().enumerate() {
            total += row.iter().fold(0, |acc, &c| if c == 'O' { acc + 1 + i } else { acc });
        }

        format!("{}", total)
    }
}
