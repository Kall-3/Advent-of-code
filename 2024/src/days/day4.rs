use crate::problem::Problem;

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let board: Vec<Vec<char>> = input
            .lines()
            .map(move |line| {
                line.chars().collect()
            })
            .collect();

        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
        let word = ['X', 'M', 'A', 'S'];

        let mut occurences = 0;

        for (x, &ref row) in board.iter().enumerate() {
            for (y, &ref c) in row.iter().enumerate() {
                if c == &'X' {
                    for &(dx, dy) in &directions {
                        for step in 1..=3 {
                            let nx = x as i32 + dx * step;
                            let ny = y as i32 + dy * step;

                            if nx < 0 || nx >= board.len() as i32 || ny < 0 || ny >= row.len() as i32 {
                                break;
                            }
                            if board[nx as usize][ny as usize] != word[step as usize] {
                                break;
                            }
                            if step == 3 {
                                occurences += 1;
                            }
                        }
                    }
                }
            }
        }

        format!("{}", occurences)
    }

    fn part_two(&self, input: &str) -> String {
        let board: Vec<Vec<char>> = input
            .lines()
            .map(move |line| {
                line.chars().collect()
            })
            .collect();

        let mut occurences = 0;

        for (x, &ref row) in board.iter().enumerate().skip(1).take(board.len() - 2) {
            for (y, &ref c) in row.iter().enumerate().skip(1).take(row.len() - 2) {
                if c == &'A' {
                    // Check top left to bottom right
                    if board[x - 1][y - 1] == 'M' {
                        if board[x + 1][y + 1] != 'S' {
                            continue;
                        }
                    } else if board[x - 1][y - 1] == 'S' {
                        if board[x + 1][y + 1] != 'M' {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    // Check top right to bottom left
                    if board[x - 1][y + 1] == 'M' {
                        if board[x + 1][y - 1] != 'S' {
                            continue;
                        }
                    } else if board[x - 1][y + 1] == 'S' {
                        if board[x + 1][y - 1] != 'M' {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    occurences += 1;
                }
            }
        }

        format!("{}", occurences)
    }
}
