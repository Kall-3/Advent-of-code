fn main() {
    part1();
    part2();
}

fn part2() {
    let grid: Vec<Vec<i32>> = include_str!("input.txt").lines().map(|x| x.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut best_score = 0;

    for row_index in 1..rows-1 {
        for col_index in 1..cols-1 {
            let mut score = Vec::new();

            let mut temp = 0;
            for i in 1..=col_index {
                temp += 1;
                if !(grid[row_index][col_index - i] < grid[row_index][col_index]) {
                    break;
                }
            }
            score.push(temp);
            
            let mut temp = 0;
            for i in 1..cols-col_index {
                temp += 1;
                if !(grid[row_index][col_index + i] < grid[row_index][col_index]) {
                    break;
                }
            }
            score.push(temp);
            
            let mut temp = 0;
            for i in 1..=row_index {
                temp += 1;
                if !(grid[row_index - i][col_index] < grid[row_index][col_index]) {
                    break;
                }
            }
            score.push(temp);

            let mut temp = 0;
            for i in 1..rows-row_index {
                temp += 1;
                if !(grid[row_index + i][col_index] < grid[row_index][col_index]) {
                    break;
                }
            }
            score.push(temp);
            
            let scores = score.iter().fold(1, |a, b| a*b);

            if scores > best_score { best_score = scores }
        }
    }
    println!("{}", best_score);
}

fn part1() {
    let grid: Vec<Vec<i32>> = include_str!("input.txt").lines().map(|x| x.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut res = (rows+cols)*2-4;

    for row_index in 1..rows-1 {
        for col_index in 1..cols-1 {
            
            let mut vis = true;
            for i in 0..col_index {
                if grid[row_index][i] >= grid[row_index][col_index] {
                    vis = false;
                }
            }
            let mut visible = vis;
            if !visible {
                let mut vis = true;
                for i in col_index+1..cols {
                    if grid[row_index][i] >= grid[row_index][col_index] {
                        vis = false;
                    }
                }
                visible = vis;
            }
            if !visible {
                let mut vis = true;
                for i in 0..row_index {
                    if grid[i][col_index] >= grid[row_index][col_index] {
                        vis = false;
                    }
                }
                visible = vis;
            }
            if !visible {
                let mut vis = true;
                for i in row_index+1..rows {
                    if grid[i][col_index] >= grid[row_index][col_index] {
                        vis = false;
                    }
                }
                visible = vis;
            }
            if visible { res += 1; }
        }
    }
    println!("{}", res);
} 