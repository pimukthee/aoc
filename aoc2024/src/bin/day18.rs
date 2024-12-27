use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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
            let row = r as i32 + dir.0;
            let col = c as i32 + dir.1;
            if row < 0 || row as usize >= HEIGHT || col < 0 || col as usize >= WIDTH {
                continue;
            }
            let row = row as usize;
            let col = col as usize;
            if block[row][col] {
                continue;
            }

            q.push(Reverse((dis + 1, (row as usize, col as usize))));
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
    let mut block = [[false; WIDTH]; HEIGHT];
    for (i, &pos) in corrupted.iter().enumerate() {
        block[pos.0][pos.1] = true;
        if i >= 1024 {
            if shortest_path(&block) == usize::MAX {
                return pos;
            }
        }
    }

    unreachable!()
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