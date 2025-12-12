use std::{collections::HashMap, mem};

use regex::Regex;

use crate::problem::Problem;

pub struct DayEleven;

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"(?P<node>\w+): (?P<neighbors>[\w ]+)").unwrap();

        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for cap in re.captures_iter(input) {
            let node: String = cap["node"].to_string();
            let neighbors: Vec<String> = cap["neighbors"]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(node, neighbors);
        }

        let mut queue: HashMap<String, usize> = HashMap::new();
        queue.insert("you".to_string(), 1);
        let mut next_queue: HashMap<String, usize> = HashMap::new();
        let mut total_paths = 0;

        while !queue.is_empty() {
            for (node, num_paths) in queue.drain() {
                if node == "out" {
                    total_paths += num_paths;
                    continue;
                }

                if let Some(neighbors) = graph.get(&node) {
                    for neighbor in neighbors {
                        *next_queue.entry(neighbor.clone()).or_insert(0) += num_paths;
                    }
                }
            }

            mem::swap(&mut queue, &mut next_queue);
        }

        format!("{}", total_paths)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new(r"(?P<node>\w+): (?P<neighbors>[\w ]+)").unwrap();

        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for cap in re.captures_iter(input) {
            let node: String = cap["node"].to_string();
            let neighbors: Vec<String> = cap["neighbors"]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(node, neighbors);
        }

        let mut queue: HashMap<(String, bool, bool), usize> = HashMap::new();
        queue.insert(("svr".to_string(), false, false), 1);
        let mut next_queue: HashMap<(String, bool, bool), usize> = HashMap::new();
        let mut total_paths = 0;

        while !queue.is_empty() {
            for ((node, fft, dac), num_paths) in queue.drain() {
                if node == "out" && fft && dac {
                    total_paths += num_paths;
                    continue;
                }

                let fft = if node == "fft" { true } else { fft };
                let dac = if node == "dac" { true } else { dac };

                if let Some(neighbors) = graph.get(&node) {
                    for neighbor in neighbors {
                        *next_queue.entry((neighbor.clone(), fft, dac)).or_insert(0) += num_paths;
                    }
                }
            }

            mem::swap(&mut queue, &mut next_queue);
        }

        format!("{}", total_paths)
    }
}
