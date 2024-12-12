use crate::problem::Problem;

use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let mut nums1 = BinaryHeap::new();
        let mut nums2 = BinaryHeap::new();

        for line in input.lines() {
            let [num1, num2] = line.split_whitespace().collect::<Vec<&str>>()
                .try_into().unwrap();
            nums1.push(num1.parse::<i32>().unwrap());
            nums2.push(num2.parse::<i32>().unwrap());
        }

        let mut sum = 0;
        while let (Some(num1), Some(num2)) = (nums1.pop(), nums2.pop()) {
            sum += (num1 - num2).abs();
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let mut nums1 = Vec::new();
        let mut nums2 = HashMap::new();

        for line in input.lines() {
            let [num1, num2] = line.split_whitespace().collect::<Vec<&str>>()
                .try_into().unwrap();
            nums1.push(num1.parse::<i32>().unwrap());
            nums2.entry(num2.parse::<i32>().unwrap())
                .and_modify(|counter| *counter += 1).or_insert(1);
        }

        let mut similarity = 0;
        for num1 in nums1 {
            similarity += num1 * nums2.get(&num1).unwrap_or(&0);
        }

        format!("{}", similarity)
    }
}
