use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}
/*
procedure BFS(G, root) is
2      let queue be a queue
3      label root as explored
4      queue.enqueue(root)
5      while queue is not empty do
6          v := queue.dequeue()
7          if v is the goal then
8              return v
9          for all edges from v to w in G.adjacentEdges(v) do
10              if w is not labeled as explored then
11                  label w as explored
12                  w.parent := v
13                  queue.enqueue(w)
*/

fn bfs(graph: HashMap<(usize, usize), Vec<(usize, usize)>>, root: (usize, usize), goal: (usize, usize)) -> HashMap<(usize,usize), (usize, usize)> {
    let mut queue = VecDeque::from([root]);
    let mut visited = Vec::from([root]);
    let mut parents: HashMap<(usize,usize), (usize, usize)> = HashMap::new();
    while queue.len() != 0 {
        let v = queue.pop_back().unwrap();
        if v == goal {
            return parents;
        }
        for neighbour in graph.get(&v).unwrap_or(&vec![]) {
            if !visited.contains(neighbour) {
                visited.push(neighbour.clone());
                parents.insert(neighbour.clone(),v);
                queue.push_front(neighbour.clone());
            }
        }
    }
    return HashMap::new();
}

fn part2() {
    let mut input: Vec<Vec<u32>> = include_str!("input.txt").lines().map(|x| x.chars().map(|y| y as u32).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let rows: usize = input.len();
    let cols: usize = input[0].len();

    let start = input.iter().map(|x| x.iter().position(|r| r == &83_u32)).enumerate().find(|(_, b)| b.is_some()).unwrap();
    let start = (start.0, start.1.unwrap());
    input[start.0][start.1] = 'a' as u32;

    let mut start_poins = Vec::new();
    for (x, row) in input.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if col == &('a' as u32) {
                start_poins.push((x,y));
            }
        }
    }

    let end = input.iter().map(|x| x.iter().position(|r| r == &69_u32)).enumerate().find(|(_, b)| b.is_some()).unwrap();
    let end = (end.0, end.1.unwrap());
    input[end.0][end.1] = 123;

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for row in 0usize..rows {
        for col in 0usize..cols {
            for dir in vec![(1i32,0i32), (0,1), (-1,0), (0,-1)] {
                let new_dir = ((row as i32 + dir.0) as usize, (col as i32 + dir.1) as usize);
                //println!("({}, {}) + {:?} = {:?}", row, col, dir, new_dir);

                if new_dir.0 < rows && new_dir.1 < cols && (input[row][col].abs_diff(input[new_dir.0][new_dir.1]) <= 1 || (input[row][col] == 122 && new_dir == end)) {
                    //println!("from ({:?},{:?} with {:?}, to {:?} with {:?}",row, col, input[row][col], new_dir, input[new_dir.0][new_dir.1]);
                    
                    if let Some(x) = graph.get_mut(&(row, col)) {
                        x.push(new_dir);
                    } else {
                        graph.insert((row, col), vec![new_dir]);
                    }
                }
            }
        }
    }

    let mut best_res = u32::MAX;

    for start in start_poins {
        let parents = bfs(graph.clone(), start, end);
        if parents.len() != 0 {
            let mut res = 0;
            let mut next = parents.get(&end);
            loop {
                if next.is_some() {
                    res += 1;
                    next = parents.get(next.unwrap());
                } else {
                    break
                }
            }
            if res < best_res { best_res = res;}
        }
    }
    println!("{:?}", best_res);
}

fn part1() {
    let mut input: Vec<Vec<u32>> = include_str!("input.txt").lines().map(|x| x.chars().map(|y| y as u32).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let rows: usize = input.len();
    let cols: usize = input[0].len();

    let start = input.iter().map(|x| x.iter().position(|r| r == &83_u32)).enumerate().find(|(_, b)| b.is_some()).unwrap();
    let start = (start.0, start.1.unwrap());
    input[start.0][start.1] = 96;

    let end = input.iter().map(|x| x.iter().position(|r| r == &69_u32)).enumerate().find(|(_, b)| b.is_some()).unwrap();
    let end = (end.0, end.1.unwrap());
    input[end.0][end.1] = 123;

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for row in 0usize..rows {
        for col in 0usize..cols {
            for dir in vec![(1i32,0i32), (0,1), (-1,0), (0,-1)] {
                let new_dir = ((row as i32 + dir.0) as usize, (col as i32 + dir.1) as usize);
                //println!("({}, {}) + {:?} = {:?}", row, col, dir, new_dir);

                if new_dir.0 < rows && new_dir.1 < cols && (input[row][col].abs_diff(input[new_dir.0][new_dir.1]) <= 1 || (input[row][col] == 122 && new_dir == end)) {
                    //println!("from ({:?},{:?} with {:?}, to {:?} with {:?}",row, col, input[row][col], new_dir, input[new_dir.0][new_dir.1]);
                    
                    if let Some(x) = graph.get_mut(&(row, col)) {
                        x.push(new_dir);
                    } else {
                        graph.insert((row, col), vec![new_dir]);
                    }
                }
            }
        }
    }

    let parents = bfs(graph, start, end);
    let mut res = 0;
    let mut next = parents.get(&end);
    loop {
        if next.is_some() {
            res += 1;
            next = parents.get(next.unwrap());
        } else {
            break
        }
    }
    println!("{:?}", res);
}