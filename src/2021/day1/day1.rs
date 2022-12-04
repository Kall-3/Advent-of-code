fn main() {
    let (_numbers, boards) = include_str!("input.txt").split_once("\d\n").unwrap();
    let board: Vec<&str> = boards.split("\n").collect();

    let mut vec: Vec<String> = Vec::new();
    let mut temp = String::from("");

    for b in board {
        println!("{}", b);
    }
    
    /*for v in vec {
        println!("{}", v);
    }*/
}