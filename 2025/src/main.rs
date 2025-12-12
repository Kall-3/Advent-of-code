mod problem;
mod days;

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
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> <optional: testing 0/1> <optional: test file>", args[0]);
        return;
    }

    let day = args[1].parse::<usize>().expect("Day must be a number");
    let test_file_1 = args.get(2).cloned().unwrap_or_else(|| String::new());
    let test_file_2 = args.get(3).cloned().unwrap_or_else(|| String::new());
    let testing = !test_file_1.is_empty();

    if let Some(problem) = day_to_problem(day) {
        let (input_path_1, input_path_2) = if testing {
            if test_file_1 == "1" {
                (format!("inputs/day{:02}t", day), 
                 format!("inputs/day{:02}t", day))
            } else if !test_file_2.is_empty() {
                (format!("inputs/day{:02}{}", day, test_file_1),
                 format!("inputs/day{:02}{}", day, test_file_2))
            } else {
                (format!("inputs/day{:02}{}", day, test_file_1),
                 format!("inputs/day{:02}{}", day, test_file_1))
            }
        } else {
            (format!("inputs/day{:02}", day),
             format!("inputs/day{:02}", day))
        };

        let input_1 = std::fs::read_to_string(&input_path_1)
            .expect(&format!("Failed to read input file: {}", input_path_1));
        let input_2 = std::fs::read_to_string(&input_path_2)
            .expect(&format!("Failed to read input file: {}", input_path_2));

        println!("Day {} - Part One:\n{}", day, problem.part_one(&input_1));
        println!("Day {} - Part Two:\n{}", day, problem.part_two(&input_2));
    } else {
        println!("Day {} is not implemented yet.", day);
    }
}
