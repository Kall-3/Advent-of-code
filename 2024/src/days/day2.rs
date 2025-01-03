use crate::problem::Problem;

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let reports: Vec<Vec<i32>> = input.lines().map(|line| {
            line.split_whitespace().map(|num| num.parse().unwrap()).collect()
        }).collect();

        let m = reports.len();

        let mut problems = 0;
        for report in reports.into_iter() {
            let direction = report[0] - report[1];
            for level in report.windows(2) {
                let diff = level[0] - level[1];
                if diff.abs() < 1 || diff.abs() > 3 || diff * direction < 0 {
                    problems += 1;
                    break;
                }
            }
        }

        format!("{}", m - problems)
    }

    fn part_two(&self, input: &str) -> String {
        let reports: Vec<Vec<i32>> = input.lines().map(|line| {
            line.split_whitespace().map(|num| num.parse().unwrap()).collect()
        }).collect();

        let len = reports.len();

        let mut problems = 0;

        for report in reports.into_iter() {
            let direction = report[0] - report[1];
            let (mut i, mut j, mut tolerance) = (0, 1, 0);
            while j < report.len() {
                let diff = report[i] - report[j];
                if diff.abs() < 1 || diff.abs() > 3 || diff * direction < 0 {
                    if tolerance == 0 {
                        tolerance = 1;

                        if j + 1 < report.len() {
                            let diff1 = report[i] - report[j + 1];
                            let diff2 = report[j] - report[j + 1];
                            if diff1.abs() > 0 && diff1.abs() < 4 && diff1 * direction > 0 {
                                j += 1;
                                continue;
                            } else if diff2.abs() > 0 && diff2.abs() < 4 && diff2 * direction > 0 {
                                i = j;
                                j += 1;
                                continue;
                            }
                            problems += 1;
                            break;
                        }
                    } else {
                        problems += 1;
                        break;
                    }
                }
                i = j;
                j += 1;
            }
        }

        format!("{}", len - problems)
    }
}
