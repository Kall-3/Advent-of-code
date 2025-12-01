use crate::problem::Problem;

pub struct DayTwelve;

use std::collections::HashMap;

fn dfs(
    i: usize,
    gi: usize,
    run: usize,
    pattern: &[u8],
    groups: &[usize],
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&ans) = memo.get(&(i, gi, run)) {
        return ans;
    }

    if i == pattern.len() {
        if run > 0 {
            if gi + 1 == groups.len() && run == groups[gi] {
                return 1;
            } else {
                return 0;
            }
        }
        if gi == groups.len() {
            return 1;
        } else {
            return 0;
        }
    }

    let ch = pattern[i];
    let mut ways: usize = 0;

    // treat position as '.'
    if ch == b'.' || ch == b'?' {
        if run > 0 {
            // end current group
            if gi < groups.len() && run == groups[gi] {
                ways += dfs(i + 1, gi + 1, 0, pattern, groups, memo);
            }
        } else {
            // continue with no group
            ways += dfs(i + 1, gi, 0, pattern, groups, memo);
        }
    }

    // treat position as '#'
    if ch == b'#' || ch == b'?' {
        if gi < groups.len() && run + 1 <= groups[gi] {
            // continue current group
            ways += dfs(i + 1, gi, run + 1, pattern, groups, memo);
        }
    }

    memo.insert((i, gi, run), ways);
    ways
}

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let mut total: usize = 0;

        for line in input.lines() {
            let (records, groups) = line.split_once(" ").unwrap();

            let springs: Vec<u8> = records.bytes().collect();
            let group_sizes: Vec<usize> = groups.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();

            let arrangements = dfs(
                0,
                0,
                0,
                &springs,
                &group_sizes,
                &mut memo,
            );

            total += arrangements;
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let mut total: usize = 0;

        for line in input.lines() {
            let (records, groups) = line.split_once(" ").unwrap();

            let springs: Vec<u8> = (0..5).map(|_| records)
                .fold(String::new(), |mut acc, s| {
                    acc.push_str(s);
                    acc.push_str("?");
                    acc
                })
                .bytes()
                .take(records.len() * 5 + 4)
                .collect();
            let group_sizes: Vec<usize> = (0..5).flat_map(|_| {
                    groups.split(",").map(|s| s.parse::<usize>().unwrap())
                })
                .collect();
            let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();

            let arrangements = dfs(
                0,
                0,
                0,
                &springs,
                &group_sizes,
                &mut memo,
            );

            total += arrangements;
        }

        format!("{}", total)
    }
}
