use std::{collections::HashMap, io, time::Instant};

fn part_one(x: &[i32], y: &[i32]) -> i32 {
    x.iter()
        .zip(y.iter())
        .fold(0, |acc, p| acc + i32::abs(p.0 - p.1))
}

fn part_two(x: &[i32], y: &[i32]) -> i32 {
    x.iter().fold(0, |acc, &b| {
        let lower_bound = y.partition_point(|&yy| yy < b);
        let upper_bound = y.partition_point(|&yy| yy <= b);
        acc + (b*(upper_bound as i32 - lower_bound as i32))
    })
}

fn part_two_hash(x: &[i32], y: &[i32]) -> i32 {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for a in y {
        *hash.entry(*a).or_insert(0) += 1;
    }

    x.iter().fold(0, |acc, &b| {
        acc + b*hash.get(&b).unwrap_or(&0)
    })
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let (mut x, mut y) = (vec![], vec![]);

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = aoc2024::scan!(line, "   ", i32, i32);
        x.push(a);
        y.push(b);
    }
    x.sort_unstable();
    y.sort_unstable();

    println!("part_one = {}", part_one(&x, &y));
    let now = Instant::now();
    println!("part_two = {}", part_two(&x, &y));
    println!("elapsed = {:?}", now.elapsed());

    let now2 = Instant::now();
    println!("part_two_hash = {}", part_two_hash(&x, &y));
    println!("elapsed = {:?}", now2.elapsed());
}
