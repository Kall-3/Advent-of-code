use std::collections::VecDeque;

fn main() {
    part2()
}

fn part1() {
    let f = include_str!("input.txt");

    let mut head = VecDeque::new();
    for c in f.chars() {
        if head.contains(&c) {
            loop {
                if head.front() != Some(&c) {
                    head.pop_front();
                } else if head.front() == None {
                    break;
                } else {
                    head.pop_front();
                    break;
                }
            }
            head.push_back(c);
        } else {
            head.push_back(c);
        }
        
        for h in &head {
            print!("{}", h);
        }
        println!("");

        if head.len() == 4 {
            break
        }
    }

    let head: String = head.clone().into_iter().collect::<String>();
    let head: &str = &*head;
    let idx = f.find(head).unwrap();
    println!("{}", idx+4);
}

fn part2() {
    let mut head = VecDeque::new();
    
    let f: String = include_str!("input2.txt").chars().map(|x| if head.contains(&x) { ' ' } else { head.push_back(x) }).into_iter().collect();

    println!("{}", f);
}