use crate::problem::Problem;

use std::collections::HashMap;

pub struct DayFive {}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        for rule in parts[0].lines() {
            let nums: Vec<usize> = rule
                .split("|")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect();
            rules.entry(nums[0]).or_insert(Vec::new()).push(nums[1]);
        }

        let books: Vec<Vec<usize>> = parts[1]
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            }).collect();

        let mut sum = 0;

        for book in books.iter() {
            let mut ok = true;
            'outer: for (i, page) in book.iter().enumerate() {
                if let Some(r) = rules.get(page) {
                    for j in 0..i {
                        if r.contains(&book[j]) {
                            ok = false;
                            break 'outer;
                        }
                    }
                }
            }
            if ok {
                sum += book[book.len() / 2];
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        for rule in parts[0].lines() {
            let nums: Vec<usize> = rule
                .split("|")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect();
            rules.entry(nums[0]).or_insert(Vec::new()).push(nums[1]);
        }

        let books: Vec<Vec<usize>> = parts[1]
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<usize>().unwrap_or(0))
                    .collect()
            }).collect();

        let mut incorrect = Vec::new();

        for book in books.iter() {
            'outer: for (i, page) in book.iter().enumerate() {
                if let Some(r) = rules.get(page) {
                    for j in 0..i {
                        if r.contains(&book[j]) {
                            incorrect.push(book.clone());
                            break 'outer;
                        }
                    }
                }
            }
        }

        let mut sum = 0;

        for book in incorrect.iter() {
            let mut new_book = vec![book[0].clone()];

            for i in 1..book.len() {
                let book = book.get(i).unwrap();
                let rule = if let Some(r) = rules.get(book) {
                    r
                } else {
                    new_book.push(book.clone());
                    continue;
                };

                let mut inserted = false;
                for j in 0..i {
                    if rule.contains(&new_book[j]) {
                        inserted = true;
                        new_book.insert(j, book.clone());
                        break;
                    }
                }
                if !inserted {
                    new_book.push(book.clone());
                }
            }

            sum += new_book[new_book.len() / 2];
        }

        format!("{}", sum)
    }
}
