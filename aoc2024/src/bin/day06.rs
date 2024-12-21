use std::collections::HashSet;

// up right down left
const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const MAX: usize = 135 * 135;

fn traverse(graph: &[Vec<u8>], (row, col): (i32, i32), dir: usize) -> (usize, bool) {
    let (mut row, mut col, mut dir) = (row, col, dir);
    let mut visited_with_dir: Vec<bool> = vec![false; 135 * 135 * 4];
    let mut visited: Vec<bool> = vec![false; 135 * 135];
    let mut step = 0;

    loop {
        // println!("{:?}", (row, col));
        let urow = row as usize;
        let ucol = col as usize;
        let dir_hash = (urow * 135 + ucol) * 4 + dir;
        let hash = urow * 135 + ucol;
        if visited_with_dir[dir_hash] {
            return (step, false);
        }

        visited_with_dir[dir_hash] = true;
        if !visited[hash] {
            step += 1;
        }
        visited[hash] = true;

        let (new_row, new_col) = (row + DIR[dir].0, col + DIR[dir].1);
        if new_row < 0 || new_col < 0 {
            return (step, true);
        }
        let (new_row, new_col) = (new_row as usize, new_col as usize);
        if new_row >= graph.len() || new_col >= graph[new_row].len() {
            return (step, true);
        }

        if graph[new_row][new_col] != b'#' {
            row = new_row as i32;
            col = new_col as i32;
        } else {
            dir = turn_right(dir);
        }
    }
}

fn turn_right(dir: usize) -> usize {
    (dir + 1) % 4
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut graph: Vec<Vec<u8>> = Vec::new();
    let mut guard_pos: (i32, i32) = (0, 0);
    while let Some(Ok(line)) = lines.next() {
        if let Some(col) = line.find("^") {
            guard_pos = (graph.len() as i32, col as i32);
        }
        graph.push(line.into_bytes());
    }

    let mut ans2 = 0;
    let (ans1, _) = traverse(&graph, guard_pos, 0);
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if graph[i][j] != b'.' {
                continue;
            }

            graph[i][j] = b'#';
            let (_, can_exit) = traverse(&graph, guard_pos, 0);
            ans2 += if can_exit { 0 } else { 1 };
            graph[i][j] = b'.'
        }
    }
    println!("ans1 = {}", ans1-1);
    println!("ans22 = {}", ans2);
}
