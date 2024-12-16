use crate::problem::Problem;

use std::collections::HashSet;
use std::collections::HashMap;

fn find_paths(hike_guide: &Vec<Vec<usize>>, x: usize, y: usize, h: usize, visited: &mut HashSet<(usize, usize)>) {
    let height = hike_guide.len();
    let width = hike_guide[0].len();

    let mut possible_paths: Vec<(usize, usize)> = Vec::new();
    if x > 0 && hike_guide[y][x - 1] == h + 1 {
        possible_paths.push((x - 1, y));
    }
    if x + 1 < width && hike_guide[y][x + 1] == h + 1 {
        possible_paths.push((x + 1, y));
    }
    if y > 0 && hike_guide[y - 1][x] == h + 1 {
        possible_paths.push((x, y - 1));
    }
    if y + 1 < height && hike_guide[y + 1][x] == h + 1 {
        possible_paths.push((x, y + 1));
    }

    if h + 1 == 9 {
        for pos in possible_paths {
            visited.insert(pos);
        }
    } else {
        for pos in possible_paths {
            find_paths(hike_guide, pos.0, pos.1, h + 1, visited);
        }
    }
}

fn find_paths_rating(hike_guide: &Vec<Vec<usize>>, x: usize, y: usize, h: usize, visited: &mut HashMap<(usize, usize), usize>) {
    let height = hike_guide.len();
    let width = hike_guide[0].len();

    let mut possible_paths: Vec<(usize, usize)> = Vec::new();
    if x > 0 && hike_guide[y][x - 1] == h + 1 {
        possible_paths.push((x - 1, y));
    }
    if x + 1 < width && hike_guide[y][x + 1] == h + 1 {
        possible_paths.push((x + 1, y));
    }
    if y > 0 && hike_guide[y - 1][x] == h + 1 {
        possible_paths.push((x, y - 1));
    }
    if y + 1 < height && hike_guide[y + 1][x] == h + 1 {
        possible_paths.push((x, y + 1));
    }

    if h + 1 == 9 {
        for pos in possible_paths {
            *visited.entry(pos).or_insert(0) += 1;
        }
    } else {
        for pos in possible_paths {
            find_paths_rating(hike_guide, pos.0, pos.1, h + 1, visited);
        }
    }
}


pub struct DayTen {}

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let hike_guide: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
            ).collect();

        let mut sum = 0;

        for (j, r) in hike_guide.iter().enumerate() {
            for (i, c) in r.iter().enumerate() {
                if *c == 0 {
                    let mut visited = HashSet::new();
                    find_paths(&hike_guide, i, j, 0, &mut visited);
                    sum += visited.len();
                }
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let hike_guide: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
            ).collect();

        let mut sum = 0;

        for (j, r) in hike_guide.iter().enumerate() {
            for (i, c) in r.iter().enumerate() {
                if *c == 0 {
                    let mut visited = HashMap::new();
                    find_paths_rating(&hike_guide, i, j, 0, &mut visited);
                    sum += visited.into_values().sum::<usize>();
                }
            }
        }

        format!("{}", sum)
    }
}
