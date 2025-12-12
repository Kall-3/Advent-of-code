use crate::problem::Problem;

pub struct DaySeventeen;

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::None => (0, 0),
        }
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::None => Dir::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    position: (i32, i32),
    direction: Dir,
    consecutive: u8,
}

impl State {
    fn new(position: (i32, i32), direction: Dir, consecutive: u8) -> Self {
        Self {
            position,
            direction,
            consecutive,
        }
    }
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {

        let target_x = input.lines().next().unwrap().chars().count() as i32 - 1;
        let target_y = input.lines().count() as i32 - 1;

        let heatloss_map: HashMap<(i32, i32), usize> = input.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, ch)| {
                    ((x as i32, y as i32), ch.to_digit(10).unwrap() as usize)
                })
            }).collect();

        // dijkstra
        let mut heap: BinaryHeap<(Reverse<usize>, State)> = BinaryHeap::new();
        heap.push((Reverse(0), State::new((0, 0), Dir::None, 0)));

        let mut memo: HashMap<State, usize> = HashMap::new();

        while let Some((Reverse(heatloss), state)) = heap.pop() {
            if state.position == (target_x, target_y) {
                // reached the goal
                return format!("{}", heatloss);
            }

            if let Some(&best_heatloss) = memo.get(&state) {
                if heatloss >= best_heatloss {
                    // already found a better or equal way to this state
                    continue;
                }
            }

            memo.insert(state.clone(), heatloss);

            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter() {
                if dir == &state.direction.opposite() {
                    // cannot go backwards
                    continue;
                }

                if dir == &state.direction && state.consecutive > 2 {
                    // cannot go more than 3 times in the same direction
                    continue;
                }

                let (delta_x, delta_y) = dir.delta();
                let new_position = (state.position.0 + delta_x, state.position.1 + delta_y);

                if !heatloss_map.contains_key(&new_position) {
                    // out of bounds
                    continue;
                }

                let new_consecutive = if dir == &state.direction {
                    state.consecutive + 1
                } else {
                    1
                };

                let new_heatloss = heatloss + heatloss_map.get(&new_position).unwrap();

                let new_state = State::new(new_position, *dir, new_consecutive);

                if memo.get(&new_state).map_or(true, |&old| new_heatloss < old) {
                    heap.push((Reverse(new_heatloss), new_state));
                }
            }
        }

        format!("No path found")
    }

    fn part_two(&self, input: &str) -> String {

        let target_x = input.lines().next().unwrap().chars().count() as i32 - 1;
        let target_y = input.lines().count() as i32 - 1;

        let heatmap: HashMap<(i32, i32), usize> = input.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, ch)| {
                    ((x as i32, y as i32), ch.to_digit(10).unwrap() as usize)
                })
            }).collect();

        // dijkstra
        let mut heap: BinaryHeap<(Reverse<usize>, State)> = BinaryHeap::new();
        heap.push((Reverse(0), State::new((0, 0), Dir::None, 0)));

        let mut memo: HashMap<State, usize> = HashMap::new();

        while let Some((Reverse(cost), state)) = heap.pop() {
            if state.position == (target_x, target_y) {
                // reached end
                return format!("{}", cost);
            }

            if let Some(&best_cost) = memo.get(&state) {
                if cost >= best_cost {
                    // already found a better or equal way to this state
                    continue;
                }
            }

            memo.insert(state.clone(), cost);

            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter() {
                if dir == &state.direction.opposite() {
                    // cannot go backwards
                    continue;
                }

                if dir != &state.direction {
                    // need to move a minimum of 4 steps before turning, so take 4 steps in the new
                    // direction
                    let (delta_x, delta_y) = dir.delta();
                    let new_position = (
                        state.position.0 + delta_x * 4,
                        state.position.1 + delta_y * 4,
                    );

                    if !heatmap.contains_key(&new_position) {
                        // out of bounds
                        continue;
                    }

                    let step_cost: usize = (1..=4).map(|step| {
                        let step_position = (
                            state.position.0 + delta_x * step,
                            state.position.1 + delta_y * step,
                        );
                        *heatmap.get(&step_position).unwrap()
                    }).sum();

                    let new_cost = cost + step_cost;

                    let new_state = State::new(new_position, *dir, 4);

                    if memo.get(&new_state).map_or(true, |&old| new_cost < old) {
                        heap.push((Reverse(new_cost), new_state));
                    }
                } else {
                    let (delta_x, delta_y) = dir.delta();
                    let new_position = (state.position.0 + delta_x, state.position.1 + delta_y);

                    if !heatmap.contains_key(&new_position) {
                        // out of bounds
                        continue;
                    }

                    // take no more than 10 steps in the same direction
                    let new_consecutive = state.consecutive + 1;

                    if new_consecutive > 10 {
                        continue;
                    }

                    let new_cost = cost + heatmap.get(&new_position).unwrap();

                    let new_state = State::new(new_position, *dir, new_consecutive);

                    if memo.get(&new_state).map_or(true, |&old| new_cost < old) {
                        heap.push((Reverse(new_cost), new_state));
                    }
                }
            }
        }

        format!("No path found")
    }
}
