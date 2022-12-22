use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub struct Run2;
impl Run2 {
    pub fn part1() -> i32{
        part1()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: i32,
    temp_paths: Vec<String>,
    paths: HashMap<String, usize>
}

impl Valve {
    fn parse_fill(&mut self, line: &str) {
        let line = line.split_once(" has flow rate=").unwrap();
        self.name = line.0.split_once("Valve ").unwrap().1.to_string();
        let line = line.1.split_once("; tunnel").unwrap();
        self.flow_rate = line.0.parse().unwrap();
        self.temp_paths = line.1.split_once("to valve").unwrap().1.split_once(' ').unwrap().1.split(", ").map(|l| l.to_string()).collect();
    }

    // calculate shortcuts when all paths have been found
    fn paths_fill(&mut self, graph: &HashMap<String, Valve>) {
        let mut seen = HashSet::from([&self.name]);
        let mut queue = VecDeque::from([(&self.name, 0)]);
        let mut shortcuts = HashMap::new();
    
        while let Some((name, dist)) = queue.pop_front() {
            let v = graph.get(name).unwrap();
    
            for path in &v.temp_paths {
                if !seen.insert(path) {
                    continue
                }
                let room = graph.get(path).unwrap();
                if room.flow_rate != 0 && &room.name != &self.name {
                    shortcuts.insert(room.name.clone(), dist + 1);
                }
    
                queue.push_back((&room.name, dist + 1));
            }
        }
        
        self.temp_paths = Vec::new();
        self.paths = shortcuts
    }
}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.flow_rate.hash(state);

        let mut vec = self.paths.clone().into_iter().collect::<Vec<_>>();
        vec.sort();
        for path in &self.paths {
            path.hash(state);
        }
    }
}



#[derive(Debug, PartialEq, Eq, Clone)]
struct Walker {
    pos: String,
    time_remaining: usize,
    turned_on: HashSet<String>,
}

impl Hash for Walker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.time_remaining.hash(state);

        let mut vec = self.turned_on.clone().into_iter().collect::<Vec<_>>();
        vec.sort();
        for path in vec {
            path.hash(state);
        }
    }
}

#[derive(Default)]
struct Search {
    seen: HashMap<Walker, i32>,     // old states, reuse to skip duplicate searches
    best: i32,
}

impl Search {
    fn bfs(&mut self, walker: Walker, graph: &HashMap<String, Valve>, depth_check: &Vec<usize>, depth: usize, start_pressure: i32) -> i32 {
        if walker.time_remaining == 0 {
            return 0
        }
        if let Some(flow) = self.seen.get(&walker) {
            return *flow
        }

        // break early branches
        if depth_check.contains(&depth) {
            let mut max_flow = 0;
            for (name, valve) in graph {
                if !walker.turned_on.contains(name) {
                    max_flow += valve.flow_rate * (walker.time_remaining as i32 - 1)
                }
            }
            if (start_pressure + max_flow) < self.best {
                return -9999
            }
        }

        let mut best_move = 0;

        if !walker.turned_on.contains(&walker.pos) && walker.pos != "AA". to_string() {
            let new_walker = Walker { pos: walker.pos.clone(), time_remaining: walker.time_remaining - 1, turned_on: {let mut t = walker.turned_on.clone(); t.insert(walker.pos.clone()); t} };
            let new_pressure = graph.get(&walker.pos).unwrap().flow_rate * (walker.time_remaining as i32 - 1);
            best_move = best_move.max(self.bfs(new_walker, &graph, depth_check, depth + 1, start_pressure + new_pressure)) + new_pressure;
        } else {
            for (path, dist) in &graph.get(&walker.pos).unwrap().paths {
                if &walker.time_remaining > dist {
                    let new_walker = Walker { pos: path.clone(), time_remaining: walker.time_remaining - dist, turned_on: walker.turned_on.clone() };
                    best_move = best_move.max(self.bfs(new_walker, graph, depth_check, depth + 1, start_pressure));
                }
            }
        }
        
        self.seen.insert(walker, best_move);
        self.best = self.best.max(start_pressure + best_move);
        best_move
    }
}

fn part2() -> i32 {

    0
}

#[inline]
fn part1() -> i32 {
    let input = include_str!("input.txt").lines();
    let mut graph = HashMap::new();

    for line in input {
        let mut valve = Valve::default();
        valve.parse_fill(line);
        graph.insert(valve.name.clone(), valve);
    }

    let graph_ref = graph.clone();
    graph.retain(|name, valve| {
        valve.paths_fill(&graph_ref);
        !(valve.flow_rate == 0 && name != "AA")
    });

    let mut search = Search::default();
    search.bfs(Walker {pos: "AA".to_string(), time_remaining: 30, turned_on: HashSet::new()}, &graph, &vec![graph.len()/3, graph.len()*2/3], 0, 0)
}
