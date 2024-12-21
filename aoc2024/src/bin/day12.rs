use std::collections::VecDeque;

fn bfs(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, (i, j): (usize, usize)) -> (usize, usize) {
    let mut q = VecDeque::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut area = 0;
    let mut perimeter = 0;

    q.push_back((i, j));
    while let Some((i, j)) = q.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;
        if i == 0 || grid[i-1][j] != grid[i][j] {
            perimeter += 1;
        } else {
            q.push_back((i-1, j));
        }
        if j == 0 || grid[i][j-1] != grid[i][j] {
            perimeter += 1;
        } else {
            q.push_back((i, j-1));
        }
        if i == rows-1 || grid[i+1][j] != grid[i][j] {
            perimeter += 1;
        } else {
            q.push_back((i+1, j));
        }
        if j == cols-1 || grid[i][j+1] != grid[i][j] {
            perimeter += 1;
        } else {
            q.push_back((i, j+1));
        }
        
        area += 1;
    }

    (area, perimeter)
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
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            if visited[i][j] {
                continue;
            }

            let (area, perimeter) = bfs(&grid, &mut visited, (i, j));
            ans1 += area * perimeter;
        }
    }

    println!("ans1 {}", ans1);
}