use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}, time::Instant};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn shortest_path(block: &[[bool; WIDTH]; HEIGHT]) -> usize {
    let mut q = BinaryHeap::new();
    let mut dist = [[usize::MAX; WIDTH]; HEIGHT];
    q.push(Reverse((0, (0, 0))));

    while let Some(Reverse((dis, (r, c)))) = q.pop() {
        if dist[r][c] <= dis {
            continue;
        }
        dist[r][c] = dis;

        for dir in DIRS {
            let row = r as isize + dir.0;
            let col = c as isize + dir.1;
            if row < 0 || row as usize >= HEIGHT || col < 0 || col as usize >= WIDTH {
                continue;
            }
            let row = row as usize;
            let col = col as usize;
            if block[row][col] {
                continue;
            }

            q.push(Reverse((dis + 1, (row, col))));
        }
    }

    dist[HEIGHT-1][WIDTH-1]
}

fn part1(corrupted: &[(usize, usize)]) -> usize {
    let block = corrupted[..1024].iter().fold([[false; WIDTH]; HEIGHT], |mut acc, pos| {
        acc[pos.0][pos.1] = true;
        acc
    });

    shortest_path(&block)
}

fn part2(corrupted: &[(usize, usize)]) -> (usize, usize) {
    let mut block = [[corrupted.len() - 1; WIDTH]; HEIGHT];
    for (i, &pos) in corrupted.iter().enumerate() {
        block[pos.0][pos.1] = i;
    }

    let mut dist = [[0; WIDTH]; HEIGHT];
    dist[0][0] = block[0][0];
    let mut q = BinaryHeap::new();
    q.push((dist[0][0], (0_usize, 0_usize)));
    let mut cnt = 0;

    while let Some((dis, pos)) = q.pop() {
        cnt += 1;
       for dir in DIRS {
            let row = pos.0 as isize + dir.0;
            let col = pos.1 as isize + dir.1;
            
            if row < 0 || row >= HEIGHT as isize || col < 0 || col >= WIDTH as isize {
                continue;
            }

            let row = row as usize;
            let col = col as usize;
            let x = dis.min(block[row][col]);
            if x > dist[row][col] {
                dist[row][col] = x;
                q.push((x, (row, col)));
            }
        }
    }

    println!("{}", cnt);

    corrupted[dist[HEIGHT-1][WIDTH-1]]
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut corrupted = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let (x, y) = aoc2024::scan!(&line, ',', usize, usize);
        corrupted.push((y, x));
    }

    let ans1 = part1(&corrupted);
    println!("{}", ans1);
    let ans2 = part2(&corrupted);
    println!("({},{})", ans2.1, ans2.0);
}