use crate::problem::Problem;

use std::collections::HashMap;
use std::collections::HashSet;

fn flood_fill(graph: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>, perimiter: &mut i32, x: i32, y: i32, c: char) {
    visited.insert((x, y));

    let height = graph.len() as i32;
    let width = graph[0].len() as i32;
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut neighbours = 0;

    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
            continue;
        }
        if graph[ny as usize][nx as usize] == c {
            neighbours += 1;
            if !visited.contains(&(nx, ny)) {
                flood_fill(graph, visited, perimiter, nx, ny, c);
            }
        }
    }

    *perimiter += 4 - neighbours;
}

fn find_groups(graph: &Vec<Vec<char>>) -> HashMap<String, (HashSet<(i32, i32)>, i32)> {
    let mut groups: HashMap<String, (HashSet<(i32, i32)>, i32)> = HashMap::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (y, row) in graph.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if visited.contains(&(x as i32, y as i32)) {
                continue;
            }

            let mut new_visited = HashSet::new();
            let mut perimiter = 0;

            flood_fill(&graph, &mut new_visited, &mut perimiter, x as i32, y as i32, *c);
            visited.extend(new_visited.iter());

            let mut id = c.to_string();
            while groups.contains_key(&id) {
                id = format!("{}'", id);
            }
            groups.insert(id, (new_visited, perimiter));
        }
    }
    groups
}

pub struct DayTwelve {}

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let graph: Vec<Vec<char>> = input
            .lines()
            .map(move |line| {
                line.chars().collect()
            }).collect();

        let groups = find_groups(&graph);

        let sum = groups.iter().map(|(_, (n, p))| n.len() as i32 * p).sum::<i32>();

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let graph: Vec<Vec<char>> = input
            .lines()
            .map(move |line| {
                line.chars().collect()
            }).collect();

        // Parse the graph into groups
        let groups = find_groups(&graph);

        // The number of sides is equal to the number of corners, to count the corner
        // check the neighbours in each pair of orthogonal directions. If they dont match 
        // it's a corner. If they match AND the corner does not, that is also a corner.

        let mut sum = 0;

        for (_, (nodes, _)) in groups.iter() {
            let mut corners = 0;
            for (x, y) in nodes.iter() {
                // North east
                if !nodes.contains(&(*x + 1, *y)) && !nodes.contains(&(*x, *y - 1)) {
                    corners += 1;
                } else if nodes.contains(&(*x + 1, *y)) && nodes.contains(&(*x, *y - 1)) && !nodes.contains(&(*x + 1, *y - 1)) {
                    corners += 1;
                }
                // North west
                if !nodes.contains(&(*x - 1, *y)) && !nodes.contains(&(*x, *y - 1)) {
                    corners += 1;
                } else if nodes.contains(&(*x - 1, *y)) && nodes.contains(&(*x, *y - 1)) && !nodes.contains(&(*x - 1, *y - 1)) {
                    corners += 1;
                }
                // South east
                if !nodes.contains(&(*x + 1, *y)) && !nodes.contains(&(*x, *y + 1)) {
                    corners += 1;
                } else if nodes.contains(&(*x + 1, *y)) && nodes.contains(&(*x, *y + 1)) && !nodes.contains(&(*x + 1, *y + 1)) {
                    corners += 1;
                }
                // South west
                if !nodes.contains(&(*x - 1, *y)) && !nodes.contains(&(*x, *y + 1)) {
                    corners += 1;
                } else if nodes.contains(&(*x - 1, *y)) && nodes.contains(&(*x, *y + 1)) && !nodes.contains(&(*x - 1, *y + 1)) {
                    corners += 1;
                }
            }
            sum += nodes.len() as i32 * corners;
        }

        format!("{}", sum)
    }
}
