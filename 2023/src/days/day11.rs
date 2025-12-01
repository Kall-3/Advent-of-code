use crate::problem::Problem;

pub struct DayEleven;

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        // parse
        let mut map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        // space expansion
        let empty_rows: Vec<bool> = map.iter().map(|row| row.iter().all(|&c| c == '.')).collect();
        let empty_cols: Vec<bool> = (0..map[0].len())
            .map(|col_idx| (0..map.len()).all(|row_idx| map[row_idx][col_idx] == '.'))
            .collect();

        for (i, &is_empty) in empty_rows.iter().enumerate().rev() {
            if is_empty {
                map.insert(i, vec!['.'; map[0].len()]);
            }
        }

        for (j, &is_empty) in empty_cols.iter().enumerate().rev() {
            if is_empty {
                for row in &mut map {
                    row.insert(j, '.');
                }
            }
        }

        // find galaxies
        let galaxies: Vec<(i32, i32)> = map.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &c)| {
                    if c == '#' {
                        Some((i as i32, j as i32))
                    } else {
                        None
                    }
                })
            }).collect();

        // calculate distances
        let mut total: usize = 0;

        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let (x1, y1) = galaxies[i];
                let (x2, y2) = galaxies[j];

                let dist = ((x1 - x2).abs() + (y1 - y2).abs()) as usize;
                total += dist;
            }
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {
        // parse
        let map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        // find empty space
        let empty_rows: Vec<bool> = map.iter().map(|row| row.iter().all(|&c| c == '.')).collect();
        let empty_cols: Vec<bool> = (0..map[0].len())
            .map(|col_idx| (0..map.len()).all(|row_idx| map[row_idx][col_idx] == '.'))
            .collect();

        // find galaxies
        let galaxies: Vec<(isize, isize)> = map.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &c)| {
                    if c == '#' {
                        Some((j as isize, i as isize))
                    } else {
                        None
                    }
                })
            }).collect();

        // calculate distances
        let mut total: usize = 0;

        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let (x1, y1) = galaxies[i];
                let (x2, y2) = galaxies[j];

                let mut empty_cols_passed = 0;
                for c in (x1.min(x2) as usize + 1)..(x1.max(x2) as usize) {
                    if empty_cols[c] {
                        empty_cols_passed += 1;
                    }
                }
                let mut empty_rows_passed = 0;
                for r in (y1.min(y2) as usize + 1)..(y1.max(y2) as usize) {
                    if empty_rows[r] {
                        empty_rows_passed += 1;
                    }
                }

                let dist = ((x1 - x2).abs() + (y1 - y2).abs()) as usize
                    + (empty_cols_passed + empty_rows_passed) * 999_999;
                total += dist;
            }
        }

        format!("{}", total)
    }
}
