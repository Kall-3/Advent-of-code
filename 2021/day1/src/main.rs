use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut f = include_str!("input.txt").split("\r\n").map(|x| x.parse::<i128>().unwrap_or(i128::MAX)).collect::<Vec<i128>>().into_iter();
    
    let mut res = 0;
    let mut prev = f.next().unwrap();
    for e in f {
        if prev < e { res += 1; }
        prev = e;
    }

    println!("{}", res);
}

fn part2() {
    let binding = include_str!("input.txt").split("\r\n").map(|x| x.parse::<i128>().unwrap_or(i128::MAX)).collect::<Vec<i128>>();
    let f = binding.iter();

    let mut res = 0;
    let mut binding = f.clone();
    let mut prev: VecDeque<&i128> = VecDeque::from([binding.next().unwrap(), binding.next().unwrap(), binding.next().unwrap()]);
    let mut prev_sum: i128 = prev.iter().copied().sum();
    
    for e in f {
        prev.pop_front();
        prev.push_back(e);
        let sum = prev.iter().copied().sum();
        if prev_sum < sum { res += 1; }
        prev_sum = sum;
    }

    println!("{}", res);
}