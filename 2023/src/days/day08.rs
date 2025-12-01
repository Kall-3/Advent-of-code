use crate::problem::Problem;

pub struct DayEight;

use std::collections::HashMap;
use regex::Regex;

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let mut input = input.lines();

        let instructions: String = input.next().unwrap().to_string();

        input.next();

        let re = Regex::new(r"(?P<node>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

        for line in input {
            if let Some(caps) = re.captures(line) {
                let node = caps.name("node").unwrap().as_str();
                let left = caps.name("left").unwrap().as_str();
                let right = caps.name("right").unwrap().as_str();

                map.insert(node, (left, right));
            }
        }

        let chars: Vec<char> = instructions.chars().collect();
        let mut steps = 0;
        let mut current: &str = "AAA";

        while current != "ZZZ" {
            let c = chars[steps % chars.len()];

            let (left, right) = map.get(&current).unwrap();

            if c == 'L' {
                current = left;
            } else if c == 'R' {
                current = right;
            }

            steps += 1;
        }

        format!("{}", steps)
    }

    fn part_two(&self, input: &str) -> String {
        let mut input = input.lines();

        let instructions: String = input.next().unwrap().to_string();

        input.next();

        let re = Regex::new(r"(?P<node>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut starting_nodes: Vec<&str> = Vec::new();

        for line in input {
            if let Some(caps) = re.captures(line) {
                let node = caps.name("node").unwrap().as_str();
                let left = caps.name("left").unwrap().as_str();
                let right = caps.name("right").unwrap().as_str();

                map.insert(node, (left, right));

                if node.ends_with('A') {
                    starting_nodes.push(node);
                }
            }
        }

        let chars: Vec<char> = instructions.chars().collect();

        let mut lcms: Vec<usize> = Vec::new();

        for node in starting_nodes {
            let mut current: &str = node;
            let mut step = 0;

            while !current.ends_with("Z") {
                let (left, right) = map.get(&current).unwrap();

                let i = chars[step % chars.len()];

                if i == 'L' {
                    current = left;
                } else if i == 'R' {
                    current = right;
                }

                step += 1;
            }

            lcms.push(step);
        }

        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        fn lcm(a: usize, b: usize) -> usize {
            (a * b) / gcd(a, b)
        }

        let ans = lcms.into_iter().reduce(|a, b| lcm(a, b)).unwrap();

        format!("{}", ans)
    }
}
