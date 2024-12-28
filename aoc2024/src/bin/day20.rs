use std::{
    cmp::Reverse,
    collections::BinaryHeap, time::Instant,
};

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn find(grid: &Vec<Vec<u8>>, target: u8) -> (usize, usize) {
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

fn shortest_path_from(grid: &Vec<Vec<u8>>, src: (usize, usize)) -> Vec<Vec<usize>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut dist = vec![vec![usize::MAX; width]; height];

    let mut q = BinaryHeap::new();
    q.push(Reverse((0_usize, src)));

    while let Some(Reverse((dis, pos))) = q.pop() {
        if dist[pos.0][pos.1] <= dis {
            continue;
        }

        dist[pos.0][pos.1] = dis;
        for dir in DIRS {
            let row = (pos.0 as isize + dir.0) as usize;
            let col = (pos.1 as isize + dir.1) as usize;

            if grid[row][col] == b'.' || grid[row][col] == b'S' || grid[row][col] == b'E' {
                q.push(Reverse((dis + 1, (row, col))));
            }
        }
    }
  
    dist
}

fn part1(grid: &Vec<Vec<u8>>, cheat: isize) -> usize {
    let src = find(grid, b'S');
    let dst = find(grid, b'E');
    let height = grid.len();
    let width = grid[0].len();
    let dist_src = shortest_path_from(grid, src);
    let dist_dst = shortest_path_from(grid, dst);
    let orig_cost = dist_src[dst.0][dst.1];
    let mut ans = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if dist_src[i][j] == usize::MAX {
                continue;
            }
            for k in -cheat..cheat+1 {
                for l in -cheat..cheat+1 {
                    let k_abs = k.abs() as usize;
                    let l_abs = l.abs() as usize;
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
                
                    let new_cost = dist_src[i][j] + k_abs + l_abs + dist_dst[row as usize][col as usize];
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
    let now = Instant::now();
    let ans2 = part1(&grid, 20);
    println!("{:?}", now.elapsed());
    println!("ans1 {}\nans2 {}", ans1, ans2);
}
