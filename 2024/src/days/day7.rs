use crate::problem::Problem;

pub struct DaySeven {}

fn recursive_eval(numbers: &Vec<i32>, num_idx: usize, sum: i64, target: &i64) -> bool {
    if num_idx == numbers.len() {
        return sum == *target;
    }

    let fst = recursive_eval(numbers, num_idx + 1, sum + numbers[num_idx] as i64, target);
    let snd = recursive_eval(numbers, num_idx + 1, sum * numbers[num_idx] as i64, target);

    fst || snd
}

fn recursive_eval_concat(numbers: &Vec<i32>, num_idx: usize, sum: i64, target: &i64) -> bool {
    if num_idx == numbers.len() {
        return sum == *target;
    }

    let fst = recursive_eval_concat(numbers, num_idx + 1, sum + numbers[num_idx] as i64, target);
    let snd = recursive_eval_concat(numbers, num_idx + 1, sum * numbers[num_idx] as i64, target);

    let num = (sum.to_string() + &numbers[num_idx].to_string()).parse::<i64>().unwrap();
    let trd = recursive_eval_concat(numbers, num_idx + 1, num, target);

    fst || snd || trd
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let equations: Vec<(i64, Vec<i32>)> = input
            .lines()
            .map(|line| {
                let p: Vec<&str> = line.split(": ").collect();
                let test_value = p[0].parse::<i64>().unwrap();
                let numbers = p[1]
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                (test_value, numbers)
            }).collect();

        let mut sum = 0;

        for (test_value, numbers) in equations.iter() {
            if recursive_eval(numbers, 1, numbers[0].into(), test_value) {
                sum += test_value;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let equations: Vec<(i64, Vec<i32>)> = input
            .lines()
            .map(|line| {
                let p: Vec<&str> = line.split(": ").collect();
                let test_value = p[0].parse::<i64>().unwrap();
                let numbers = p[1]
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                (test_value, numbers)
            }).collect();

        let mut sum = 0;

        for (test_value, numbers) in equations.iter() {
            if recursive_eval_concat(numbers, 1, numbers[0].into(), test_value) {
                sum += test_value;
            }
        }

        format!("{}", sum)
    }
}
