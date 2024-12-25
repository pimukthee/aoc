use std::{cmp::Reverse, collections::{BinaryHeap, HashSet, VecDeque}};

const DIRS: [(i32, i32); 4]  = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DECISIONS: [(i32, usize); 3] = [(0, 1), (-1, 1000), (1, 1000)]; // straight, counter-clockwise, clockwise

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

fn solve(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    let src = find(grid, b'S');
    let dst = find(grid, b'E');
    let width = grid[0].len();
    let height = grid.len();
    let mut dist = vec![vec![vec![usize::MAX; 4]; width]; height];
    let mut parent: Vec<Vec<Vec<Vec<((usize, usize), usize)>>>> = Vec::new();
    for i in 0..grid.len() {
        parent.push(vec![vec![vec![]; 4]; grid[i].len()]);
    }
    dist[src.0][src.1][1] = 0;

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, (src, 1))));

    while let Some(Reverse((dis, (pos, dir)))) = q.pop() {
        dist[pos.0][pos.1][dir] = dis;
      
        for (next_dir, cost) in DECISIONS {
            let next_dir = ((dir as i32 + next_dir + 4) % 4) as usize;
            let new_cost = dis + cost;
            if next_dir != dir {
                let w = &mut dist[pos.0][pos.1][next_dir];
                if *w > new_cost {
                    parent[pos.0][pos.1][next_dir] = vec![((pos.0, pos.1), dir)];
                    q.push(Reverse((new_cost, ((pos.0, pos.1), next_dir))));
                    *w = new_cost;
                } else if *w == new_cost {
                    parent[pos.0][pos.1][next_dir].push(((pos.0, pos.1), dir));
                }
                continue;
            }
            let row = (pos.0 as i32 + DIRS[dir].0) as usize;
            let col = (pos.1 as i32 + DIRS[dir].1) as usize;
            if (grid[row][col] == b'.' || grid[row][col] == b'E') && dist[row][col][dir] >= new_cost {
                if dist[row][col][dir] > new_cost {
                    parent[row][col][dir] = vec![((pos.0, pos.1), dir)]
                } else {
                    parent[row][col][dir].push(((pos.0, pos.1), dir));
                }
                q.push(Reverse((new_cost, ((row, col), next_dir))));
            }
        }
    }

    let min_cost = dist[dst.0][dst.1].iter().min().cloned().unwrap();
    let mut q = VecDeque::new();
    for i in 0..4 {
        if dist[dst.0][dst.1][i] == min_cost {
            q.push_back(((dst.0, dst.1), i));
        }
    }

    let mut all_nodes = HashSet::new();
    while let Some((pos, dir)) = q.pop_front() {
        all_nodes.insert(pos);
        for child in parent[pos.0][pos.1][dir].iter() {
            q.push_back(((child.0.0, child.0.1), child.1));
        }
    }

    (min_cost, all_nodes.len())
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut grid: Vec<Vec<u8>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        grid.push(line.into_bytes());
    }

    let (ans1, ans2) = solve(&grid);
    println!("{}\n{}", ans1, ans2);
}
