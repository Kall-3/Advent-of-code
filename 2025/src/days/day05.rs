use crate::problem::Problem;

pub struct DayFive;

use std::ops::{RangeInclusive};
use regex::Regex;

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {

        let (ranges_input, ingredients_input) = input.split_once("\n\n").unwrap();

        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();

        for cap in re.captures_iter(ranges_input) {
            let start: usize = cap[1].parse().unwrap();
            let end: usize = cap[2].parse().unwrap();
            ranges.push(RangeInclusive::new(start, end));
        }

        let mut ans: u32 = 0;

        for line in ingredients_input.lines() {
            let value: usize = line.parse().unwrap();
            for range in &ranges {
                if range.contains(&value) {
                    ans += 1;
                    break;
                }
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {
        let (ranges_input, _) = input.split_once("\n\n").unwrap();

        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();

        for cap in re.captures_iter(ranges_input) {
            let start: usize = cap[1].parse().unwrap();
            let end: usize = cap[2].parse().unwrap();
            ranges.push(RangeInclusive::new(start, end));
        }

        let mut overlaps: Vec<RangeInclusive<usize>> = Vec::new();
        overlaps.push(ranges.pop().unwrap());

        while let Some(range1) = ranges.pop() {
            let mut has_overlap = false;
            for i in 0..overlaps.len() {
                let range2 = &overlaps[i];

                if range1.end() < range2.start() || range2.end() < range1.start() {
                    continue;
                }

                let start = range1.start().min(range2.start());
                let end = range1.end().max(range2.end());

                ranges.push(RangeInclusive::new(*start, *end));
                overlaps.remove(i);
                has_overlap = true;
                break;
            }

            if !has_overlap {
                overlaps.push(range1);
            }
        }

        format!("{}", overlaps.iter().map(|r| r.end() - r.start() + 1).sum::<usize>())
    }
}
