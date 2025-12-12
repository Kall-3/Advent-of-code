use crate::problem::Problem;

pub struct DaySixteen;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

fn reflection(dir: Dir, obstacle: char) -> Dir {
    match (dir, obstacle) {
        (Dir::Up, '/') => Dir::Right,
        (Dir::Up, '\\') => Dir::Left,
        (Dir::Down, '/') => Dir::Left,
        (Dir::Down, '\\') => Dir::Right,
        (Dir::Left, '/') => Dir::Down,
        (Dir::Left, '\\') => Dir::Up,
        (Dir::Right, '/') => Dir::Up,
        (Dir::Right, '\\') => Dir::Down,
        _ => panic!("Invalid reflection"),
    }
}

fn follow_light(
    board: &Vec<Vec<char>>,
    pos: (i32, i32),
    dir: Dir,
    visited: &mut HashSet<(i32, i32, Dir)>
) {
    if !visited.insert((pos.0, pos.1, dir)) {
        return;
    }

    let (dx, dy) = dir.delta();
    let new_x = pos.0 + dx;
    let new_y = pos.1 + dy;

    if new_x < 0 || new_x >= board.len() as i32 || new_y < 0 || new_y >= board[0].len() as i32 {
        return;
    }

    let next_cell = board[new_x as usize][new_y as usize];

    if next_cell == '/' || next_cell == '\\' {
        // reflect
        let new_dir = reflection(dir, next_cell);
        follow_light(board, (new_x, new_y), new_dir, visited);

    } else if (dir == Dir::Left || dir == Dir::Right) && next_cell == '|' {
        // split up and down
        follow_light(board, (new_x, new_y), Dir::Up, visited);
        follow_light(board, (new_x, new_y), Dir::Down, visited);

    } else if (dir == Dir::Up || dir == Dir::Down) && next_cell == '-' {
        // split left and right
        follow_light(board, (new_x, new_y), Dir::Left, visited);
        follow_light(board, (new_x, new_y), Dir::Right, visited);

    } else {
        // continue straight
        follow_light(board, (new_x, new_y), dir, visited);
    }
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {

        let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();

        follow_light(&board, (0, -1), Dir::Right, &mut visited);

        let unique: HashSet<(i32, i32)> = visited.iter()
            .map(|(x, y, _)| (*x, *y))
            .collect();

        let count = unique.len() - 1; // exclude the starting position

        format!("{}", count)
    }

    fn part_two(&self, input: &str) -> String {
        let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut max_count = 0;

        for i in 0..board.len() {
            let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();
            follow_light(&board, (i as i32, -1), Dir::Right, &mut visited);
            let unique: HashSet<(i32, i32)> = visited.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            max_count = max_count.max(unique.len() - 1);

            let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();
            follow_light(&board, (i as i32, board[0].len() as i32), Dir::Left, &mut visited);
            let unique: HashSet<(i32, i32)> = visited.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            max_count = max_count.max(unique.len() - 1);
        }

        for i in 0..board[0].len() {
            let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();
            follow_light(&board, (-1, i as i32), Dir::Down, &mut visited);
            let unique: HashSet<(i32, i32)> = visited.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            max_count = max_count.max(unique.len() - 1);

            let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();
            follow_light(&board, (board.len() as i32, i as i32), Dir::Up, &mut visited);
            let unique: HashSet<(i32, i32)> = visited.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            max_count = max_count.max(unique.len() - 1);
        }

        format!("{}", max_count)
    }
}
