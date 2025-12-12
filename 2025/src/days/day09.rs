use std::collections::VecDeque;

use crate::problem::Problem;

pub struct DayNine;

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let re = regex::Regex::new(r"(\d+),(\d+)").unwrap();

        let numbers: Vec<(isize, isize)> = re.captures_iter(input).map(|cap| {
            let a: isize = cap[1].parse().unwrap();
            let b: isize = cap[2].parse().unwrap();
            (a, b)
        }).collect();

        let mut max_area: isize = 0;

        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                let (a1, b1) = numbers[i];
                let (a2, b2) = numbers[j];

                let side_a = (a2 - a1).abs() + 1;
                let side_b = (b2 - b1).abs() + 1;

                let area = side_a * side_b;
                if area > max_area {
                    max_area = area;
                }
            }
        }

        format!("{}", max_area)
    }

    fn part_two(&self, input: &str) -> String {
        let re = regex::Regex::new(r"(\d+),(\d+)").unwrap();

        let mut boundarys: Vec<(isize, isize)> = re.captures_iter(input).map(|cap| {
            let a: isize = cap[1].parse().unwrap();
            let b: isize = cap[2].parse().unwrap();
            (a, b)
        }).collect();

        let mut xs: Vec<isize> = Vec::new();
        let mut ys: Vec<isize> = Vec::new();

        xs.push(0);
        ys.push(0);

        for &(x, y) in &boundarys {
            xs.push(x);
            xs.push(x + 1);
            ys.push(y);
            ys.push(y + 1);
        }

        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();

        xs.push(*xs.last().unwrap() + 1);

        fn cx(xs: &Vec<isize>, x: isize) -> usize {
            xs.binary_search(&x).unwrap()
        }
        fn cy(ys: &Vec<isize>, y: isize) -> usize {
            ys.binary_search(&y).unwrap()
        }

        let w = xs.len() - 1;
        let h = ys.len() - 1;

        // let mut cell_w = vec![0isize; w];
        // let mut cell_h = vec![0isize; h];
        // for i in 0..w {
        //     cell_w[i] = xs[i + 1] - xs[i];
        // }
        // for j in 0..h {
        //     cell_h[j] = ys[j + 1] - ys[j];
        // }

        let mut blocked = vec![vec![false; w]; h];  // [y][x]

        for &(x, y) in &boundarys {
            let ix = cx(&xs, x);
            let iy = cy(&ys, y);

            if ix < w && iy < h {
                blocked[iy][ix] = true;
            }
        }

        boundarys.push(boundarys[0]);
        for w in boundarys.windows(2) {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];

            let ix1 = cx(&xs, x1);
            let iy1 = cy(&ys, y1);
            let ix2 = cx(&xs, x2);
            let iy2 = cy(&ys, y2);

            if ix1 == ix2 {
                let j0 = iy1.min(iy2);
                let j1 = iy1.max(iy2);
                for j in j0..j1 {
                    blocked[j][ix1] = true;
                }
            } else if iy1 == iy2 {
                let i0 = ix1.min(ix2);
                let i1 = ix1.max(ix2);
                for i in i0..i1 {
                    blocked[iy1][i] = true;
                }
            }
        }

        let mut outside = vec![vec![false; w]; h];

        let mut q = VecDeque::new();
        q.push_back((0isize, 0isize));
        outside[0][0] = true;

        let dirs = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];

        while let Some((i, j)) = q.pop_front() {
            for (dx, dy) in dirs {
                let ni = i + dx;
                let nj = j + dy;
                if ni < 0 || nj < 0 || ni >= w as isize || nj >= h as isize {
                    continue;
                }
                let (xi, yj) = (ni as usize, nj as usize);
                if blocked[yj][xi] || outside[yj][xi] {
                    continue;
                }
                outside[yj][xi] = true;
                q.push_back((ni, nj));
            }
        }

        // calculate max area

        let mut max_area: isize = 0;

        for (i, (x1, y1)) in boundarys.iter().enumerate() {
            for (x2, y2) in boundarys.iter().skip(i + 1) {
                if x1 == x2 || y1 == y2 {
                    // skip aligned points, unlikely max area
                    continue;
                }

                // indexes of the rectangle corners
                let ix1 = cx(&xs, *x1);
                let iy1 = cy(&ys, *y1);
                let ix2 = cx(&xs, *x2);
                let iy2 = cy(&ys, *y2);

                // rectangle index boundary ranges
                let i0 = ix1.min(ix2);
                let i1 = ix1.max(ix2);
                let j0 = iy1.min(iy2);
                let j1 = iy1.max(iy2);

                // check if the rectangle boundary is blocked at any point
                let mut blocked_rect = false;
                for i in i0..i1 {
                    if outside[j0][i] || outside[j1][i] {
                        blocked_rect = true;
                        break;
                    }
                }
                for j in j0..j1 {
                    if outside[j][i0] || outside[j][i1] {
                        blocked_rect = true;
                        break;
                    }
                }

                if blocked_rect {
                    continue;
                }

                // calculate area
                let width = (x1 - x2).abs() + 1;
                let height = (y1 - y2).abs() + 1;
                max_area = max_area.max(width * height);
            }
        }

        format!("{}", max_area)
    }
}
