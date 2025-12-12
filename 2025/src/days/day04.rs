use std::collections::{HashSet, VecDeque};

use crate::problem::Problem;

pub struct DayFour;

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {

        let map: HashSet<(i32, i32)> = input.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '@' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            }).collect();

        let dirs = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

        let mut total = 0;

        for i in 0..input.lines().count() as i32 {
            for j in 0..input.lines().next().unwrap().chars().count() as i32 {

                let mut count = 0;

                for (dx, dy) in dirs.iter() {
                    if map.contains(&(i + dx, j + dy)) {
                        count += 1;
                    }
                }
                if count < 4 && map.contains(&(i, j)) {
                    total += 1;
                }
            }
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {

        let mut map: HashSet<(i32, i32)> = input.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '@' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            }).collect();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::from_iter(map.iter().cloned());

        let dirs = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

        let mut total = 0;
        let mut break_cond = map.len();

        while let Some((i, j)) = queue.pop_front() {
            if break_cond == 0 {
                break;
            }
            break_cond -= 1;

            let mut count = 0;
            for (dx, dy) in dirs.iter() {
                if map.contains(&(i + dx, j + dy)) {
                    count += 1;
                }
            }
            if count < 4 && map.contains(&(i, j)) {
                map.remove(&(i, j));
                total += 1;

                break_cond = map.len();
            } else {
                queue.push_back((i, j));
            }
        }

        format!("{}", total)
    }
}
