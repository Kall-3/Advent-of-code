use std::collections::HashMap;

use crate::problem::Problem;

pub struct DayThree;

use regex::Regex;

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let (rows, cols) = (input.lines().count(), input.lines().next().unwrap().len());

        let mut total = 0;

        let re = Regex::new(r"(\d+)").unwrap();

        for (i, line) in input.lines().enumerate() {
            for m in re.find_iter(line) {
                let num = m.as_str().parse::<usize>().unwrap();
                let start = m.start();
                let end = m.end();

                'outer: for x in (i.max(1)-1)..=(i+1) {
                    for y in (start.max(1)-1)..=end {
                        if x >= rows || y >= cols {
                            continue;
                        }
                        if x == i && (start <= y && y < end) {
                            continue;
                        }
                        let c = input.lines().nth(x).unwrap().chars().nth(y).unwrap();
                        if !(c.is_digit(10) || c == '.') {
                            total += num;
                            break 'outer;
                        }
                    }
                }
            }
        }
        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {

        let mut parts: HashMap<(usize, usize), u32> = HashMap::new();
        let mut parts_num: HashMap<u32, usize> = HashMap::new();
        let mut part_id = 0;


        let re = Regex::new(r"(\d+)").unwrap();

        for (i, line) in input.lines().enumerate() {
            for m in re.find_iter(line) {
                let num = m.as_str().parse::<usize>().unwrap();
                let start = m.start();
                let end = m.end();

                for j in start..end {
                    parts.insert((i, j), part_id);
                }
                parts_num.insert(part_id, num);

                part_id += 1;
            }
        }

        let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut total = 0;

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j].is_digit(10) || input[i][j] == '.' {
                    continue;
                }

                let mut part_1 = None;
                for x in (i.max(1)-1)..=(i+1) {
                    for y in (j.max(1)-1)..=(j+1) {
                        if x >= input.len() || y >= input[0].len() {
                            continue;
                        }
                        if let Some(part) = parts.get(&(x, y)) {
                            part_1 = Some(*part);
                        }
                    }
                }

                let mut part_2 = None;
                for x in (i.max(1)-1)..=(i+1) {
                    for y in (j.max(1)-1)..=(j+1) {
                        if x >= input.len() || y >= input[0].len() {
                            continue;
                        }
                        if let Some(part) = parts.get(&(x, y)) {
                            if Some(*part) != part_1 {
                                part_2 = Some(*part);
                            }
                        }
                    }
                }

                if let (Some(p1), Some(p2)) = (part_1, part_2) {
                    let num1 = parts_num.get(&p1).unwrap();
                    let num2 = parts_num.get(&p2).unwrap();
                    total += num1 * num2;
                }
            }
        }
        format!("{}", total)
    }
}
