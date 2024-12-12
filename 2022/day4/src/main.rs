fn main() {
    part1();
    part2();
}

fn part1() {
    let f: Vec<Vec<&str>> = include_str!("input.txt").split("\r\n").into_iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<_>>();

    let mut res = 0;
    for l in f {
        let e: Vec<i32> = l.iter().map(|x| x.split("-").collect::<Vec<&str>>()).flatten().map(|x| x.trim_end_matches('\n').parse::<i32>().unwrap_or(i32::MAX)).collect();
        if (e[0] <= e[2] && e[1] >= e[3]) || (e[2] <= e[0] && e[3] >= e[1]) {
            res += 1;
        }
    }
    println!("{}", res);
}

fn part2() {
    let f: Vec<Vec<&str>> = include_str!("input.txt").split("\r\n").into_iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<_>>>();

    let mut res = 0;
    for l in f {
        let e: Vec<i32> = l.iter().map(|x| x.split("-").collect::<Vec<&str>>()).flatten().map(|x| x.trim_end_matches('\n').parse::<i32>().unwrap_or(i32::MAX)).collect();
        if (e[0] >= e[2] && e[1] <= e[2]) || (e[0] <= e[3] && e[1] >= e[3]) || (e[0] >= e[2] && e[1] <= e[3]) {
            res += 1;
        } else if (e[2] >= e[0] && e[3] <= e[0]) || (e[2] <= e[1] && e[3] >= e[1]) || (e[2] >= e[0] && e[3] <= e[1]) {
            res += 1;
        }
    }
    println!("{}", res);
}