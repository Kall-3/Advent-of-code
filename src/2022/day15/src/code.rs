use std::{collections::HashSet, ops::RangeInclusive, io::Write};
use tuple_map::TupleMap2;
use range_set::*;

pub struct Run;
impl Run {
    pub fn part1() {
        part1();
    }
    pub fn part2() {
        part2();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radix: i32,
}

impl Sensor {
    fn new(line: &str) -> Self {
        let (sensor, beacon) = line.split_once(": closest beacon is at x=").unwrap();
        let sensor: (i32, i32) = sensor.split_once("Sensor at x=").unwrap().1.split_once(", y=").unwrap().map(|l| l.parse().unwrap());
        let beacon: (i32, i32) = beacon.split_once(", y=").unwrap().map(|l| l.parse().unwrap());
        let radix = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        Sensor { sensor, beacon, radix }
    }

    fn coverage_on_line(&self, row: i32) -> RangeInclusive<i32> {
        let range = self.radix - (self.sensor.1 - row).abs();
        (self.sensor.0 - range)..=(self.sensor.0 + range)
    }

    fn incorporate(&self, coverage: &mut RangeSet<[RangeInclusive<i32>; 1]>, row: i32) {
        let cov = self.coverage_on_line(row);
        coverage.insert_range(cov);
        if row == self.beacon.1 {
            coverage.remove(self.beacon.0);
        }
    }

    fn exclude(&self, coverage: &mut RangeSet<[RangeInclusive<i32>; 1]>, row: i32) {
        let cov = self.coverage_on_line(row);
        coverage.remove_range(cov);
    }
}

fn part2() {
    let input = include_str!("input.txt").lines();
    let mut coverage = vec![RangeSet::<[RangeInclusive<i32>; 1]>::from(0..=4000000); 4000001];
    
    for line in input {
        let sensor = Sensor::new(line);
        for row in 0..=4000000 {
            sensor.exclude(&mut coverage[row], row as i32)
        }
    }

    let coordinate = coverage.iter().enumerate().filter(|(_, l)| !l.is_empty()).next().unwrap();
    let coordinate: (usize, usize) = (coordinate.1.clone().into_smallvec().first().unwrap().clone().next().unwrap().try_into().unwrap(), coordinate.0);
    let res = coordinate.0 * 4000000 + coordinate.1;
    std::io::stdout().write(format!("{:?}\n", res).as_bytes()).unwrap();
}

fn part1() {
    let input = include_str!("input.txt").lines();
    let mut coverage = RangeSet::<[RangeInclusive<i32>; 1]>::from(0..=0);
    
    for line in input {
        let sensor = Sensor::new(line);
        sensor.incorporate(&mut coverage, 2000000);
    }

    let size = coverage.into_smallvec().iter().fold(0, |a, b| a + (b.clone().count()));
    std::io::stdout().write(format!("{}\n", size).as_bytes()).unwrap();
}