use std::collections::VecDeque;

fn main() {
    part2();
}

#[derive(Copy,Debug,Clone)]
enum Operator {
    Add,
    Mul
}

#[derive(Copy,Debug,Clone)]
enum Operand {
    Old,
    Num(u128)
}

#[derive(Copy,Debug,Clone)]
struct Operation {
    operator: Operator,
    operand: Operand
}

impl Operation {
    fn apply(&self, item: u128) -> u128 {
        let operand = match self.operand {
            Operand::Old => item,
            Operand::Num(n) => n
        };
        match self.operator {
            Operator::Add => operand + item,
            Operator::Mul => operand * item
        }
    }
}

#[derive(Debug,Clone)]
struct Monkey {
    number: u32,
    items: VecDeque<u128>,
    operation: Operation,
    denominator: u128,
    case_true: u32,
    case_false: u32,
    items_inspected: u32
}

fn part2() {
    let input = include_str!("input.txt").split("\r\n\r\n").collect::<Vec<&str>>().iter().map(|x| x.lines().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let monkeys: &mut Vec<Monkey> = &mut Vec::new();

    for monkey in input {
        let mut m = monkey.iter().filter(|x| x != &&"").into_iter();

        let number: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];
        let items: VecDeque<u128> = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect();
        let temp = m.next().unwrap().split(' ').collect::<Vec<&str>>()[6..8].iter().cloned().collect::<Vec<&str>>();
        let operator = match temp[0] {
            "+" => Operator::Add,
            _ => Operator::Mul
        };
        let operand = match temp[1] {
            "old" => Operand::Old,
            _ => Operand::Num(temp[1].parse().unwrap())
        };
        let operation = Operation { operator, operand };
        let denominator: u128 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u128>>()[0];
        let case_true: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];
        let case_false: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];

        monkeys.push( Monkey { number, items, operation, denominator, case_true, case_false, items_inspected: 0 });
    }

    let mut div_mod: u128 = 1;
    for m in monkeys.clone() {
        div_mod *= m.denominator;
    }

    for i in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_idx].clone();
            let items_len = monkey.items.len();

            for _ in 0..items_len {
                let worry: u128 = monkey.operation.apply(monkeys[monkey_idx].items.pop_back().unwrap() % div_mod) % div_mod;

                if worry % monkey.denominator == 0 {
                    monkeys[monkey.case_true as usize].items.push_front(worry % div_mod);
                } else {
                    monkeys[monkey.case_false as usize].items.push_front(worry % div_mod);
                }
                monkeys[monkey_idx].items_inspected += 1;
            }
        }
    }
    
    let mut res = monkeys.iter().map(|x| x.items_inspected as u128).collect::<Vec<u128>>();
    res.sort();
    let res: u128 = res.iter().rev().cloned().collect::<Vec<u128>>()[0..2].iter().product();
    
    println!("{}", res);
    
}

fn part1() {
    let input = include_str!("input.txt").split("\r\n\r\n").collect::<Vec<&str>>().iter().map(|x| x.lines().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let monkeys: &mut Vec<Monkey> = &mut Vec::new();

    for monkey in input {
        let mut m = monkey.iter().filter(|x| x != &&"").into_iter();

        let number: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];
        let items: VecDeque<u128> = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect();
        let temp = m.next().unwrap().split(' ').collect::<Vec<&str>>()[6..8].iter().cloned().collect::<Vec<&str>>();
        let operator = match temp[0] {
            "+" => Operator::Add,
            _ => Operator::Mul
        };
        let operand = match temp[1] {
            "old" => Operand::Old,
            _ => Operand::Num(temp[1].parse().unwrap())
        };
        let operation = Operation { operator, operand };
        let denominator: u128 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u128>>()[0];
        let case_true: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];
        let case_false: u32 = m.next().unwrap().replace(&[',', ':', '='][..], "").split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()[0];

        monkeys.push( Monkey { number, items, operation, denominator, case_true, case_false, items_inspected: 0 });
    }

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_idx].clone();
            let items_len = monkey.items.len();

            for _ in 0..items_len {
                let worry: u128 = monkey.operation.apply(monkeys[monkey_idx].items.pop_back().unwrap()) / 3;

                if worry % monkey.denominator == 0 {
                    monkeys[monkey.case_true as usize].items.push_front(worry);
                } else {
                    monkeys[monkey.case_false as usize].items.push_front(worry);
                }
                monkeys[monkey_idx].items_inspected += 1;
            }
        }
    }


    let mut res = monkeys.iter().map(|x| x.items_inspected).collect::<Vec<u32>>();
    res.sort();
    let res: u32 = res.iter().rev().cloned().collect::<Vec<u32>>()[0..2].iter().product();
    
    println!("{}", res);
    
}
