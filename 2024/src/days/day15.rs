use crate::problem::Problem;

use std::{thread, time::Duration, io::{self, Write}};
use std::hash::{Hash, Hasher};

use std::collections::HashSet;
use std::collections::HashMap;

fn _print_warehouse(walls: HashSet<(i32, i32)>, boxes: HashSet<(i32, i32)>, robot: (i32, i32), 
                    height: usize, width: usize, live: bool) {

    let mut warehouse = vec![vec!['.'; width]; height];

    for (x, y) in walls {
        warehouse[y as usize][x as usize] = '#';
    }
    for (x, y) in boxes {
        warehouse[y as usize][x as usize] = 'O';
    }
    let (x, y) = robot;
    warehouse[y as usize][x as usize] = '@';

    for line in warehouse {
        println!("{}", line.iter().collect::<String>());
    }

    // Flush output, sleep and clear screen
    if live {
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        for _ in 0..height {
            print!("\x1B[2K\x1B[1A\r");
        }
    }
}

fn _print_warehouse_part_two(walls: HashSet<(i32, i32)>, boxes: HashMap<(i32, i32), BoxPair>, robot: (i32, i32), 
                    height: usize, width: usize, live: bool, time: usize, dx: i32, dy: i32) {

    let mut warehouse = vec![vec!['.'; width]; height];

    for (x, y) in walls {
        warehouse[y as usize][x as usize] = '#';
    }
    for b in boxes.values() {
        warehouse[b.left.1 as usize][b.left.0 as usize] = '[';
        warehouse[b.right.1 as usize][b.right.0 as usize] = ']';
    }
    let (x, y) = robot;
    warehouse[y as usize][x as usize] = '@';

    for line in warehouse {
        println!("{}", line.iter().collect::<String>());
    }

    // Flush output, sleep and clear screen
    if live {
        print!("Time: {}   ", time);
        match (dx, dy) {
            (0, -1) => print!("Direction: UP\n"),
            (0, 1) => print!("Direction: DOWN\n"),
            (-1, 0) => print!("Direction: LEFT\n"),
            (1, 0) => print!("Direction: RIGHT\n"),
            _ => print!("Direction: NONE\n"),
        }
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
        for _ in 0..=height {
            print!("\x1B[2K\x1B[1A\r");
        }
    }
}

#[derive(Clone, Debug)]
struct BoxPair {
    left: (i32, i32),
    right: (i32, i32),
}

impl PartialEq for BoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left || self.right == other.left || self.left == other.right
    }
}

impl Eq for BoxPair {}

impl Hash for BoxPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pair = [self.left, self.right];
        pair.hash(state);
    }
}

impl BoxPair {
    fn new(left: (i32, i32), right: (i32, i32)) -> BoxPair {
        BoxPair {
            left: left,
            right: right,
        }
    }
}

pub struct DayFifteen {}

