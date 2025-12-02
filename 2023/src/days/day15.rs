use crate::problem::Problem;

pub struct DayFifteen;

use std::collections::{HashMap, VecDeque};
use regex::Regex;

fn hasher(input: &str) -> u8 {
    let mut hash: usize = 0;
    for byte in input.as_bytes() {
        hash += *byte as usize; 
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}

impl Problem for DayFifteen {
    fn part_one(&self, input: &str) -> String {

        let init_sequence = input.trim();

        let mut sum: usize = 0;

        for step in init_sequence.split(',') {
            let h = hasher(step);
            sum += h as usize;
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {

        let re = Regex::new(r"(?P<val>[a-z]+)(?P<op>[=-])(?P<focal>\d*)").unwrap();

        let mut boxes = vec![(HashMap::new(), VecDeque::new()); 256];

        for step in input.trim().split(',') {
            if let Some(caps) = re.captures(step) {
                let val = &caps["val"];
                let op = &caps["op"];
                let focal = caps.name("focal").map_or("", |m| m.as_str());

                let label = val;
                let box_idx = hasher(label) as usize;
                let lenses = &mut boxes[box_idx];
                
                if op == "=" {
                    // add element
                    let lens = label.to_string() + " " + focal;

                    if let Some(lens_pos) = lenses.0.get(label) {
                        // replace old lens at lens_pos
                        *lenses.1.get_mut(*lens_pos).unwrap() = lens;
                    } else {
                        // insert a new lens
                        lenses.0.insert(label.to_string(), lenses.1.len());
                        lenses.1.push_back(lens);
                    }
                } else if op == "-" {
                    // remove element
                    if !lenses.0.contains_key(label) {
                        // nothing to remove
                        continue;
                    }
                    lenses.1.remove(lenses.0[label]);
                    if let Some(old_pos) = lenses.0.remove(label) {
                        // update positions
                        lenses.0.iter_mut()
                            .for_each(|(_, pos)| {
                                if *pos > old_pos { *pos -= 1; }
                            });
                    }
                }
            }
        }

        let mut focus_power: usize = 0;
        let re = Regex::new(r"(\d+)").unwrap();

        for (i, lenses) in boxes.into_iter().enumerate() {
            for (j, lens) in lenses.1.into_iter().enumerate() {
                let focal = re.captures(&lens).unwrap()[1].parse::<usize>().unwrap();
                focus_power += (i + 1) * (j + 1) * focal;
            }
        }

        format!("{}", focus_power)
    }
}
