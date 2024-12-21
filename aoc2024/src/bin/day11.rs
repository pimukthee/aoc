use std::{collections::HashMap, time::Instant};

fn n_len(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n.ilog10() as usize + 1
}

fn split_num(n: usize, l: usize) -> (usize, usize) {
    let div = 10_u32.pow(l as u32/2) as usize;
    (n/div, n%div)
}

fn solve(arr: &[usize], n: usize) -> usize {
    let mut cur = arr.iter().fold(HashMap::<usize, usize>::new(), |mut m, x| {
        *m.entry(*x).or_insert(0) += 1;
        m
    });
    for _ in 0..n {
        cur = cur.into_iter().fold(HashMap::<usize, usize>::with_capacity(4000), |mut m, (num, cnt)| {
            if num == 0 {
                *m.entry(1).or_insert(0) += cnt;
                return m;
            }

            let l = n_len(num);
            if l % 2 == 0 {
                let (left, right) = split_num(num, l);
                *m.entry(left).or_insert(0) += cnt;
                *m.entry(right).or_insert(0) += cnt;
                return m;
            }

            *m.entry(num * 2024).or_insert(0) += cnt;

            m
        })
    }

    cur.into_iter().map(|(_, cnt)| cnt).sum()
}

fn main() {
    //println!("{}", 253000_i32.ilog10() + 1);
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let arr: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();

    // println!("ans1 {}", solve(&arr, 25));
    let now = Instant::now();
    println!("ans2 {}", solve(&arr, 75));
    println!("{:?}", now.elapsed());
}