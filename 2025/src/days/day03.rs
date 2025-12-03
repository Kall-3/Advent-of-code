use crate::problem::Problem;

pub struct DayThree;

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {

        let mut total: u32 = 0;

        for line in input.lines() {
            let mut fst: u8 = 0;
            let mut fst_idx: usize = 0;
            let mut snd: u8 = 0;

            for (i, char) in line.chars().enumerate().take(line.len() - 1) {
                let num = char.to_digit(10).unwrap() as u8;
                if num > fst {
                    fst = num;
                    fst_idx = i;
                }
            }

            for char in line.chars().skip(fst_idx + 1) {
                let num = char.to_digit(10).unwrap() as u8;
                if num > snd {
                    snd = num;
                }
            }

            total += (fst as u32) * 10 + (snd as u32);
        }

        format!("{}", total)
    }

    fn part_two(&self, input: &str) -> String {

        let mut total: usize = 0;

        for line in input.lines() {
            
            let bats: Vec<u8> = line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            let mut selected: Vec<u8> = Vec::new();
            let mut taken: u8 = 0;
            let mut pos: usize = 0;
            let mut skipped: u8 = 0;

            while taken < 12 {
                let window = bats.len() - (11 + skipped as usize);
                let mut max: u8 = 0;
                let mut max_pos: usize = pos;

                for i in pos..pos+window {
                    if bats[i] > max {
                        max = bats[i];
                        max_pos = i;
                    }
                }

                selected.push(max);
                taken += 1;
                skipped += (max_pos - pos) as u8;
                pos = max_pos + 1;
            }

            total += selected.iter().fold(0usize, |acc, &x| acc * 10 + x as usize);
        }

        format!("{}", total)
    }
}
