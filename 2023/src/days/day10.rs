use crate::problem::Problem;

pub struct DayTen;

use std::collections::HashMap;

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {

        let (x, y) = input.lines()
            .enumerate()
            .find_map(|(i, line)| {
                line.chars().enumerate().find_map(|(j, c)| {
                    if c == 'S' {
                        Some((j as i32, i as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        fn travel(map: &Vec<Vec<char>>, pos: (i32, i32), visited: &mut HashMap<(i32, i32), i32>, distance: i32) {
            if visited.contains_key(&pos) && visited[&pos] <= distance {
                return;
            }

            visited.insert(pos, distance);

            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in directions {
                let new_x = pos.0 + dx;
                let new_y = pos.1 + dy;

                if new_x < 0 || new_x >= map[0].len() as i32 || new_y < 0 || new_y >= map.len() as i32 {
                    // Out of bounds
                    continue;
                }

                let cur_c = map[pos.1 as usize][pos.0 as usize];
                let new_c = map[new_y as usize][new_x as usize];

                if dx > 0 && (new_c == 'J' || new_c == '7' || new_c == '-')
                    && (cur_c == 'S' || cur_c == 'F' || cur_c == 'L' || cur_c == '-') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited, distance + 1);
                } else if dx < 0 && (new_c == 'L' || new_c == 'F' || new_c == '-') 
                    && (cur_c == 'S' || cur_c == 'J' || cur_c == '7' || cur_c == '-') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited, distance + 1);
                } else if dy > 0 && (new_c == 'J' || new_c == 'L' || new_c == '|') 
                    && (cur_c == 'S' || cur_c == 'F' || cur_c == '7' || cur_c == '|') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited, distance + 1);
                } else if dy < 0 && (new_c == 'F' || new_c == '7' || new_c == '|') 
                    && (cur_c == 'S' || cur_c == 'J' || cur_c == 'L' || cur_c == '|') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited, distance + 1);
                }
            }
        }

        let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

        travel(&map, (x, y), &mut visited, 0);

        let max = visited.values().max().unwrap();

        format!("{}", max)
    }

    fn part_two(&self, input: &str) -> String {
        let (x, y) = input.lines()
            .enumerate()
            .find_map(|(i, line)| {
                line.chars().enumerate().find_map(|(j, c)| {
                    if c == 'S' {
                        Some((j as i32, i as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        fn travel(map: &Vec<Vec<char>>, pos: (i32, i32), visited: &mut Vec<(i32, i32)>) {
            if visited.contains(&pos) {
                return;
            }

            visited.push(pos);

            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in directions {
                let new_x = pos.0 + dx;
                let new_y = pos.1 + dy;

                if visited.contains(&(new_x, new_y)) {
                    continue;
                }

                if new_x < 0 || new_x >= map[0].len() as i32 || new_y < 0 || new_y >= map.len() as i32 {
                    // Out of bounds
                    continue;
                }

                let cur_c = map[pos.1 as usize][pos.0 as usize];
                let new_c = map[new_y as usize][new_x as usize];

                if dx > 0 && (new_c == 'J' || new_c == '7' || new_c == '-')
                    && (cur_c == 'S' || cur_c == 'F' || cur_c == 'L' || cur_c == '-') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited);
                    break;
                } else if dx < 0 && (new_c == 'L' || new_c == 'F' || new_c == '-') 
                    && (cur_c == 'S' || cur_c == 'J' || cur_c == '7' || cur_c == '-') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited);
                    break;
                } else if dy > 0 && (new_c == 'J' || new_c == 'L' || new_c == '|') 
                    && (cur_c == 'S' || cur_c == 'F' || cur_c == '7' || cur_c == '|') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited);
                    break;
               } else if dy < 0 && (new_c == 'F' || new_c == '7' || new_c == '|') 
                    && (cur_c == 'S' || cur_c == 'J' || cur_c == 'L' || cur_c == '|') {
                    travel(map, (pos.0 + dx, pos.1 + dy), visited);
                    break;
                }
            }
        }

        let mut visited: Vec<(i32, i32)> = Vec::new();
        
        travel(&map, (x, y), &mut visited);

        visited.push((x, y));

        let sum: i32 = visited.as_slice().windows(2).fold(0, |sum, w| {
            sum + (w[0].0 * w[1].1 - w[0].1 * w[1].0)
        });
        let area = sum.abs() / 2;
        let b = visited.len() as i32 - 1;
        let i = area - (b / 2) + 1;

        format!("{}", i)
    }
}
