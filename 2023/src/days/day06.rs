use crate::problem::Problem;

pub struct DaySix;

use regex::Regex;

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)").unwrap();

        let mut times: Vec<u32> = Vec::new();
        let mut distances: Vec<u32> = Vec::new();

        for m in re.find_iter(input.lines().next().unwrap()) {
            times.push(m.as_str().parse::<u32>().unwrap());
        }
        for m in re.find_iter(input.lines().nth(1).unwrap()) {
            distances.push(m.as_str().parse::<u32>().unwrap());
        }

        let mut total: u32 = 1;

        for (time, distance) in times.iter().zip(distances.iter()) {
            let discriminant = (*time * *time).checked_sub(4 * distance).expect("Discriminant must be non-negative") as f64;

            let x_min = ((*time as f64 - discriminant.sqrt()) / 2.0) as u32 + 1;
            let x_max = (((*time as f64 + discriminant.sqrt()) / 2.0) - 1.0).ceil() as u32;

            total *= x_max - x_min + 1;
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)").unwrap();

        let time = input.lines().next().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let distance = input.lines().nth(1).unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();

        let time = re.captures(&time).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        let distance = re.captures(&distance).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();

        let discriminant = (time * time).checked_sub(4 * distance).expect("Discriminant must be non-negative") as f64;

        let x_min = ((time as f64 - discriminant.sqrt()) / 2.0) as u32 + 1;
        let x_max = (((time as f64 + discriminant.sqrt()) / 2.0) - 1.0).ceil() as u32;

        let count = x_max - x_min + 1;

        format!("{}", count)
    }
}
