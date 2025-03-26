use crate::problem::Problem;

use regex::Regex;

pub struct DaySeventeen {}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {

        let reg_re = Regex::new(r"Register (.): (\d+)").unwrap();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        let mut program: Vec<i32> = Vec::new();

        for line in input.lines() {
            if let Some(caps) = reg_re.captures(line) {
                let reg = &caps[1];
                let val = caps[2].parse::<i32>().unwrap();

                match reg {
                    "A" => a = val,
                    "B" => b = val,
                    "C" => c = val,
                    _ => panic!("Unknown register: {}", reg),
                }
            } else if !line.is_empty() {
                program = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|c| {
                        c.parse::<i32>().unwrap()
                    }).collect::<Vec<i32>>();
            }
        }

        let combo = |o: i32, a: i32, b: i32, c: i32| -> i32 {
            match o {
                0..=3 => o,
                4 => a,
                5 => b,
                6 => c,
                7 => panic!("Reserved combo operand 7."),
                _ => panic!("Unknown combo operand: {}", o),
            }
        };

        let mut output: Vec<String> = Vec::new();

        let mut pointer = 0;
        while pointer < program.len() {
            let instruction = program[pointer];
            let operand = program[pointer + 1];

            match instruction {
                0 => {
                    a = a / 2_i32.pow(combo(operand, a, b, c) as u32);
                }
                1 => {
                    b ^= operand;
                }
                2 => {
                    b = combo(operand, a, b, c) % 8;
                }
                3 => {
                    if a != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    b ^= c;
                }
                5 => {
                    output.push((combo(operand, a, b, c) % 8).to_string());
                }
                6 => {
                    b = a / 2_i32.pow(combo(operand, a, b, c) as u32);
                }
                7 => {
                    c = a / 2_i32.pow(combo(operand, a, b, c) as u32);
                }
                _ => panic!("Unknown instruction: {}", instruction),
            };

            pointer += 2;
        }

        format!("{}", output.join(","))
    }

    fn part_two(&self, input: &str) -> String {
        let program = input.lines().last().unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(|c| {
                c.parse::<usize>().unwrap()
            }).collect::<Vec<usize>>();

        if let Some(a) = get_a(&program) {
            return format!("{}", a);
        } else {
            return format!("No solution found.");
        }
    }
}

fn simulate_loop(a: usize) -> usize {
    let mut b = a % 8;
    b ^= 1;
    let c = a / (1 << b);
    b ^= 5;
    b ^= c;
    b % 8
}

fn get_a(program: &[usize]) -> Option<usize> {
    println!("Program: {:?}", program);
    let mut stack = vec![(0, program.len())];
    while let Some((a, depth)) = stack.pop() {
        println!("{} {}", a, depth);
        if depth == 0 {
            return Some(a);
        }

        // 164 546 487 349 181

        for b in 0..8 {
            let a = (a << 3) | b;
            if simulate_loop(a) == program[depth - 1] {
                stack.push((a, depth - 1));
            }
        }
    }
    None
}
