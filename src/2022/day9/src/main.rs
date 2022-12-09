fn main() {
    part1();
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

fn part1() {
    let instructions: Vec<(char, i32)> = include_str!("input.txt").lines().map(|x| x.split_once(' ').unwrap()).collect::<Vec<(&str,&str)>>().iter().map(|x| (x.0.parse::<char>().unwrap(), x.1.parse::<i32>().unwrap())).collect();
    
    let mut head_pos = Pos { x_pos: 0, y_pos: 0 };
    let mut prev_head_pos = head_pos.clone();
    let mut tail_pos = head_pos.clone();
    let mut visited: Vec<Pos> = Vec::new();
    visited.push( tail_pos.clone() );

    for i in instructions {
        match i.0 {
            'R' => {
                for j in 0..i.1 {
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
                for j in 0..i.1 {
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
                for j in 0..i.1 {
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
                for j in 0..i.1 {
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
