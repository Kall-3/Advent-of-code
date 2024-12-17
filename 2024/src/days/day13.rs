use crate::problem::Problem;

use regex::Regex;
use ndarray::array;
use ndarray::Array2;
use ndarray::Array1;
use ndarray_linalg::Solve;

pub struct DayThirteen {}

impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let re_xy = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let re_ans = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        let mut all_lines = input.lines().filter(|line| !line.is_empty());

        let mut sum = 0;

        while let (Some(button_a), Some(button_b), Some(prize)) = (all_lines.next(), all_lines.next(), all_lines.next()){
            let (x_a, y_a) = re_xy.captures(button_a).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();
            let (x_b, y_b) = re_xy.captures(button_b).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();
            let (x_p, y_p) = re_ans.captures(prize).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();

            let a: Array2<f64> = array![[x_a, x_b], [y_a, y_b]];
            let b: Array1<f64> = array![x_p, y_p];
            let x = a.solve_into(b).unwrap();

            if (x[0] - x[0].round()).abs() < 1e-6 && (x[1] - x[1].round()).abs() < 1e-6 {
                sum += x[0].round() as i32 * 3 + x[1].round() as i32;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let re_xy = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let re_ans = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        let mut all_lines = input.lines().filter(|line| !line.is_empty());
        let extra: f64 = 10_000_000_000_000.0;

        let mut sum = 0;

        while let (Some(button_a), Some(button_b), Some(prize)) = (all_lines.next(), all_lines.next(), all_lines.next()){
            let (x_a, y_a) = re_xy.captures(button_a).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();
            let (x_b, y_b) = re_xy.captures(button_b).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();
            let (x_p, y_p) = re_ans.captures(prize).map(|cap| {
                (cap[1].parse::<f64>().unwrap(), cap[2].parse::<f64>().unwrap())
            }).unwrap();

            let a: Array2<f64> = array![[x_a, x_b], [y_a, y_b]];
            let b: Array1<f64> = array![x_p + extra, y_p + extra];
            let x = a.solve_into(b).unwrap();

            // This is a bit of a hack, but it works. The precision of the linear solver is not
            // enough to get the exact answer, so played around with the precision until it worked.
            if (x[0] - x[0].round()).abs() < 1e-2 && (x[1] - x[1].round()).abs() < 1e-2 {
                sum += x[0].round() as i64 * 3 + x[1].round() as i64;
            }
        }

        format!("{}", sum)
    }
}
