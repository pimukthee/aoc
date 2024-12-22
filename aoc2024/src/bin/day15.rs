use core::str;
use std::mem;

const DIRS: [(u8, (i32, i32)); 4] = [
    (b'^', (-1, 0)),
    (b'v', (1, 0)),
    (b'<', (0, -1)),
    (b'>', (0, 1)),
];

fn find_robot(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == b'@' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn find_available_end(
    grid: &Vec<Vec<u8>>,
    (mut row, mut col): (usize, usize),
    dir: (i32, i32),
) -> Option<(usize, usize)> {
    loop {
        if grid[row][col] == b'.' {
            return Some((row, col));
        }
        if grid[row][col] == b'#' {
            return None;
        }

        row = (row as i32 + dir.0) as usize;
        col = (col as i32 + dir.1) as usize;
    }
}

fn swap_to(grid: &mut Vec<Vec<u8>>, mut src: (usize, usize), dst: (usize, usize), dir: (i32, i32)) {
    while src != dst {
        let next = (
            (src.0 as i32 + dir.0) as usize,
            (src.1 as i32 + dir.1) as usize,
        );
        let temp = grid[src.0][src.1];
        grid[src.0][src.1] = grid[next.0][next.1];
        grid[next.0][next.1] = temp;
        src.0 = next.0;
        src.1 = next.1;
    }
}

fn part1(grid: &mut Vec<Vec<u8>>, commands: &Vec<u8>) -> usize {
    let mut pos = find_robot(grid);

    for command in commands {
        let dir = DIRS.iter().find(|dir| &dir.0 == command).unwrap().1;
        if let Some((row, col)) = find_available_end(grid, pos, dir) {
            swap_to(grid, (row, col), pos, (dir.0 * -1, dir.1 * -1));
            pos.0 = (pos.0 as i32 + dir.0) as usize;
            pos.1 = (pos.1 as i32 + dir.1) as usize;
        }
    }

    grid.iter()
        .enumerate()
        .flat_map(|row| {
            row.1.iter().enumerate().filter_map(move |col| {
                if col.1 == &b'O' {
                    Some(100 * row.0 + col.0)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut is_command = false;
    let mut commands: Vec<u8> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            is_command = true;
            continue;
        }
        let line = line.into_bytes();
        if !is_command {
            grid.push(line);
        } else {
            commands.extend(line);
        }
    }
    println!("{}", part1(&mut grid, &commands));
}
