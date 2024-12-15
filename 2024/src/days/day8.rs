use crate::problem::Problem;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct DayEight {}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '.' => {},
                    _ => { antennas.entry(c).or_insert(Vec::new()).push((x as i32, y as i32)); }
                }
            }
        }

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        // Debugging visiual graph
        let mut graph: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];

        for (freq, positions) in antennas.iter() {
            for i in 0..positions.len() {
                graph[positions[i].1 as usize][positions[i].0 as usize] = *freq;
                for j in 0..i {
                    let dx = positions[i].0 - positions[j].0;
                    let dy = positions[i].1 - positions[j].1;

                    let antinode1 = (positions[i].0 + dx, positions[i].1 + dy);
                    let antinode2 = (positions[j].0 - dx, positions[j].1 - dy);

                    if antinode1.0 >= 0 && antinode1.0 < width && antinode1.1 >= 0 && antinode1.1 < height {
                        antinodes.insert(antinode1);
                        graph[antinode1.1 as usize][antinode1.0 as usize] = '#';
                    }
                    if antinode2.0 >= 0 && antinode2.0 < width && antinode2.1 >= 0 && antinode2.1 < height {
                        antinodes.insert(antinode2);
                        graph[antinode2.1 as usize][antinode2.0 as usize] = '#';
                    }
                }
            }
        }

        for l in graph.iter() {
            for c in l.iter() {
                print!("{}", c);
            }
            println!();
        }

        format!("{}", antinodes.len())
    }

    fn part_two(&self, input: &str) -> String {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '.' => {},
                    _ => { antennas.entry(c).or_insert(Vec::new()).push((x as i32, y as i32)); }
                }
            }
        }

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        // Debugging visiual graph
        let mut graph: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];

        for (freq, positions) in antennas.iter() {
            for i in 0..positions.len() {
                graph[positions[i].1 as usize][positions[i].0 as usize] = *freq;
                for j in 0..i {
                    let dx = positions[i].0 - positions[j].0;
                    let dy = positions[i].1 - positions[j].1;

                    let mut min = positions[i];
                    loop {
                        let new_min = (min.0 + dx, min.1 + dy);
                        if new_min.0 < 0 || new_min.0 >= width || new_min.1 < 0 || new_min.1 >= height {
                            break;
                        }
                        min = new_min;
                    }

                    loop {
                        if min.0 >= 0 && min.0 < width && min.1 >= 0 && min.1 < height {
                            antinodes.insert(min);
                            graph[min.1 as usize][min.0 as usize] = '#';
                        } else {
                            break;
                        }
                        min = (min.0 - dx, min.1 - dy);
                    }
                }
            }
        }

        for l in graph.iter() {
            for c in l.iter() {
                print!("{}", c);
            }
            println!();
        }
        format!("{}", antinodes.len())
    }
}
