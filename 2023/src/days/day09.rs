use crate::problem::Problem;

pub struct DayNine;

use regex::Regex;

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"(-?\d+)").unwrap();

        let mut total = 0;

        for line in input.lines() {
            let mut numbers: Vec<Vec<i32>> = vec![re.captures_iter(line)
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .collect()];

            let mut i = 0;

            while numbers[i].iter().any(|&x| x != 0) {
                let diffrences = &numbers[i].windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();

                numbers.push(diffrences.clone());
                i += 1;
            }

            while i > 0 {
                let new = numbers[i].last().unwrap_or(&0) + numbers[i - 1].last().unwrap_or(&0);
                numbers[i - 1].push(new);
                i -= 1;
            }

            total += numbers[0].last().unwrap();
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new(r"(-?\d+)").unwrap();

        let mut total = 0;

        for line in input.lines() {
            let mut numbers: Vec<Vec<i32>> = vec![re.captures_iter(line)
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .collect()];

            let mut i = 0;

            while numbers[i].iter().any(|&x| x != 0) {
                let diffrences = &numbers[i].windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();

                numbers.push(diffrences.clone());
                i += 1;
            }

            while i > 0 {
                let new = numbers[i - 1].first().unwrap_or(&0) - numbers[i].first().unwrap_or(&0);
                numbers[i - 1].insert(0, new);
                i -= 1;
            }

            total += numbers[0].first().unwrap();
        }

        format!("{}", total)
    }
}
