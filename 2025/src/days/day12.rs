use regex::Regex;

use crate::problem::Problem;

pub struct DayTwelve;

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let present_re = Regex::new(r"(?P<id>\d+):\n(?P<shape>([#.]{3}\n){3})").unwrap();
        let regions_re = Regex::new(r"(?P<width>\d+)x(?P<height>\d+): (?P<nbr_presents>[\d ]+)").unwrap();

        let mut presents: Vec<Vec<String>> = Vec::new();
        let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

        for cap in present_re.captures_iter(input) {
            let _id: usize = cap["id"].parse().unwrap();
            let shape: Vec<String> = cap["shape"].lines()
                .map(|line| line.to_string())
                .collect();
            presents.push(shape);
        }
        for cap in regions_re.captures_iter(input) {
            let width: usize = cap["width"].parse().unwrap();
            let height: usize = cap["height"].parse().unwrap();
            let nbr_presents: Vec<usize> = cap["nbr_presents"].split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push((width, height, nbr_presents));
        }

        let mut ans: usize = 0;
        for (width, height, nbr_presents) in regions {
            // guessed that each 2 presents need 4x4 space
            let present_count: usize = nbr_presents.iter().sum();
            if ((present_count / 2) + present_count % 2) * 4 * 4 <= width * height {
                ans += 1;
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, _input: &str) -> String {
        format!("No part 2!")
    }
}
