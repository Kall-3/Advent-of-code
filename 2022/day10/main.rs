fn main() {
    part1();
    part2();
}

struct Instruction<'a> {
    name: &'a str,
    value: i32,
    life: i32
}

impl <'a> Instruction<'a> {
    fn print(&self) {
        println!("name: {}, value: {}, life: {}", self.name, self.value, self.life);
    }
}

fn part2() {
    let instructions = include_str!("input.txt").lines().map(|x| if x.split_once(' ').is_some() { x.split_once(' ').unwrap() } else { (x, "0") } ).collect::<Vec<(&str, &str)>>().iter().map(|x| (x.0, x.1.parse::<i32>().unwrap_or(0))).collect::<Vec<(&str, i32)>>();
    let mut instructions = instructions.iter();

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut inst = Instruction { name: "", value: 0, life: 0 };

    loop {
        if cycle % 40 == 0 {
            cycle = 0;
            println!();
        }
        if cycle >= register - 1 && cycle <= register + 1 {
            print!("#");
        } else {
            print!(".");
        }
        
        
        if inst.life <= 0 {
            let (name, value) = instructions.next().unwrap_or(&("stop",0));
            inst = Instruction {name: name, value: *value, life: if name == &"addx" { 2 } else { 1 }};
        }
        
        match inst.name {
            "addx" => {
                if inst.life == 1 { register += inst.value }
                inst.life -= 1;
            }
            "noop" => {
                inst.life -= 1;
            }
            "stop" => {
                println!("End of instructions");
                break
            }
            _ => {
                println!("Not a instruction!");
                break
            }
        }
        cycle += 1;
    }
}

fn part1() {
    let instructions = include_str!("input.txt").lines().map(|x| if x.split_once(' ').is_some() { x.split_once(' ').unwrap() } else { (x, "0") } ).collect::<Vec<(&str, &str)>>().iter().map(|x| (x.0, x.1.parse::<i32>().unwrap_or(0))).collect::<Vec<(&str, i32)>>();
    let mut instructions = instructions.iter();

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut inst = Instruction { name: "", value: 0, life: 0 };
    let mut interesting: Vec<i32> = Vec::new();

    loop {
        cycle += 1;
        
        if (cycle - 20) % 40 == 0 { 
            println!("Register * cycle at cycle {} is {}", cycle, cycle*register);
            interesting.push(cycle*register);
        }

        if inst.life <= 0 {
            let (name, value) = instructions.next().unwrap_or(&("stop",0));
            inst = Instruction {name: name, value: *value, life: if name == &"addx" { 2 } else { 1 }};
        }

        match inst.name {
            "addx" => {
                if inst.life == 1 { register += inst.value }
                inst.life -= 1;
            }
            "noop" => {
                inst.life -= 1;
            }
            "stop" => {
                println!("End of instructions");
                break
            }
            _ => {
                println!("Not a instruction!");
                break
            }
        }
    }

    println!("{}", interesting.iter().fold(0, |a, b| a + b));
}
