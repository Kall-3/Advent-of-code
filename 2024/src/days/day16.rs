use crate::problem::Problem;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

static _COUNTER: AtomicUsize = AtomicUsize::new(0);

fn _print_map(paths: &HashSet<(i32, i32)>, raindeer: (i32, i32), goal: (i32, i32),
    height: usize, width: usize, current_score: i32, best_score: i32, optimal_path: HashSet<(i32, i32)>, live: bool) {

    _COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut visiual: Vec<Vec<char>> = vec![vec!['#'; width]; height];
    for (x, y) in paths {
        visiual[(*y + 1) as usize][(*x + 1) as usize] = '.';
    }
    visiual[height - 2][1] = 'S';
    visiual[(goal.1 + 1) as usize][(goal.0 + 1) as usize] = 'E';
    visiual[(raindeer.1 + 1) as usize][(raindeer.0 + 1) as usize] = '@';
    for (x, y) in &optimal_path {
        visiual[(*y + 1) as usize][(*x + 1) as usize] = 'O';
    }

    for line in visiual {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    println!("Time: {}  Cur: {}  Best: {}", _COUNTER.load(Ordering::Relaxed), current_score, best_score);

    if live {
        // Flush output, sleep and clear screen
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        for _ in 0..=height {
            print!("\x1B[2K\x1B[1A\r");
        }
    }
}

fn dti(d: (i32, i32)) -> i32 {
    match d {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("ERROR: Not valid direction"),
    }
}

pub struct DaySixteen {}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        let mut paths: HashSet<(i32, i32)> = HashSet::new();
        let mut goal = (0, 0);
        let mut raindeer = (0, 0);
        let heading = (1, 0);

        for (y, line) in input.lines().skip(1).take(height - 2).enumerate() {
            for (x, c) in line.chars().skip(1).take(width - 2).enumerate() {
                match c {
                    '.' => { paths.insert((x as i32, y as i32)); }
                    'S' => { raindeer = (x as i32, y as i32); }
                    'E' => {
                        goal = (x as i32, y as i32);
                        paths.insert((x as i32, y as i32));
                    }
                    _ => {}
                }
            }
        }

        // BFS dynamic programming

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut queue: VecDeque<((i32, i32), (i32, i32), i32)> = VecDeque::new();
        let mut visited: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
        queue.push_back((raindeer, heading, 0));

        while let Some((pos, heading, score)) = queue.pop_front().clone() {
            if pos == goal {
                continue;
            }
            // _print_map(&map, pos, goal, height, width, score, Rc::new(RefCell::new(i32::MAX)));
            for (dx, dy) in &directions {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;

                if paths.contains(&(nx, ny)) {
                    let number_of_turns = (4 + dti((*dx, *dy)) - dti(heading)) % 4;
                    let number_of_turns = number_of_turns.min(4 - number_of_turns);

                    let new_score = score + number_of_turns * 1000 + 1;

                    if let Some(best_score) = visited.get(&((nx, ny), (*dx, *dy))).cloned() {
                        if new_score < best_score {
                            queue.push_back(((nx, ny), (*dx, *dy), new_score));
                            visited.insert(((nx, ny), (*dx, *dy)), new_score);
                        }
                    } else {
                        queue.push_back(((nx, ny), (*dx, *dy), new_score));
                        visited.insert(((nx, ny), (*dx, *dy)), new_score);
                    }
                }
            }
        }

        let mut best_score = i32::MAX;
        for dir in &directions {
            best_score = best_score.min(*visited.get(&(goal, *dir)).unwrap_or(&i32::MAX));
        }

        format!("{}", best_score)
    }

    fn part_two(&self, input: &str) -> String {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        let mut paths: HashSet<(i32, i32)> = HashSet::new();
        let mut goal = (0, 0);
        let mut raindeer = (0, 0);
        let heading = (1, 0);

        for (y, line) in input.lines().skip(1).take(height - 2).enumerate() {
            for (x, c) in line.chars().skip(1).take(width - 2).enumerate() {
                match c {
                    '.' => { paths.insert((x as i32, y as i32)); }
                    'S' => { raindeer = (x as i32, y as i32); }
                    'E' => {
                        goal = (x as i32, y as i32);
                        paths.insert((x as i32, y as i32));
                    }
                    _ => {}
                }
            }
        }

        // BFS dynamic programming

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut queue: VecDeque<((i32, i32), (i32, i32), i32)> = VecDeque::new();
        let mut visited: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
        queue.push_back((raindeer, heading, 0));
        visited.insert((raindeer, heading), 0);

        while let Some((pos, heading, score)) = queue.pop_front().clone() {
            if pos == goal {
                continue;
            }
            // _print_map(&paths, pos, goal, height, width, score, *directions.iter().map(|(dx, dy)| visited.get(&(goal, (*dx, *dy))).unwrap_or(&i32::MAX)).min().unwrap_or(&i32::MAX));
            for (dx, dy) in &directions {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;

                if paths.contains(&(nx, ny)) {
                    let number_of_turns = (4 + dti((*dx, *dy)) - dti(heading)) % 4;
                    let number_of_turns = number_of_turns.min(4 - number_of_turns);

                    let new_score = score + number_of_turns * 1000 + 1;

                    if let Some(best_score) = visited.get(&((nx, ny), (*dx, *dy))).cloned() {
                        if new_score < best_score {
                            queue.push_back(((nx, ny), (*dx, *dy), new_score));
                            visited.insert(((nx, ny), (*dx, *dy)), new_score);
                        }
                    } else {
                        queue.push_back(((nx, ny), (*dx, *dy), new_score));
                        visited.insert(((nx, ny), (*dx, *dy)), new_score);
                    }
                }
            }
        }

        // Find all optimal paths
        let best_score = directions.iter().map(|(dx, dy)| visited.get(&(goal, (*dx, *dy))).unwrap_or(&i32::MAX)).min().unwrap_or(&i32::MAX);

        let mut optimal_path: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<((i32, i32), (i32, i32), i32)> = VecDeque::new();

        // Add first element with path it had (can be multiple optimals)
        for dir in &directions {
            if visited.get(&(goal, *dir)).unwrap_or(&i32::MAX) == best_score {
                queue.push_back((goal, *dir, *best_score));
            }
        }
        optimal_path.insert(goal);

        while let Some((pos, heading, score)) = queue.pop_front().clone() {
            for (dx, dy) in &directions {
                let nx = pos.0 - dx;
                let ny = pos.1 - dy;

                for (c_dx, c_dy) in &directions {

                    // Check if neighbour has optimal score, otherwise not optimal path
                    if let Some(neighbour_score) = visited.get(&((nx, ny), (*c_dx, *c_dy))).cloned() {

                        let mut number_of_turns = (4 + dti((*c_dx, *c_dy)) - dti(heading)) % 4;
                        number_of_turns = number_of_turns.min(4 - number_of_turns);

                        if neighbour_score + number_of_turns * 1000 + 1 == score {
                            queue.push_back(((nx, ny), (*c_dx, *c_dy), neighbour_score));
                            optimal_path.insert((nx, ny));
                        }
                    }
                }
            }
        }

        let result = optimal_path.len();

        // _print_map(&paths, raindeer, goal, height, width, *best_score, *best_score, optimal_path, false);

        format!("{}", result)
    }
}
