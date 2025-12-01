use crate::problem::Problem;

pub struct DayFive;

use regex::Regex;

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let mut input = input.lines();

        let re = Regex::new(r"(\d+)").unwrap();

        let seeds: Vec<u32> = re.find_iter(input.next().unwrap())
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let re = Regex::new(r"^(\d+)\s+(\d+)\s+(\d+)$").unwrap();

        let mut map: Vec<Vec<(usize, usize, i64)>> = vec![Vec::new(); 7];
        let mut i = 0;

        // skip 2 lines
        input.next(); input.next();

        for line in input {
            if let Some(caps) = re.captures(line) {
                let destination_start = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let source_start = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let range = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

                let source_end = source_start + range;
                let offset = (destination_start as i64) - (source_start as i64);

                map[i/2].push((source_start, source_end, offset));
            } else {
                i += 1;
            }
        }

        let mut min = i64::MAX;

        for seed in seeds {
            let mut value = seed as i64;

            for operations in &map {
                for (source_start, source_end, offset) in operations {
                    if value >= *source_start as i64 && value < *source_end as i64 {
                        value += *offset;
                        break;
                    }
                }
            }

            min = min.min(value);
        }

        format!("{}", min)
    }

    fn part_two(&self, input: &str) -> String {
        let mut input = input.lines();

        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

        let mut seeds: Vec<(usize, usize)> = Vec::new();

        for caps in re.captures_iter(input.next().unwrap()) {
            let seeds_start = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let seeds_range = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

            seeds.push((seeds_start, seeds_start + seeds_range - 1));
        }

        let re = Regex::new(r"^(\d+)\s+(\d+)\s+(\d+)$").unwrap();

        // skip 2
        input.next(); input.next();
        
        let mut rules: Vec<(usize, usize, usize)> = Vec::new();

        for line in input {
            if let Some(caps) = re.captures(line) {
                let destination_start = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let source_start = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let range = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

                rules.push((destination_start, source_start, range));
            } else {
                seeds = apply_map_block(seeds, &rules);
                rules.clear();
            }
        }

        if !rules.is_empty() {
            seeds = apply_map_block(seeds, &rules);
        }

        let mut min = usize::MAX;
        for (seed_start, _) in seeds.iter() {
            min = min.min(*seed_start);
        }
        format!("{}", min)
    }
}

fn apply_map_block(mut seeds: Vec<(usize, usize)>, rules: &[(usize, usize, usize)]) -> Vec<(usize, usize)> {
    let mut mapped: Vec<(usize, usize)> = Vec::new();

    for (destination_start, source_start, range) in rules {
        let source_end = source_start + range;
        let offset = *destination_start as isize - *source_start as isize;

        let mut next_unmapped: Vec<(usize, usize)> = Vec::new();

        for (seed_start, seed_end) in seeds.into_iter() {
            // no overlap -> stay unmapped for later changed (rules)
            if seed_end < *source_start || seed_start >= source_end {
                next_unmapped.push((seed_start, seed_end));
                continue;
            }

            // overlap -> split and map middle
            if seed_start < *source_start {
                next_unmapped.push((seed_start, *source_start - 1));
            }

            let overlap_start = (seed_start).max(*source_start);
            let overlap_end = (seed_end).min(source_end - 1);

            let new_start = (overlap_start as isize + offset) as usize;
            let new_end = (overlap_end as isize + offset) as usize;
            mapped.push((new_start, new_end));

            if seed_end >= source_end {
                next_unmapped.push((source_end, seed_end));
            }
        }

        seeds = next_unmapped;
    }

    mapped.extend(seeds);
    mapped
}
