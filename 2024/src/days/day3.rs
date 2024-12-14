use crate::problem::Problem;

use regex::Regex;

pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let matches = re.captures_iter(input).map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        }).sum::<i32>();

        format!("{}", matches)
    }

    fn part_two(&self, input: &str) -> String {
        let parts: Vec<&str> = input
            .split("do()")
            .map(|chunk| chunk.splitn(2, "don't()").next().unwrap_or("").trim())
            .collect();

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let matches = parts.into_iter().flat_map(|part| {
            re.captures_iter(part).map(|cap| {
                cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
            })
        }).sum::<i32>();

        format!("{}", matches)
    }
}
