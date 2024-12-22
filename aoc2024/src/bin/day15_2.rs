use std::collections::{HashSet, VecDeque};

fn get_dir(command: u8) -> (i32, i32) {
    match command {
        b'^' => (-1, 0),
        b'v' => (1, 0),
        b'<' => (0, -1),
        b'>' => (0, 1),
        _ => unreachable!()
    }
}

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
    pos: (usize, usize),
    dir: (i32, i32),
) -> Option<Vec<(usize, usize)>> {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut moved: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    q.push_back(pos);
    while let Some((mut row, mut col)) = q.pop_front() {
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));
        moved.push((row, col));
        row = (row as i32 + dir.0) as usize;
        col = (col as i32 + dir.1) as usize;
        if grid[row][col] == b'.' {
            continue;
        }
        if grid[row][col] == b'#' {
            return None;
        }

        if grid[row][col] == b'[' {
            q.push_back((row, col));
            q.push_back((row, col + 1));
            continue;
        }
        if grid[row][col] == b']' {
            q.push_back((row, col));
            q.push_back((row, col-1));
            continue;
        }
    }

    Some(moved)
}

fn move_robot(grid: &mut Vec<Vec<u8>>, mut to_move: Vec<(usize, usize)>, dir: (i32, i32)) {
    while let Some((row, col)) = to_move.pop() {
        let next = (
            (row as i32 + dir.0) as usize,
            (col as i32 + dir.1) as usize,
        );
        let temp = grid[row][col];
        grid[row][col] = grid[next.0][next.1];
        grid[next.0][next.1] = temp;
    }
}

fn part2(mut grid: Vec<Vec<u8>>, commands: &[u8]) -> usize {
    let mut pos = find_robot(&grid);

    for command in commands {
        let dir = get_dir(*command);
        if let Some(to_moves) = find_available_end(&grid, pos, dir) {
            move_robot(&mut grid, to_moves, dir);
            pos.0 = (pos.0 as i32 + dir.0) as usize;
            pos.1 = (pos.1 as i32 + dir.1) as usize;
        }
    }

    grid.iter()
    .enumerate()
    .flat_map(|row| {
        row.1.iter().enumerate().filter_map(move |col| {
            if col.1 == &b'[' {
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
            let line = line.iter().flat_map(|&b| {
                if b == b'#' || b == b'.' {
                    return vec![b, b];
                }
                if b == b'O' {
                    return vec![b'[', b']'];
                }
                vec![b'@', b'.']
            }).collect();
            grid.push(line);
        } else {
            commands.extend(line);
        }
    }
   

    println!("{}", part2(grid, &commands));
}
