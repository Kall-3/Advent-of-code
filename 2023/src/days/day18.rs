use crate::problem::Problem;

pub struct DayEighteen;

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Direction::None,
        }
    }

    fn from_string_int(s: &str) -> Direction {
        match s {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => Direction::None,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::None => (0, 0),
        }
    }
}

fn shoelace_i32(points: &Vec<(i32, i32)>) -> i32 {
    let n = points.len();
    let mut area = 0;
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        area += x1 * y2 - x2 * y1;
    }
    area.abs() / 2
}

fn shoelace_isize(points: &Vec<(isize, isize)>) -> isize {
    let n = points.len();
    let mut area = 0;
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        area += x1 * y2 - x2 * y1;
    }
    area.abs() / 2
}

impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {

        let re = Regex::new(r"(?P<dir>[UDLR]) (?P<dist>\d+)").unwrap();

        let mut points: Vec<(i32, i32)> = Vec::new();
        let mut current_pos = (0, 0);
        let mut outer_points = 0;

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let dir = Direction::from_str(&caps["dir"]);
                let dist: i32 = caps["dist"].parse().unwrap();

                let (delta_x, delta_y) = dir.delta();
                let new_pos = (current_pos.0 + delta_x * dist, current_pos.1 + delta_y * dist);

                points.push(new_pos);
                current_pos = new_pos;
                outer_points += dist;
            }
        }

        let area = shoelace_i32(&points);
        let tiles = area + (outer_points / 2) + 1;

        format!("{}", tiles)
    }

    fn part_two(&self, input: &str) -> String {

        let re = Regex::new(r"#([0-9a-f]{5})([0-3])").unwrap();

        let mut points: Vec<(isize, isize)> = Vec::new();
        let mut current_pos = (0, 0);
        let mut outer_points = 0;

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let dir = Direction::from_string_int(&caps[2]);
                let dist = isize::from_str_radix(&caps[1], 16).unwrap();

                let (delta_x, delta_y) = dir.delta();
                let new_pos = (current_pos.0 + (delta_x as isize * dist), current_pos.1 + (delta_y as isize * dist));

                points.push(new_pos);
                current_pos = new_pos;
                outer_points += dist;
            }
        }

        let area = shoelace_isize(&points);
        let tiles = area + (outer_points / 2) + 1;

        format!("{}", tiles)
    }
}
