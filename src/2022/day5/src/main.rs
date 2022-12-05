use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (crates, instructions) = include_str!("input.txt").split_once("\r\n\r\n").unwrap_or(("",""));
    let crates: Vec<Vec<char>> = crates.split("\r\n").map(|x| x.chars().collect::<Vec<char>>()).collect();
    //let crates: Vec<Vec<_>> = crates.iter().map(|x| x.iter().filter(|x| (x.clone().clone() as i32) >= 65 && (x.clone().clone() as i32) <= 90).collect::<Vec<_>>()).collect(); 
    let instructions: Vec<Vec<usize>> = instructions.split("\r\n").map(|x| x.split(" ").into_iter().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>()).collect();

    let rows = crates.len() - 1;
    let cols = (crates[0].len() + 1) / 4;
    let mut cra: Vec<Vec<char>> = vec![Vec::new(); cols]; // size 9
    for ii in (0..=rows).rev() { // initially 8 rows
        for jj in (1..=((cols*4)-3)).step_by(4) { // columns 33
            if crates[ii][jj] != ' ' {
                cra[(jj-1)/4].push(crates[ii][jj]);
            }
        }
    }

    for i in instructions {
        for _ in 0..i[0] {
            let temp = cra[i[1]-1].pop().unwrap();
            cra[i[2]-1].push(temp);
        }
    }

    let mut res = String::new();
    for l in &cra {
        res.push(*l.last().unwrap_or(&' '));
    }

    println!("{}", res);
}

fn part2() {
    let (crates, instructions) = include_str!("input.txt").split_once("\r\n\r\n").unwrap_or(("",""));
    let crates: Vec<Vec<char>> = crates.split("\r\n").map(|x| x.chars().collect::<Vec<char>>()).collect();
    let instructions: Vec<Vec<usize>> = instructions.split("\r\n").map(|x| x.split(" ").into_iter().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>()).collect();

    let rows = crates.len() - 1;
    let cols = (crates[0].len() + 1) / 4;
    let mut cra: Vec<Vec<char>> = vec![Vec::new(); cols]; // size 9
    for ii in (0..=rows).rev() { // initially 8 rows
        for jj in (1..=((cols*4)-3)).step_by(4) { // columns 33
            if crates[ii][jj] != ' ' {
                cra[(jj-1)/4].push(crates[ii][jj]);
            }
        }
    }
    let mut crates = cra;

    for i in instructions {
        let mut crane: VecDeque<_> = VecDeque::new();
        for _ in 0..i[0] {
            let temp = crates[i[1]-1].pop().unwrap();
            crane.push_front(temp);
        }
        //crates[i[2]-1].append(&mut *crane.iter().collect::<Vec<_>>());
        for b in &crane {
            crates[i[2]-1].push(*b);
        }
    }

    let mut res = String::new();
    for l in &crates {
        res.push(*l.last().unwrap_or(&' '));
    }

    println!("{}", res);
}