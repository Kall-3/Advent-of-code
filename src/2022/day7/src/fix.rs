fn main() {
    let input = include_str!("fix.txt");

    let mut min = 100000000;
    input
        .lines()
        .for_each(|line| {
            let s = line.parse::<u32>().unwrap();
            if s < min {
                min = s;
            }
        });
    println!("{}", min);
}