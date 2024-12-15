use crate::problem::Problem;

pub struct DayNine {}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let numbers: Vec<i32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut disk: Vec<i64> = Vec::new();

        for (i, num) in numbers.iter().enumerate() {
            if i % 2 == 0 {
                disk.append(&mut vec![i as i64/2; *num as usize]);
            } else {
                disk.append(&mut vec![-1; *num as usize]);
            }
        }

        let (mut i, mut j) = (0, disk.len() - 1);
        while i < j {
            if disk[i] != -1 {
                i += 1;
            } else if disk[j] == -1 {
                j -= 1;
            } else {
                disk.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        let checksum = disk.iter()
            .filter(|&c| *c != -1)
            .enumerate()
            .map(|(i, n)| {
                n * i as i64
            }).sum::<i64>();

        format!("{}", checksum)
    }

    fn part_two(&self, input: &str) -> String {
        let numbers: Vec<i32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut disk: Vec<i64> = Vec::new();

        for (i, num) in numbers.iter().enumerate() {
            if i % 2 == 0 {
                disk.append(&mut vec![i as i64/2; *num as usize]);
            } else {
                disk.append(&mut vec![-1; *num as usize]);
            }
        }

        let (mut i, mut ii) = (0, 0);
        let mut j  = disk.len() - 1;
        let mut j_id = disk[j];

        while j_id > 0 {
            // Find size of the group
            while disk[j] != j_id {
                j -= 1;
            }
            let mut jj = j;
            while disk[jj - 1] == j_id {
                jj -= 1;
            }
            let j_size = j - jj + 1;

            // Find first empty slot from the left with enough space
            while ii < jj {
                // println!("try i ({}, {})", i, ii);
                while disk[i] != -1 {
                    i += 1;
                }
                ii = i;
                while ii + 1 < disk.len() && disk[ii + 1] == -1 { 
                    ii += 1;
                }
                if ii - i + 1 >= j_size {
                    break;
                }
                i = ii + 1;
            }

            // If no empty slot found, ignore group
            if ii >= jj {
                i = 0;
                ii = 0;
                j -= 1;
                j_id -= 1;
                continue;
            }

            // Move group to the left
            for k in 0..j_size {
                disk.swap(i + k, jj + k);
            }

            // Update pointers
            i = 0;
            ii = 0;
            j = jj;
            j_id -= 1;
        }

        let checksum = disk.iter()
            .enumerate()
            .filter(|(_, &c)| c != -1)
            .map(|(i, n)| {
                n * i as i64
            }).sum::<i64>();

        format!("{}", checksum)
    }
}
