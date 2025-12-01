use crate::problem::Problem;

pub struct DayOne;

use regex::Regex;

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"^(?P<dir>[LR])(?P<nbr>(\d+))$").unwrap();

        let mut pos: i32 = 50;
        let mut ans: u32 = 0;

        for line in input.lines() {
            let caps = re.captures(line).unwrap();
            let dir = &caps["dir"];
            let nbr: i32 = caps["nbr"].parse().unwrap();

            if dir == "L" {
                pos -= nbr;
            } else {
                pos += nbr;
            }

            if (pos % 100) == 0 {
                ans += 1;
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new(r"^(?P<dir>[LR])(?P<nbr>(\d+))$").unwrap();

        let mut pos: i32 = 50;
        let mut crossings: u32 = 0;

        for line in input.lines() {
            let caps = re.captures(line).unwrap();
            let dir = &caps["dir"];
            let nbr: i32 = caps["nbr"].parse().unwrap();

            let step = if dir == "L" { -nbr } else { nbr };

            crossings += hits_from(pos, step);

            pos += step;

            println!("Line: {}", line);
            println!("Total pos: {}", pos);
            println!("Ans: {}\n", crossings);
        }

        format!("{}", crossings)
    }
}

fn hits_from(pos: i32, step: i32) -> u32 {
    let pos = pos.rem_euclid(100);

    if step > 0 {
        // distance from pos to next 0 going right
        let offset = 100 - pos;
        if step < offset {
            0
        } else {
            1 + ((step - offset) as u32 / 100)
        }
    } else if step < 0 {
        let step_abs = -step;

        // distance from pos to next 0 going left
        let mut offset = pos;
        if offset == 0 {
            offset = 100; // first 0 is 100 away
        }

        if step_abs < offset {
            0
        } else {
            1 + ((step_abs - offset) as u32 / 100)
        }
    } else {
        0
    }
}
