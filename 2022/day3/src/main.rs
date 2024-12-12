use std::collections::HashSet;


fn main() {
    let res1 = part1();
    let res2 = part2();
    println!("{}, {}", res1, res2);
}

fn part1() -> i32 {
    let f: Vec<&str> = include_str!("input.txt").split("\n").collect();
    let mut prios: Vec<i32> = Vec::new();

    for l in f {
        let (first, last) = l.split_at(l.len()/2);
        let mut set: HashSet<i32> = HashSet::new();

        for c in first.chars() {
            if last.contains(c) {
                set.insert(c as i32);
            }
        }
        prios.append(&mut set.into_iter().collect());
    }

    let mut res = 0;
    for c in prios {
        if c > 96 { //lower case, prio 1-26
            res += c - 96;
        } else { //upper case, prio 27-52
            res += c - 38;
        }
    }
    res
}

fn part2() -> i32 {
    let f: Vec<Vec<&str>> = include_str!("input.txt").split("\n").collect::<Vec<&str>>().chunks(3).map(|x| x.to_vec()).collect();
    let mut prios: Vec<i32> = Vec::new();

    for v in f {
        for c in v[0].chars() {
            if v[1].contains(c) {
                if v[2].contains(c) {
                    prios.push(c as i32);
                    break;
                }
            }
        }
    }

    let mut res = 0;
    for c in prios {
        if c > 96 { //lower case, prio 1-26
            res += c - 96;
        } else { //upper case, prio 27-52
            res += c - 38;
        }
    }
    res
}