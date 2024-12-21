use std::cmp::Ordering;

fn get_antinode(f: &(i32, i32), s: &(i32, i32), width: usize, height: usize, scale: i32) -> Option<(i32, i32)> {
    let (row, col) = match (f.0.cmp(&s.0), f.1.cmp(&s.1)) {
        (Ordering::Greater, Ordering::Greater) => (f.0 - scale*(f.0 - s.0), f.1 - scale*(f.1 - s.1)),
        (Ordering::Greater, Ordering::Less) => (f.0 - scale*(f.0 - s.0), f.1 + scale*(s.1 - f.1)),
        (Ordering::Less, Ordering::Greater) => (f.0 + scale*(s.0 - f.0), f.1 - scale*(f.1 - s.1)),
        (Ordering::Less, Ordering::Less) => (f.0 + scale*(s.0 - f.0), f.1 + scale*(s.1 - f.1)),
        _ => unreachable!("not happened")
    };
    if col < 0 || row < 0 || col >= width as i32 || row >= height as i32 {
        return None;
    }

    Some((row, col))
}

fn part1(pos: &Vec<Vec<(i32, i32)>>, width: usize, height: usize, scale: i32) -> usize {
    let mut grid = vec![vec![false; width]; height];
    for p in pos {
        if p.is_empty() {
            continue;
        }

        for f in p {
            for s in p {
                if f == s {
                    continue;
                }

            
                for mul in 2..scale+1 {
                    if let Some((row, col)) = get_antinode(f, s, width, height, mul) {
                        grid[row as usize][col as usize] = true;
                    }
                }
            }

        }
    }

    grid.iter().map(|row| row.iter().filter(|&&col| col).count()).sum()
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut pos: Vec<Vec<(i32, i32)>> = Vec::with_capacity(1<<8);
    let mut row = 0;
    let mut width = 0;
    let mut n = 0;
    for _ in 0..1<<8 {
        pos.push(vec![]);
    }

    while let Some(Ok(line)) = lines.next() {
        width = line.len();
        for (i, c) in line.as_bytes().iter().enumerate() {
            if c == &b'.' {
                continue;
            }
            pos[*c as usize].push((row as i32, i as i32));
        }
        row += 1;
    }

    println!("ans1 = {:?}", part1(&pos, row, width, 2));
    println!("ans2 = {:?}", part1(&pos, row, width, 50));
}