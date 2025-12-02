use crate::problem::Problem;

pub struct DayThirteen;

impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let mut ans: usize = 0;

        'outer: for puzzle in input.split("\n\n") {
            // process each puzzle block
            let lines: Vec<Vec<char>> = puzzle.lines()
                .map(|line| line.chars().collect())
                .collect();

            // check row reflections
            let mut rows: Vec<String> = Vec::new();
            for line in lines.iter() {
                rows.push(line.iter().collect());
            }

            for i in 0..rows.len() - 1 {
                let mut n = i as isize;
                let mut m = n + 1;
                loop {
                    if n < 0 || m >= rows.len() as isize {
                        // reflection!
                        ans += (i + 1) * 100;
                        continue 'outer;
                    }
                    if rows[n as usize] != rows[m as usize] {
                        // no reflection
                        break;
                    }
                    n -= 1;
                    m += 1;
                }
            }

            // check column reflections
            let mut cols: Vec<String> = Vec::new();
            for col_idx in 0..rows[0].len() {
                let col: String = (0..rows.len()).map(|row_idx| {
                    rows[row_idx].chars().nth(col_idx).unwrap()
                }).collect();
                cols.push(col);
            }

            for i in 0..rows[0].len() - 1 {
                let mut n = i as isize;
                let mut m = n + 1;
                loop {
                    if n < 0 || m >= cols.len() as isize {
                        // reflection!
                        ans += i + 1;
                        continue 'outer;
                    }
                    if cols[n as usize] != cols[m as usize] {
                        // no reflection
                        break;
                    }
                    n -= 1;
                    m += 1;
                }
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {
        let mut ans: usize = 0;

        'outer: for puzzle in input.split("\n\n") {
            // process each puzzle block
            let lines: Vec<Vec<char>> = puzzle.lines()
                .map(|line| line.chars().collect())
                .collect();

            // check row reflections
            let rows: Vec<Vec<char>> = lines;

            for i in 0..rows.len() - 1 {
                if find_reflection(&rows, i as i32, (i as i32) + 1, false) {
                    ans += (i + 1) * 100;
                    continue 'outer;
                }
            }

            // check column reflections
            let mut cols: Vec<Vec<char>> = Vec::new();
            for col_idx in 0..rows[0].len() {
                let col: Vec<char> = (0..rows.len()).map(|row_idx| {
                    rows[row_idx][col_idx]
                }).collect();
                cols.push(col);
            }

            for i in 0..cols.len() - 1 {
                if find_reflection(&cols, i as i32, (i as i32) + 1, false) {
                    ans += i + 1;
                    continue 'outer;
                }
            }
        }

        format!("{}", ans)
    }
}

fn find_reflection(list: &Vec<Vec<char>>, i: i32, j: i32, smudge_fixed: bool) -> bool {
    if i < 0 || j as usize >= list.len() {
        // reflection!
        if smudge_fixed {
            return true;
        } else {
            return false;
        }
    }

    let diffs = list[i as usize].iter().zip(list[j as usize].iter()).fold(0, |acc, (a, b)| {
        if a != b { acc + 1 } else { acc }
    });

    if diffs == 0 {
        // smudge not fixed yet
        return find_reflection(list, i - 1, j + 1, smudge_fixed);
    } else if diffs == 1 {
        // fixed smudge, but rest needs to match
        return find_reflection(list, i - 1, j + 1, true);
    } else {
        // no reflection
        return false;
    }
}
