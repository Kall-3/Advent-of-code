use std::cell::RefCell;
use std::rc::Rc;

struct Dir {
    pub name: String,
    pub size: u32,
    pub children: Vec<Rc<RefCell<Dir>>>,
    pub parent: Option<Rc<RefCell<Dir>>>
}

impl Dir {
    fn new(p_name: String, cur: Option<Rc<RefCell<Dir>>>) -> Dir {
        return Dir {
            size: 0,
            name: p_name,
            children: Vec::new(),
            parent: cur,
        }
    }
}

fn ret_size(dir: Rc<RefCell<Dir>>) {
    let dir_clone = Rc::clone(&dir);
    let y = dir_clone.borrow().size;
    if y > 4125990 {
        println!("{}", y);
    }
    for d in &dir_clone.borrow().children {
        let d_d = Rc::clone(&d);
        ret_size(d_d);
    }
}

fn main() {
    
    let input = include_str!("input.txt");

    let home = Rc::new(RefCell::new(Dir::new("/".to_string(), None)));
    let mut current = Rc::clone(&home);

    input
        .lines()
        .skip(1)
        .for_each(|l| {
            let (cmd1, cmd2) = l.split_once(" ").unwrap();
            match cmd1 {
                "$" => {
                    let cmd3 = cmd2.split(" ").collect::<Vec<&str>>();
                    if cmd3[0] == "cd" {
                        if cmd3[1] == ".." {
                            let current_clone = Rc::clone(&current);
                            current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                        } else {
                            let current_clone = Rc::clone(&current);
                            let c_children = &current_clone.borrow().children;
                            for child in c_children {
                                if child.borrow().name == cmd3[1] {
                                    current = Rc::clone(&child);
                                }
                            }
                        }
                    }
                },
                "dir" => {
                    let child = Rc::new(RefCell::new(Dir::new(cmd2.to_string(), Some(Rc::clone(&current)))));
                    current.borrow_mut().children.push(Rc::clone(&child));
                }
                _ => {
                    let t_size = cmd1.parse::<u32>().unwrap();
                    let mut c_current = Rc::clone(&current);
                    loop {
                        c_current.borrow_mut().size += t_size;
                        if c_current.borrow().name == "/" {
                            break;
                        }
                        let c_c_current = Rc::clone(&c_current);
                        c_current = Rc::clone(c_c_current.borrow().parent.as_ref().unwrap());
                    }
                }
            }
        }
    );

    let f = Rc::clone(&home);
    ret_size(f);
}

