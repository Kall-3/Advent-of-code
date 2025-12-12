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
        13 => Some(Box::new(DayThirteen{})),
        14 => Some(Box::new(DayFourteen{})),
        15 => Some(Box::new(DayFifteen{})),
        16 => Some(Box::new(DaySixteen{})),
        17 => Some(Box::new(DaySeventeen{})),
        18 => Some(Box::new(DayEighteen{})),
        19 => Some(Box::new(DayNineteen{})),
        20 => Some(Box::new(DayTwenty{})),
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
    let testing = args.get(2).map(|s| s == "1").unwrap_or(false);

    let (test_file_1, test_file_2) = if args.len() >= 5 {
        (args[3].clone(), args[4].clone())
    } else if args.len() == 4 {
        (args[3].clone(), String::new())
    } else {
        (String::new(), String::new())
    };

    if let Some(problem) = day_to_problem(day) {
        let (input_path_1, input_path_2) = if testing {
            if !test_file_1.is_empty() && !test_file_2.is_empty() {
                (format!("inputs/day{:02}_{}.txt", day, test_file_1),
                 format!("inputs/day{:02}_{}.txt", day, test_file_2))
            } else if !test_file_1.is_empty() {
                (format!("inputs/day{:02}_{}.txt", day, test_file_1),
                 format!("inputs/day{:02}_{}.txt", day, test_file_1))
            } else {
                (format!("inputs/day{:02}_test.txt", day), 
                 format!("inputs/day{:02}_test.txt", day))
            }
        } else {
            (format!("inputs/day{:02}.txt", day),
             format!("inputs/day{:02}.txt", day))
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
