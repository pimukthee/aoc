use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque}, time::Instant,
};

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn find(grid: &[Vec<u8>], target: u8) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, col)| col == &&target)
                .map(|(j, _)| (i, j))
        })
        .unwrap()
}

fn shortest_path_from(grid: &[Vec<u8>], src: (usize, usize)) -> Vec<Vec<usize>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut dist = vec![vec![usize::MAX; width]; height];

    let mut q = VecDeque::new();
    q.push_back(src);

    while let Some(pos) = q.pop_front() {
        for dir in DIRS {
            let row = (pos.0 as isize + dir.0) as usize;
            let col = (pos.1 as isize + dir.1) as usize;

            if dist[row][col] == usize::MAX && (grid[row][col] == b'.' || grid[row][col] == b'S' || grid[row][col] == b'E') {
                dist[row][col] = dist[pos.0][pos.1] + 1;
                q.push_back((row, col));
            }
        }
    }
  
    dist
}

fn part1(grid: &[Vec<u8>], cheat: isize) -> usize {
    let src = find(grid, b'S');
    let dst = find(grid, b'E');
    let height = grid.len();
    let width = grid[0].len();
    let dist_dst = shortest_path_from(grid, dst);
    let orig_cost = dist_dst[src.0][src.1];
    let mut ans = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if dist_dst[i][j] == usize::MAX {
                continue;
            }

            for k in -cheat..cheat+1 {
                let max_l = cheat - k.abs();
                let k_abs = k.unsigned_abs();
                for l in -max_l..max_l+1 {
                    let l_abs = l.unsigned_abs();
                    if k_abs + l_abs > cheat as usize {
                        continue;
                    }
                    let row = i as isize + k;
                    let col = j as isize + l;
                    if row <= 0 || row >= height as isize-1 || col <= 0 || col >= width as isize - 1 {
                        continue;
                    }
                    if dist_dst[row as usize][col as usize] == usize::MAX {
                        continue;
                    }
                
                    let new_cost = (orig_cost - dist_dst[i][j]) + k_abs + l_abs + dist_dst[row as usize][col as usize];
                    if new_cost >= orig_cost {
                        continue;
                    }
                    if orig_cost - new_cost >= 100 {
                        ans += 1;
                    }
                }
            }
        }
    }
    
    ans
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut grid: Vec<Vec<u8>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        grid.push(line.into_bytes());
    }

    let ans1 = part1(&grid, 2);
    let ans2 = part1(&grid, 20);
    println!("ans1 {}\nans2 {}", ans1, ans2);
}
