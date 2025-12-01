use crate::problem::Problem;

pub struct DayOne;

use std::collections::HashMap;
use regex::Regex;

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let mut total = 0;

        for line in input.lines() {
            let digits: Vec<char> = line.chars().filter(|&c| c.is_digit(10)).collect();
            let number = format!("{}{}", digits.first().unwrap_or(&'0'), digits.last().unwrap_or(&'0'))
                .parse::<u32>().unwrap();

            total += number;
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new("0|1|2|3|4|5|6|7|8|9|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();

        let mut map = HashMap::new();
        map.insert("zero", 0);
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);

        let mut total = 0;

        for line in input.lines() {
            let mut digits: Vec<usize> = Vec::new();
            let mut i = 0;

            while i < line.len() {
                if let Some(m) = re.find_at(line, i) {
                    if m.start() != i {
                        i += 1;
                        continue;
                    }

                    let s = m.as_str();

                    if s.len() == 1 {
                        digits.push(s.parse::<usize>().unwrap());
                    } else {
                        digits.push(*map.get(s).unwrap());
                    }
                }
                i += 1;
            }

            total += digits.first().unwrap() * 10 + digits.last().unwrap();
        }

        format!("{}", total)
    }
}
