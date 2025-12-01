use crate::problem::Problem;

pub struct DayTwo;

use regex::Regex;

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let mut total = 0;

        let re_game = Regex::new(r"Game (\d+):").unwrap();

        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();

        for line in input.lines() {
            let mut possible = true;
            let game = re_game.captures(line)
                .and_then(|m| m[1].parse::<usize>().ok())
                .expect("Failed to parse game number");

            for throw in line.split(';') {
                if !possible {
                    continue;
                }

                if let Some(m) = re_red.captures(throw) {
                    if m[1].parse::<usize>().unwrap() > 12 {
                        possible = false;
                        continue;
                    }
                }
                if let Some(m) = re_green.captures(throw) {
                    if m[1].parse::<usize>().unwrap() > 13 {
                        possible = false;
                        continue;
                    }
                }
                if let Some(m) = re_blue.captures(throw) {
                    if m[1].parse::<usize>().unwrap() > 14 {
                        possible = false;
                        continue;
                    }
                }
            }

            if possible {
                total += game;
            }
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let mut total = 0;

        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();

        for line in input.lines() {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for throw in line.split(';') {
                if let Some(m) = re_red.captures(throw) {
                    max_red = max_red.max(m[1].parse::<usize>().unwrap());
                }
                if let Some(m) = re_green.captures(throw) {
                    max_green = max_green.max(m[1].parse::<usize>().unwrap());
                }
                if let Some(m) = re_blue.captures(throw) {
                    max_blue = max_blue.max(m[1].parse::<usize>().unwrap());
                }
            }

            total += max_red.max(1) * max_green.max(1) * max_blue.max(1);
        }

        format!("{}", total)
    }
}
