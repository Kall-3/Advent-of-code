mod problem;
mod days;

use std::env;

use problem::Problem;
use days::*;

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{})),
        3 => Some(Box::new(DayThree{})),
        4 => Some(Box::new(DayFour{})),
        5 => Some(Box::new(DayFive{})),
        6 => Some(Box::new(DaySix{})),
        7 => Some(Box::new(DaySeven{})),
        8 => Some(Box::new(DayEight{})),
        9 => Some(Box::new(DayNine{})),
        10 => Some(Box::new(DayTen{})),
        11 => Some(Box::new(DayEleven{})),
        12 => Some(Box::new(DayTwelve{})),
        13 => Some(Box::new(DayThirteen{})),
        14 => Some(Box::new(DayFourteen{})),
        15 => Some(Box::new(DayFifteen{})),
        16 => Some(Box::new(DaySixteen{})),
        17 => Some(Box::new(DaySeventeen{})),
        18 => Some(Box::new(DayEighteen{})),
        19 => Some(Box::new(DayNineteen{})),
        20 => Some(Box::new(DayTwenty{})),
        21 => Some(Box::new(DayTwentyOne{})),
        22 => Some(Box::new(DayTwentyTwo{})),
        23 => Some(Box::new(DayTwentyThree{})),
        24 => Some(Box::new(DayTwentyFour{})),
        25 => Some(Box::new(DayTwentyFive{})),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <day> <optional: testing 0/1>", args[0]);
        return;
    }

    let day = args[1].parse::<usize>().expect("Day must be a number");
    let testing = args.get(2).map(|s| s == "1").unwrap_or(false);

    if let Some(problem) = day_to_problem(day) {
        let input_path = if testing {
            println!("Running with test data");
            format!("inputs/day{}_test.txt", day)
        } else {
            format!("inputs/day{}.txt", day)
        };

        let input = std::fs::read_to_string(input_path)
            .expect("Failed to read input file");

        println!("Day {}: Part one: {}", day, problem.part_one(&input));
        println!("Day {}: Part two: {}", day, problem.part_two(&input));
    } else {
        println!("Day {} not implemented", day);
    }
}
