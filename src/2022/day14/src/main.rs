use itertools::Itertools;
use std::{collections::HashMap, ops::{RangeInclusive}};

fn main() {
    //part1();
    part2()
}

#[derive(Debug)]
struct Cave {
    tiles: HashMap<(i32, i32), char>,
    bottom: i32,
}

impl Cave {
    fn new() -> Cave {
        Cave { tiles: HashMap::new(), bottom: 0 }
    }

    fn convert(point: Option<&str>) -> Option<(i32, i32)> {
        match point {
            Some(s) => s.split(',').map(|l| l.parse().unwrap()).next_tuple::<(_,_)>(),
            _ => None
        }
    }

    fn draw_line(&mut self, start: (i32,i32), end: (i32,i32)) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        self.bottom = vec![self.bottom, start.1, end.1].iter().max().unwrap_or(&0).clone();

        let mut point = start;
        self.tiles.insert(point, '#');
        while point != end {
            point.0 += dx;
            point.1 += dy;
            self.tiles.insert(point, '#');
        }
    }

    fn print_cave(&self, x_range: RangeInclusive<i32>) {
        for y in 0..=self.bottom {
            for x in x_range.clone() {
                if self.tiles.contains_key(&(x,y)) {
                    print!("{}", self.tiles.get(&(x,y)).unwrap())
                } else if (x, y) == (500, 0) {
                    print!("+")
                } else {
                    print!(".")
                }
            }
            println!(" {y}");
        }

    }
}

fn part2() {
    let input = include_str!("input.txt").lines();
    let mut cave = Cave::new();

    for line in input {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            start = end;
        }
    }
    cave.draw_line((470, cave.bottom+2), (530, cave.bottom+2));

    let mut sand_count = 0;
    while drop_sand(&mut cave, (500, 0)) {
        sand_count += 1;
    }
    println!("{sand_count:?}")
}

fn part1() {
    let input = include_str!("test_input.txt").lines();
    let mut cave = Cave::new();

    for line in input {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            start = end;
        }
    }

    let mut sand_count = 0;
    while drop_sand(&mut cave, (500, 1)) {
        sand_count += 1;
        cave.print_cave(493..=504);
    }

    println!("{sand_count:?}")
}

fn drop_sand(cave: &mut Cave, pos: (i32, i32)) -> bool {
    for d_x in vec![0, -1, 1] {
        let new_pos = (pos.0 + d_x, pos.1 + 1);
        //println!("bottom: {:?}, pos: {new_pos:?}", cave.bottom);
        if !cave.tiles.contains_key(&new_pos) && pos.1 < cave.bottom {
            let success = drop_sand(cave, new_pos);
            if success { return true }
        }
    }
    if pos.1 < cave.bottom && !cave.tiles.contains_key(&pos) {
        cave.tiles.insert(pos, 'o');
        return true
    }
    false
}