impl Problem for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let _height = input.split("\n\n").next().unwrap().lines().count();
        let _width = input.split("\n\n").next().unwrap().lines().next().unwrap().len();

        let mut walls: HashSet<(i32, i32)> = HashSet::new();
        let mut boxes: HashSet<(i32, i32)> = HashSet::new();
        let mut robot: (i32, i32) = (0, 0);

        // Parse warehouse
        for (y, line) in input.split("\n\n").next().unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert((x as i32, y as i32));
                    }
                    'O' => {
                        boxes.insert((x as i32, y as i32));
                    }
                    '@' => {
                        robot = (x as i32, y as i32);
                    }
                    _ => {}
                }
            }
        }

        // Parse moves
        let moves: Vec<(i32, i32)> = input
            .split("\n\n")
            .nth(1)
            .unwrap()
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => (0, -1),
                    'v' => (0, 1),
                    '<' => (-1, 0),
                    '>' => (1, 0),
                    _ => (0, 0),
                })
            }).collect();

        // Move robot
        for (dx, dy) in moves.iter() {
            let nx = robot.0 + dx;
            let ny = robot.1 + dy;

            if walls.contains(&(nx, ny)) {
                continue;
            }
            if boxes.contains(&(nx, ny)) {
                let mut bx = nx + dx;
                let mut by = ny + dy;

                while boxes.contains(&(bx, by)) {
                    bx = bx + dx;
                    by = by + dy;
                }
                if walls.contains(&(bx, by)) {
                    continue;
                }

                boxes.remove(&(nx, ny));
                boxes.insert((bx, by));
            }
            robot = (nx, ny);

            // _print_warehouse(walls.clone(), boxes.clone(), robot, _height, _width, true);
        }

        // print_warehouse(walls.clone(), boxes.clone(), robot, height, width, false);

        let mut sum = 0;
        for (x, y) in boxes {
            sum += x + y * 100;
        }
        
        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let _height = input.split("\n\n").next().unwrap().lines().count();
        let _width = input.split("\n\n").next().unwrap().lines().next().unwrap().len() * 2;

        let mut walls: HashSet<(i32, i32)> = HashSet::new();
        let mut boxes: HashMap<(i32, i32), BoxPair> = HashMap::new();
        let mut robot: (i32, i32) = (0, 0);

        // Parse warehouse
        for (y, line) in input.split("\n\n").next().unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert(((x * 2) as i32, y as i32));
                        walls.insert(((x * 2 + 1) as i32, y as i32));
                    }
                    'O' => {
                        let b = BoxPair::new(((x * 2) as i32, y as i32), ((x * 2 + 1) as i32, y as i32));
                        boxes.insert(((x * 2) as i32, y as i32), b.clone());
                        boxes.insert(((x * 2 + 1) as i32, y as i32), b);
                    }
                    '@' => {
                        robot = ((x * 2) as i32, y as i32);
                    }
                    _ => {}
                }
            }
        }

        // Parse moves
        let moves: Vec<(i32, i32)> = input
            .split("\n\n")
            .nth(1)
            .unwrap()
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => (0, -1),
                    'v' => (0, 1),
                    '<' => (-1, 0),
                    '>' => (1, 0),
                    _ => (0, 0),
                })
            }).collect();

        for (_time, (dx, dy)) in moves.iter().enumerate() {
            let nx = robot.0 + dx;
            let ny = robot.1 + dy;

            if walls.contains(&(nx, ny)) {
                continue;
            }

            if let Some(b) = boxes.get(&(nx, ny)).cloned() {
                if can_push(&walls, &boxes, &b, *dx, *dy) {
                    // safe to move robot and push box
                    push_boxes(&mut boxes, &b, *dx, *dy);
                    robot = (nx, ny);
                }
                // box cannot be pushed, and thus robot cannot move
            } else {
                // safe to move robot, no wall or box
                robot = (nx, ny);
            }

            // _print_warehouse_part_two(walls.clone(), boxes.clone(), robot, _height, _width, true, _time, *dx, *dy);
        }

        // _print_warehouse_part_two(walls.clone(), boxes.clone(), robot, _height, _width, false, 0, 0, 0);

        let mut sum = 0;
        let boxes: HashSet<&BoxPair> = boxes.values().into_iter().collect();

        for b in boxes {
            sum += b.left.0 + b.left.1 * 100;
        }

        format!("{}", sum)
    }
}

fn can_push(walls: &HashSet<(i32, i32)>, boxes: &HashMap<(i32, i32), BoxPair>, b: &BoxPair, dx: i32, dy: i32) -> bool {
    let bx_left = b.left.0 + dx;
    let by_left = b.left.1 + dy;
    let bx_right = b.right.0 + dx;
    let by_right = b.right.1 + dy;

    if walls.contains(&(bx_left, by_left)) || walls.contains(&(bx_right, by_right)) {
        return false;
    }
    if dy != 0 {
        if let Some(b2) = boxes.get(&(bx_left, by_left)) {
            if let Some(b3) = boxes.get(&(bx_right, by_right)) {
                if b2 != b3 {
                    return can_push(walls, boxes, b2, dx, dy) && can_push(walls, boxes, b3, dx, dy);
                }
            }
            return can_push(walls, boxes, b2, dx, dy);
        }
        if let Some(b3) = boxes.get(&(bx_right, by_right)) {
            return can_push(walls, boxes, b3, dx, dy);
        }
    } else {
        let bx = if dx > 0 { bx_right } else { bx_left };
        if let Some(b2) = boxes.get(&(bx, by_left)) {
            return can_push(walls, boxes, b2, dx, dy);
        }
    }
    true
}

fn push_boxes(boxes: &mut HashMap<(i32, i32), BoxPair>, b: &BoxPair, dx: i32, dy: i32) {
    let bx_left = b.left.0 + dx;
    let by_left = b.left.1 + dy;
    let bx_right = b.right.0 + dx;
    let by_right = b.right.1 + dy;

    if dy != 0 {
        if let Some(b2) = boxes.get(&(bx_left, by_left)).cloned() {
            if let Some(b3) = boxes.get(&(bx_right, by_right)).cloned() {
                if b2 != b3 {
                    push_boxes(boxes, &b2, dx, dy);
                    push_boxes(boxes, &b3, dx, dy);
                } else {
                    push_boxes(boxes, &b2, dx, dy);
                }
            } else {
                push_boxes(boxes, &b2, dx, dy);
            }
        } else if let Some(b3) = boxes.get(&(bx_right, by_right)).cloned() {
            push_boxes(boxes, &b3, dx, dy);
        }
    } else {
        let bx_dx = if dx > 0 { bx_right } else { bx_left };
        if let Some(b2) = boxes.get(&(bx_dx, by_left)).cloned() {
            push_boxes(boxes, &b2, dx, dy);
        }
    }

    // Remove old box
    boxes.remove(&(b.left));
    boxes.remove(&(b.right));

    // Add new box
    let b = BoxPair::new((bx_left, by_left), (bx_right, by_right));

    boxes.insert((bx_left, by_left), b.clone());
    boxes.insert((bx_right, by_right), b);
}
