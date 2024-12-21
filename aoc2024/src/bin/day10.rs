use std::collections::VecDeque;

fn print(grid: &Vec<Vec<usize>>) {
    for row in grid {
        println!("{:?}", row);
    }
    println!();
}

fn count_reach(grid: &Vec<Vec<usize>>, pos: (usize, usize)) -> (usize, usize) {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 50]; 50];
    let mut dp: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    dp[pos.0][pos.1] = 1;

    let mut ans1 = 0;
    let mut ans2 = 0;
    q.push_back(pos);
    while let Some((row, col)) = q.pop_front() {
        if grid[row][col] == 9 {
            ans1 += !visited[row][col] as usize;
        }
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;
        let cur = grid[row][col];
        if row > 0 && cur + 1 == grid[row-1][col] {
            dp[row-1][col] += dp[row][col];
            q.push_back((row-1, col));
            
        }
        if row < grid.len()-1 && cur + 1 == grid[row+1][col] {
            dp[row+1][col] += dp[row][col];
            q.push_back((row+1, col));
        }
        if col > 0 && cur + 1 == grid[row][col - 1] {
            dp[row][col-1] += dp[row][col];
            q.push_back((row, col-1));
        }
        if col < grid[row].len() - 1 && cur + 1 == grid[row][col+1] {
            dp[row][col+1] += dp[row][col];
            q.push_back((row, col+1));
        }
    }

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 9 {
                ans2 += dp[i][j];
            }
        }
    }

    (ans1, ans2)
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut grid: Vec<Vec<usize>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let row = line.into_bytes().into_iter().map(|b| (b - b'0') as usize).collect(); 
        grid.push(row);
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    let height = grid.len();
    for row in 0..height {
        let width = grid[0].len();
        for col in 0..width {
            if grid[row][col] == 0 {
                let ans = count_reach(&grid, (row, col));
                
                ans1 += ans.0;
                ans2 += ans.1;
            }
        }
    }
    println!("ans1 {}", ans1);
    println!("ans2 {}", ans2);
}