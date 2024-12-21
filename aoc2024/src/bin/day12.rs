use std::{collections::VecDeque, time::Instant};

fn bfs(
    grid: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    (i, j): (usize, usize),
) -> (usize, usize, usize) {
    let mut q = VecDeque::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut area = 0;
    let mut perimeter = 0;
    let mut corner = 0;

    q.push_back((i, j));
    while let Some((i, j)) = q.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;

        let up = i == 0 || grid[i - 1][j] != grid[i][j];
        let up_right = i == 0 || j == cols - 1 || grid[i - 1][j + 1] != grid[i][j];
        let up_left = i == 0 || j == 0 || grid[i - 1][j - 1] != grid[i][j];
        let left = j == 0 || grid[i][j - 1] != grid[i][j];
        let down = i == rows - 1 || grid[i + 1][j] != grid[i][j];
        let down_left = i == rows - 1 || j == 0 || grid[i + 1][j - 1] != grid[i][j];
        let down_right = i == rows - 1 || j == cols - 1 || grid[i + 1][j + 1] != grid[i][j];
        let right = j == cols - 1 || grid[i][j + 1] != grid[i][j];

        if up {
            perimeter += 1;
            corner += left as usize + right as usize;
        } else {
            q.push_back((i - 1, j));
        }

        if left {
            perimeter += 1;
        } else {
            corner += (!up && up_left) as usize + (!down && down_left) as usize;
            q.push_back((i, j - 1));
        }

        if down {
            perimeter += 1;
            corner += left as usize + right as usize;
        } else {
            q.push_back((i + 1, j));
        }

        if right {
            perimeter += 1;
        } else {
            corner += (!up && up_right) as usize + (!down && down_right) as usize;
            q.push_back((i, j + 1));
        }

        area += 1;
    }

    (area, perimeter, corner)
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut grid: Vec<Vec<u8>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        grid.push(line.into_bytes());
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let now = Instant::now();
    for i in 0..rows {
        for j in 0..cols {
            if visited[i][j] {
                continue;
            }

            let (area, perimeter, corner) = bfs(&grid, &mut visited, (i, j));
            ans1 += area * perimeter;
            ans2 += area * corner;
        }
    }

    println!("ans1 {}", ans1);
    println!("ans2 {}", ans2);
    println!("{:?}", now.elapsed());
}

