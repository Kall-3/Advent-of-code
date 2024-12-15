use crate::problem::Problem;

use std::collections::HashSet;

pub struct DaySix {}

struct Guard {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Guard {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, dx: 0, dy: -1 }
    }

    // Rotate 90 degrees clockwise
    fn rotate(&mut self) {
        let tmp = self.dx;
        self.dx = -self.dy;
        self.dy = tmp;
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut guard: Guard = Guard::new(0, 0);

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((x as i32, y as i32));
                    }
                    '^' => {
                        guard = Guard::new(x as i32, y as i32);
                    }
                    _ => {}
                }
            }
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        loop {
            visited.insert((guard.x, guard.y));
            if !obstacles.contains(&(guard.x + guard.dx, guard.y + guard.dy)) {
                if guard.y + guard.dy < 0 || guard.y + guard.dy >= height || guard.x + guard.dx < 0 || guard.x + guard.dx >= width {
                    break;
                }
                guard.step();
            } else {
                guard.rotate();
            }
        }

        format!("{}", visited.len())
    }

    fn part_two(&self, input: &str) -> String {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let (mut guard_x, mut guard_y) = (0, 0);

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((x as i32, y as i32));
                    }
                    '^' => {
                        guard_x = x as i32;
                        guard_y = y as i32;
                    }
                    _ => {}
                }
            }
        }

        let mut guard = Guard::new(guard_x, guard_y);

        // (x, y, dx, dy)
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        loop {
            visited.insert((guard.x, guard.y));
            if !obstacles.contains(&(guard.x + guard.dx, guard.y + guard.dy)) {
                if guard.y + guard.dy < 0 || guard.y + guard.dy >= height || guard.x + guard.dx < 0 || guard.x + guard.dx >= width {
                    break;
                }
                guard.step();
            } else {
                guard.rotate();
            }
        }

        let mut loops_visited = 0;

        for (x, y) in visited.iter() {
            // Try new obstacle
            obstacles.insert((*x, *y));

            // Reset board
            let mut new_visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();
            let mut guard = Guard::new(guard_x, guard_y);

            loop {
                if new_visited.contains(&(guard.x, guard.y, guard.dx, guard.dy)) {
                    loops_visited += 1;
                    break;
                }
                new_visited.insert((guard.x, guard.y, guard.dx, guard.dy));
                if !obstacles.contains(&(guard.x + guard.dx, guard.y + guard.dy)) {
                    if guard.y + guard.dy < 0 || guard.y + guard.dy >= height || guard.x + guard.dx < 0 || guard.x + guard.dx >= width {
                        break;
                    }
                    guard.step();
                } else {
                    guard.rotate();
                }
            }

            obstacles.remove(&(*x, *y));
        }

        format!("{}", loops_visited)
    }
}
