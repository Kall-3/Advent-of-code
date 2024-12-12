use core::str::Chars;
use std::cmp::Ordering;
use itertools::Itertools;

fn main() {
    part1();
    part2()
}

#[derive(Debug,Clone)]
enum Node {
    Leaf(i32),
    Branch(Vec<Node>)
}

impl Node {
    fn parse_input(ch: &mut Chars) -> Node {
        let mut result = Vec::new();
        let mut num = -1;    
        
        while let Some(c) = ch.next() {
            match c {
                '[' => result.push(Self::parse_input(ch)),
                ',' => {if num != -1 {result.push(Self::Leaf(num))}; num = -1},
                ']' => {if num != -1 {result.push(Self::Leaf(num))}; return Node::Branch(result)},
                d => {if num == -1 {num = 0}; num = (num * 10) + d.to_digit(10).unwrap_or(0) as i32},
            }
        }

        Node::Branch(result)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == Ordering::Equal
    }
}


fn compare(first: &Node, second: &Node) -> Ordering {
    match (first, second) {
        (Node::Leaf(f), Node::Leaf(s)) => f.cmp(s),
        (Node::Leaf(f), s_b) => compare(&Node::Branch(vec![Node::Leaf(*f)]), s_b),
        (f_b, Node::Leaf(s)) => compare(f_b, &Node::Branch(vec![Node::Leaf(*s)])),
        (Node::Branch(f_b), Node::Branch(s_b)) => {
            let mut i = 0;
            while i < f_b.len() && i < s_b.len() {
                match compare(&f_b[i], &s_b[i]) {
                    Ordering::Equal => {}
                    other => return other,
                };
                i += 1;
            }
            f_b.len().cmp(&s_b.len())
        }
    }
}

fn part2() {
    let mut input: Vec<_> = include_str!("input.txt")
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| Node::parse_input(&mut x[1..].chars()))
        .collect::<Vec<_>>();
    let divider_nodes = &mut vec![Node::Branch(vec![Node::Branch(vec![Node::Leaf(2)])]), Node::Branch(vec![Node::Branch(vec![Node::Leaf(6)])])];
    input.append(&mut divider_nodes.clone());
    input.sort_by(|f, s| compare(f, s));
    let decoder_key: i32 = input
        .iter()
        .enumerate()
        .filter(|(_, k)| divider_nodes.contains(&k))
        .map(|(i, _)| i as i32 + 1)
        .product();

    println!("{decoder_key:?}")
}

fn part1() {
    let input: Vec<_> = include_str!("input.txt")
        .split("\r\n\r\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split("\r\n")
            .map(|x| Node::parse_input(&mut x[1..].chars()))
            .into_iter()
            .next_tuple::<(Node, Node)>()
            .unwrap())
        .collect();

    let res: i32 = input
        .iter()
        .map(|(first, second)| compare(first, second))
        .enumerate()
        .filter(|(_, c)| c == &Ordering::Less)
        .map(|(i, _)| i as i32 + 1)
        .sum();
    
    println!("{res:?}")
}