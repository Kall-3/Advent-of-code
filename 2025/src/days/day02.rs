use crate::problem::Problem;

pub struct DayTwo;

use regex::Regex;

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {

        let re = Regex::new(r"(?P<start>\d+)-(?P<end>\d+)").unwrap();

        let mut total: usize = 0;

        for caps in re.captures_iter(input) {
            let start: usize = caps["start"].parse().unwrap();
            let end: usize = caps["end"].parse().unwrap();

            for i in start..=end {
                let string_i = i.to_string();
                let len = string_i.len();
                let middle = len / 2;

                let first_half = &string_i[..middle];
                let second_half = &string_i[middle..];

                if first_half == second_half {
                    total += i as usize;
                }
            }
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {

        let re = Regex::new(r"(?P<start>\d+)-(?P<end>\d+)").unwrap();

        let mut total: usize = 0;

        for caps in re.captures_iter(input) {
            let start: usize = caps["start"].parse().unwrap();
            let end: usize = caps["end"].parse().unwrap();

            for i in start..=end {
                let string_i = i.to_string();
                let len = string_i.len();
                let middle = len / 2;

                for j in 1..=middle {
                    if len % j != 0 {
                        continue;
                    }

                    let template = &string_i[0..j];
                    let remaining: Vec<&str> = string_i.split(template).collect();

                    if remaining.iter().all(|&s| s.is_empty()) {
                        total += i as usize;
                        break;
                    }
                }
            }
        }

        format!("{}", total)
    }
}
