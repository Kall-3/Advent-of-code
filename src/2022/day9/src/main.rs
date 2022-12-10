fn main() {
    part2();
}

#[derive(Clone, Debug, PartialEq)]
struct Pos {
    x_pos: i32,
    y_pos: i32
}

impl Pos {
    fn move_pos(&mut self, x: i32, y: i32) {
        self.x_pos += x;
        self.y_pos += y;
    }

    fn change_pos(&mut self, other: &Pos) {
        self.x_pos = other.x_pos;
        self.y_pos = other.y_pos;
    }

    fn touching(&self, other: &Pos) -> bool {
        (self.x_pos - other.x_pos).abs() <= 1 && (self.y_pos - other.y_pos).abs() <= 1
    }
    
    fn eq(&self, other: &Pos) -> bool {
        self.x_pos == other.x_pos && self.y_pos == other.y_pos
    }

    fn print(&self) {
        println!("({}, {})", self.x_pos, self.y_pos);
    }
}

fn print_grid(head: &Pos, tail: &Vec<Pos>) {
    let x_max = tail.iter().fold(head.x_pos, |a, b| if b.x_pos > a { b.x_pos } else { a });
    let x_min = tail.iter().fold(head.x_pos, |a, b| if b.x_pos < a { b.x_pos } else { a });
    
    let y_max = tail.iter().fold(head.y_pos, |a, b| if b.y_pos > a { b.y_pos } else { a });
    let y_min = tail.iter().fold(head.y_pos, |a, b| if b.y_pos < a { b.y_pos } else { a });
    
    println!("x: {} -> {}, y: {} -> {}", x_min, x_max, y_min, y_max);

    //for i in (-5..=15).rev() {
        //for j in -11..=14 {
    for i in (y_min..y_max).rev() {
        for j in (x_min..x_max) {
            if head == &(Pos { x_pos: j, y_pos: i }) {
                print!("H");
            } else if tail.contains(&Pos { x_pos: j, y_pos: i}) {
                print!("#");
            } else if i == 0 && j == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part2() {
    let instructions: Vec<(char, i32)> = include_str!("input.txt").lines().map(|x| x.split_once(' ').unwrap()).collect::<Vec<(&str,&str)>>().iter().map(|x| (x.0.parse::<char>().unwrap(), x.1.parse::<i32>().unwrap())).collect();
    
    let mut head_pos: Pos = Pos { x_pos: 0, y_pos: 0 };
    let mut tail_pos: Vec<Pos> = Vec::new();
    let mut visited: Vec<Pos> = Vec::new();
    for _ in 0..9 {
        tail_pos.push(head_pos.clone());
    }
    visited.push(tail_pos[8].clone());

    println!("tail vis: ");
    for pos in &visited {
        pos.print();
    }

    for i in instructions {
        match i.0 {
            'R' => {
                for _ in 0..i.1 {
                    head_pos.move_pos(1,0);
                    let mut prev = head_pos.clone();
                    
                    for pos in tail_pos.iter_mut() {
                        let horizontal_move = if &prev.x_pos > &pos.x_pos { 1 } else if &prev.x_pos < &pos.x_pos { -1 } else { 0 };
                        let vertical_move = if &prev.y_pos > &pos.y_pos { 1 } else if &prev.y_pos < &pos.y_pos { -1 } else { 0 };

                        if !pos.touching(&prev) {
                            pos.move_pos(horizontal_move, vertical_move);
                        }
                        prev = pos.clone();
                    }
                    if !visited.contains(tail_pos.last().unwrap()) {
                        visited.push(tail_pos.last().unwrap().clone());
                    }
                }
            }
            'L' => {
                for _ in 0..i.1 {
                    head_pos.move_pos(-1,0);
                    let mut prev = head_pos.clone();

                    for pos in tail_pos.iter_mut() {
                        let horizontal_move = if &prev.x_pos > &pos.x_pos { 1 } else if &prev.x_pos < &pos.x_pos { -1 } else { 0 };
                        let vertical_move = if &prev.y_pos > &pos.y_pos { 1 } else if &prev.y_pos < &pos.y_pos { -1 } else { 0 };

                        if !pos.touching(&prev) {
                            pos.move_pos(horizontal_move, vertical_move);
                        }
                        prev = pos.clone();
                    }
                    if !visited.contains(tail_pos.last().unwrap()) {
                        visited.push(tail_pos.last().unwrap().clone());
                    }
                }
            }
            'U' => {
                for _ in 0..i.1 {
                    head_pos.move_pos(0,1);
                    let mut prev = head_pos.clone();

                    for pos in tail_pos.iter_mut() {
                        let horizontal_move = if &prev.x_pos > &pos.x_pos { 1 } else if &prev.x_pos < &pos.x_pos { -1 } else { 0 };
                        let vertical_move = if &prev.y_pos > &pos.y_pos { 1 } else if &prev.y_pos < &pos.y_pos { -1 } else { 0 };

                        if !pos.touching(&prev) {
                            pos.move_pos(horizontal_move, vertical_move);
                        }
                        prev = pos.clone();
                    }
                    if !visited.contains(tail_pos.last().unwrap()) {
                        visited.push(tail_pos.last().unwrap().clone());
                    }
                }
            }
            'D' => {
                for _ in 0..i.1 {
                    head_pos.move_pos(0,-1);
                    let mut prev = head_pos.clone();

                    for pos in tail_pos.iter_mut() {
                        let horizontal_move = if &prev.x_pos > &pos.x_pos { 1 } else if &prev.x_pos < &pos.x_pos { -1 } else { 0 };
                        let vertical_move = if &prev.y_pos > &pos.y_pos { 1 } else if &prev.y_pos < &pos.y_pos { -1 } else { 0 };

                        if !pos.touching(&prev) {
                            pos.move_pos(horizontal_move, vertical_move);
                        }
                        prev = pos.clone();
                    }
                    if !visited.contains(tail_pos.last().unwrap()) {
                        visited.push(tail_pos.last().unwrap().clone());
                    }
                }
            }
            _ => println!("Not a instruction!")
        }
    }
    print_grid(&head_pos, &visited);
    println!("{}", visited.len());
}

fn part1() {
    let instructions: Vec<(char, i32)> = include_str!("input.txt").lines().map(|x| x.split_once(' ').unwrap()).collect::<Vec<(&str,&str)>>().iter().map(|x| (x.0.parse::<char>().unwrap(), x.1.parse::<i32>().unwrap())).collect();
    
    let mut head_pos = Pos { x_pos: 0, y_pos: 0 };
    let mut prev_head_pos;
    let mut tail_pos = head_pos.clone();
    let mut visited: Vec<Pos> = Vec::new();
    visited.push( tail_pos.clone() );

    for i in instructions {
        match i.0 {
            'R' => {
                for _ in 0..i.1 {
                    prev_head_pos = head_pos.clone();
                    head_pos.move_pos(1,0);
                    
                    if !head_pos.touching(&tail_pos) {
                        tail_pos = prev_head_pos.clone();
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos.clone());
                        }
                    }
                }
            }
            'L' => {
                for _ in 0..i.1 {
                    prev_head_pos = head_pos.clone();
                    head_pos.move_pos(-1,0);

                    if !head_pos.touching(&tail_pos) {
                        tail_pos = prev_head_pos.clone();
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos.clone());
                        }
                    }
                }
            }
            'U' => {
                for _ in 0..i.1 {
                    prev_head_pos = head_pos.clone();
                    head_pos.move_pos(0,1);

                    if !head_pos.touching(&tail_pos) {
                        tail_pos = prev_head_pos.clone();
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos.clone());
                        }
                    }
                }
            }
            'D' => {
                for _ in 0..i.1 {
                    prev_head_pos = head_pos.clone();
                    head_pos.move_pos(0,-1);

                    if !head_pos.touching(&tail_pos) {
                        tail_pos = prev_head_pos.clone();
                        if !visited.contains(&tail_pos) {
                            visited.push(tail_pos.clone());
                        }
                    }
                }
            }
            _ => println!("Not a instruction!")
        }
    }
    println!("{}", visited.len());
}
