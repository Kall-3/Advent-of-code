use crate::problem::Problem;

use regex::Regex;
use std::collections::HashSet;

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        // Manually edit width, height and seconds to match puzzle
        let width = 101;
        let height = 103;
        let seconds = 100;

        let re_pos = Regex::new(r"p=(\d+),(\d+)").unwrap();
        let re_vel = Regex::new(r"v=(-?\d+),(-?\d+)").unwrap();

        let positions: Vec<(i32, i32)> = re_pos
            .captures_iter(input)
            .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
            .collect();
        let velocities: Vec<(i32, i32)> = re_vel
            .captures_iter(input)
            .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
            .collect();

        let mut quad_sum = vec![vec![0; 2]; 2];

        for ((x, y), (dx, dy)) in positions.iter().zip(velocities.iter()) {
            let nx = (x + dx * seconds).rem_euclid(width) as usize;
            let ny = (y + dy * seconds).rem_euclid(height) as usize;

            if nx != width as usize / 2 && ny != height as usize / 2 {
                let x = (nx * 2 / width as usize) as usize;
                let y = (ny * 2 / height as usize) as usize;
                quad_sum[y][x] += 1;
            }
        }

        let sum: i32 = quad_sum.concat().into_iter().product();

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        // Manually edit width, height and seconds to match puzzle
        let width = 101;
        let height = 103;

        let re_pos = Regex::new(r"p=(\d+),(\d+)").unwrap();
        let re_vel = Regex::new(r"v=(-?\d+),(-?\d+)").unwrap();

        let positions: Vec<(i32, i32)> = re_pos
            .captures_iter(input)
            .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
            .collect();
        let velocities: Vec<(i32, i32)> = re_vel
            .captures_iter(input)
            .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
            .collect();

        let mut results: Vec<(i32, i32)> = Vec::new();

        for seconds in 0..10000 {
            let mut quad_sum = vec![vec![0; 2]; 2];

            for ((x, y), (dx, dy)) in positions.iter().zip(velocities.iter()) {
                let nx = (x + dx * seconds).rem_euclid(width) as usize;
                let ny = (y + dy * seconds).rem_euclid(height) as usize;

                if nx != width as usize / 2 && ny != height as usize / 2 {
                    let x = (nx * 2 / width as usize) as usize;
                    let y = (ny * 2 / height as usize) as usize;
                    quad_sum[y][x] += 1;
                }
            }

            let sum: i32 = quad_sum.concat().into_iter().product();
            results.push((seconds, sum));
        }

        if let Some(&(seconds, _)) = results.iter().min_by_key(|&(_, sum)| sum) {
            let mut graph: HashSet<(i32, i32)> = HashSet::new();

            for ((x, y), (dx, dy)) in positions.iter().zip(velocities.iter()) {
                let nx = (x + dx * seconds).rem_euclid(width) as usize;
                let ny = (y + dy * seconds).rem_euclid(height) as usize;

                graph.insert((ny as i32, nx as i32));
            }

            for i in 0..height as usize {
                for j in 0..width as usize {
                    if graph.contains(&(i as i32, j as i32)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

            return format!("{}", seconds);
        }

        format!("{}", "No awnser found")
    }
}

