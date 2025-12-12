use crate::problem::Problem;

pub struct DaySix;

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let num_re = regex::Regex::new(r"(\d+)").unwrap();
        let op_re = regex::Regex::new(r"[*+]").unwrap();

        let len = num_re.captures_iter(input.lines().next().unwrap()).count();

        let mut problems: Vec<Vec<i32>> = vec![Vec::new(); len];
        let mut ops: Vec<char> = Vec::new();

        for line in input.lines() {
            for (i, cap) in num_re.captures_iter(line).enumerate() {
                let value: i32 = cap[1].parse().unwrap();
                problems[i].push(value);
            }

            for cap in op_re.captures_iter(line) {
                let op_char = cap[0].chars().next().unwrap();
                ops.push(op_char);
            }
        }

        let mut ans: usize = 0;

        for i in 0..problems.len() {
            if ops[i] == '*' {
                ans += problems[i].iter().fold(1_usize, |acc, &x| acc * x as usize);
            } else {
                ans += problems[i].iter().map(|n| *n as usize).sum::<usize>();
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {
        let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut ops: Vec<char> = input.pop().unwrap().into_iter().filter(|c| *c != ' ').rev().collect();

        let mut ans: usize = 0;
        let mut nums: Vec<usize> = Vec::new();

        for col in 0..input[0].len() {
            if input.iter().all(|row| row[col] == ' ') {
                // Found a separator column
                if ops.pop().unwrap() == '*' {
                    ans += nums.iter().product::<usize>();
                } else {
                    ans += nums.iter().sum::<usize>();
                }
                nums.clear();
                continue;
            }

            let num: usize = input.iter()
                .map(|row| row[col])
                .fold(String::new(), |mut acc, c| { acc.push(c); acc })
                .trim()
                .parse()
                .unwrap();

            nums.push(num);
        }

        if ops.pop().unwrap() == '*' {
            ans += nums.iter().product::<usize>();
        } else {
            ans += nums.iter().sum::<usize>();
        }

        format!("{}", ans)
    }
}
