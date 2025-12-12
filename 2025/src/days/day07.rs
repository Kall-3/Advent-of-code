use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

pub struct DaySeven;

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let x_start = map[0].iter().position(|&c| c == 'S').unwrap();

        let mut beams: HashSet<(usize, usize)> = HashSet::new();
        beams.insert((x_start, 0));
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();

        let mut splits: usize = 0;

        while !beams.is_empty() {
            for (x, y) in beams.drain() {
                let below = map.get(y + 1).and_then(|row| row.get(x));
                if Some(&'^') == below {
                    new_beams.insert((x - 1, y + 1));
                    new_beams.insert((x + 1, y + 1));
                    splits += 1;
                } else if Some(&'.') == below {
                    new_beams.insert((x, y + 1));
                }
            }
            std::mem::swap(&mut beams, &mut new_beams);
        }

        format!("{}", splits)
    }

    fn part_two(&self, input: &str) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let x_start = map[0].iter().position(|&c| c == 'S').unwrap();

        // (x position, count of beams)
        let mut beams: HashMap<usize, usize> = HashMap::new();
        beams.insert(x_start, 1);
        let mut new_beams: HashMap<usize, usize> = HashMap::new();

        for y in 0..map.len() - 1 {
            for (x, count) in beams.drain() {
                let below = map.get(y + 1).and_then(|row| row.get(x));
                if Some(&'^') == below {
                    *new_beams.entry(x - 1).or_default() += count;
                    *new_beams.entry(x + 1).or_default() += count;
                } else if Some(&'.') == below {
                    *new_beams.entry(x).or_default() += count;
                }
            }
            std::mem::swap(&mut beams, &mut new_beams);
        }

        let new_worlds = beams.values().sum::<usize>();

        format!("{}", new_worlds)
    }
}
