use std::collections::{HashMap, VecDeque};

const WIDTH: usize = 5;
const HEIGHT: usize = 6;
const DEPTH: usize = 25;

const DIRS: [((isize, isize), u8); 4] = [
    ((-1, 0), b'^'),
    ((0, 1), b'>'),
    ((1, 0), b'v'),
    ((0, -1), b'<'),
];
const NUMPAD: [[u8; WIDTH]; HEIGHT] = [
    [b'#', b'#', b'#', b'#', b'#'],
    [b'#', b'7', b'8', b'9', b'#'],
    [b'#', b'4', b'5', b'6', b'#'],
    [b'#', b'1', b'2', b'3', b'#'],
    [b'#', b'#', b'0', b'A', b'#'],
    [b'#', b'#', b'#', b'#', b'#'],
];
const DIRECTIONAL: [[u8; WIDTH]; HEIGHT] = [
    [b'#', b'#', b'#', b'#', b'#'],
    [b'#', b'#', b'#', b'#', b'#'],
    [b'#', b'#', b'#', b'#', b'#'],
    [b'#', b'#', b'^', b'A', b'#'],
    [b'#', b'<', b'v', b'>', b'#'],
    [b'#', b'#', b'#', b'#', b'#'],
];

fn get_pos(pad: &[[u8; 5]; 6], target: u8) -> (usize, usize) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if pad[i][j] == target {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn shortest_path(pad: &[[u8; 5]; 6], src: u8, dst: u8) -> Vec<Vec<u8>> {
    let src_pos = get_pos(pad, src);
    let dst_pos = get_pos(pad, dst);
    let mut dist = [[usize::MAX; WIDTH]; HEIGHT];
    dist[src_pos.0][src_pos.1] = 0;
    let mut q = VecDeque::new();
    q.push_back((0, (src_pos, 0)));

    while let Some((dis, (pos, dir_idx))) = q.pop_front() {
        for dir in DIRS {
            let row = (pos.0 as isize + dir.0 .0) as usize;
            let col = (pos.1 as isize + dir.0 .1) as usize;
            if pad[row][col] != b'#' && dist[row][col] > dis + 1 {
                q.push_back((dis + 1, ((row, col), dir_idx)));
                dist[row][col] = dis + 1;
            }
        }
    }

    let mut q = VecDeque::new();
    let mut all_paths = vec![];
    q.push_back(((dst_pos.0, dst_pos.1), vec![b'A']));
    while let Some((pos, path)) = q.pop_front() {
        if pos == src_pos {
            all_paths.push(path);
            continue;
        }
        for (i, dir) in DIRS.iter().enumerate() {
            let row = (pos.0 as isize + dir.0 .0) as usize;
            let col = (pos.1 as isize + dir.0 .1) as usize;
            if dist[row][col] == dist[pos.0][pos.1] - 1 {
                let path: Vec<u8> = std::iter::once(DIRS[(i + 2) % 4].1)
                    .chain(path.iter().copied())
                    .collect();
                q.push_back(((row, col), path));
            }
        }
    }

    all_paths
}

fn solve(sequence: &[u8], dp: &mut HashMap<(Vec<u8>, usize), usize>, depth: usize) -> usize {
    let mut cur = b'A';
    let mut ans = 0;
    let hash = (sequence.to_vec(), depth);

    if let Some(cnt) = dp.get(&hash) {
        return *cnt;
    }

    for c in sequence {
        let pad = if depth == DEPTH { NUMPAD } else { DIRECTIONAL };
        let paths = shortest_path(&pad, cur, *c);
        if depth == 0 {
            ans += paths[0].len();
        } else {
            ans += paths
                .iter()
                .map(|path| solve(path, dp, depth - 1))
                .min()
                .unwrap();
        }
        cur = *c;
    }
    dp.insert(hash, ans);
    ans
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let mut ans = 0;
    let mut dp: HashMap<(Vec<u8>, usize), usize> = HashMap::new();
    // let paths = shortest_path(&DIRECTIONAL, &mut dp, 0, b'A', b'<');
    // for path in paths.as_ref() {
    //     println!("{}", std::str::from_utf8(&path).unwrap());
    // }
    // println!("{}", k);

    while let Some(Ok(line)) = lines.next() {
        let num = line.split_once('A').unwrap().0.parse::<usize>().unwrap();
        let line = line.into_bytes();

        let k = solve(&line, &mut dp, DEPTH);
        ans += num * k;
    }

    println!("{}", ans);
}
