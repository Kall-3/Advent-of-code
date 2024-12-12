use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut r = 0;
    for line in f.lines() {
        let l = line?;
        let vec: Vec<&str> = l.split(' ').collect();
        if vec[0] == "A" {
            if vec[1] == "X" { r += 3 }
            else if vec[1] == "Y" { r += 4 }
            else { r += 8 }
        } else if vec[0] == "B" {
            if vec[1] == "X" { r += 1 }
            else if vec[1] == "Y" { r += 5 }
            else { r += 9 }
        } else {
            if vec[1] == "X" { r += 2 }
            else if vec[1] == "Y" { r += 6 }
            else { r += 7 }
        }
    }
    println!("{}", r);
    Ok(())
}
