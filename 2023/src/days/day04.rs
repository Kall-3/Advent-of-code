use crate::problem::Problem;

pub struct DayFour;

use std::collections::HashSet;
use regex::Regex;

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)").unwrap();

        let mut points = 0;

        for line in input.lines() {
            let mut winning_numbers: HashSet<&str> = HashSet::new();

            for m in re.find_iter(line.split(':').nth(1).unwrap().split('|').next().unwrap()) {
                winning_numbers.insert(m.as_str());
            }

            let mut wins = 0;

            for m in re.find_iter(line.split('|').nth(1).unwrap()) {
                if winning_numbers.contains(&m.as_str()) {
                    wins += 1;
                }
            }

            if wins > 0 {
                points += 1 * 2_u32.pow(wins - 1);
            }
        }

        format!("{}", points)
    }

    fn part_two(&self, input: &str) -> String {
        let mut scratch_cards: Vec<u32> = vec![1; input.lines().count()];

        let re = Regex::new(r"(\d+)").unwrap();

        for line in input.lines() {
            let card = re.captures(line).unwrap().get(0).unwrap().as_str().parse::<u32>().unwrap();

            let mut winning_numbers: HashSet<&str> = HashSet::new();
            for m in re.find_iter(line.split(':').nth(1).unwrap().split('|').next().unwrap()) {
                winning_numbers.insert(m.as_str());
            }

            let mut wins = 0;
            for m in re.find_iter(line.split('|').nth(1).unwrap()) {
                if winning_numbers.contains(&m.as_str()) {
                    wins += 1;
                }
            }

            for i in card..(card+wins) {
                scratch_cards[i as usize] += scratch_cards[card as usize - 1];
            }
        }

        let total = scratch_cards.iter().sum::<u32>();

        format!("{}", total)
    }
}
