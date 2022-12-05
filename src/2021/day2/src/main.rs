fn main() {
    part1();
    part2();
}

fn part1() {
    let f: Vec<&str> = include_str!("input.txt").split("\r\n").collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for l in f {
        let (e1, e2) = l.split_once(" ").unwrap();

        if e1 == "forward" {
            horizontal += e2.parse::<i32>().unwrap();
        } else if e1 == "up" {
            depth -= e2.parse::<i32>().unwrap();
        } else {
            depth += e2.parse::<i32>().unwrap();
        }
    }
    
    println!("{}", depth*horizontal);
}

fn part2() {
    let f: Vec<&str> = include_str!("input.txt").split("\r\n").collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for l in f {
        let (e1, e2) = l.split_once(" ").unwrap();

        if e1 == "up" {
            aim -= e2.parse::<i32>().unwrap();
        } else if e1 == "down" {
            aim += e2.parse::<i32>().unwrap();
        } else {
            horizontal += e2.parse::<i32>().unwrap();
            depth += aim * e2.parse::<i32>().unwrap();
        }
    }
    
    println!("{}", depth*horizontal);
}