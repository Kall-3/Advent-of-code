use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut vec: Vec<i32> = Vec::new();
    let mut temp: i32 = 0;

    for line in f.lines() {
        let l = line?;
        if l == "" {
            vec.push(temp);
            temp = 0;
        } else {
            temp += l.parse::<i32>().unwrap();
        }
    }
    vec.sort();
    vec.reverse();
    let mut res: i32 = vec[0];
    res += vec[1];
    res += vec[2];
    println!("{}", res);
    Ok(())
}