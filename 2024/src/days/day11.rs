use crate::problem::Problem;

use std::collections::HashMap;

fn solve(stones: &mut HashMap<usize, usize>, blinks: u32) -> usize {
    for _ in 0..blinks {
        for (stone, count) in stones.drain().collect::<Vec<_>>() {
            let mut insert = |s| {
                stones.entry(s).and_modify(|c| *c += count).or_insert(count);
            };
            if stone == 0 {
                insert(1);
            } else {
                match (stone as f32).log10().floor() as usize + 1 {
                    digits if digits % 2 == 0 => {
                        insert(stone / 10_usize.pow((digits / 2) as u32));
                        insert(stone % 10_usize.pow((digits / 2) as u32));
                    }
                    _ => {
                        insert(stone * 2024);
                    }
                }
            }
        }
    }
    stones.values().sum::<usize>()
}

pub struct DayEleven {}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let mut stones: HashMap<usize, usize> = input
            .trim()
            .split_whitespace()
            .map(|s| (s.parse::<usize>().unwrap(), 1_usize))
            .collect();

        format!("{}", solve(&mut stones, 25))
    }

    fn part_two(&self, input: &str) -> String {
        let mut stones: HashMap<usize, usize> = input
            .trim()
            .split_whitespace()
            .map(|s| (s.parse::<usize>().unwrap(), 1_usize))
            .collect();

        format!("{}", solve(&mut stones, 75))
    }
}
