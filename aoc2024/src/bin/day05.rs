use std::{cmp::Ordering, collections::HashSet, io};

fn main() {
    let mut graph: Vec<HashSet<usize>> = Vec::new();
    graph.resize(100, HashSet::new());

    let mut lines = io::stdin().lines();
    let mut parse_order = true;
    let mut ans1 = 0;
    let mut ans2 = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            parse_order = false;
            continue;
        }
        if parse_order {
            let u = line[..2].parse::<usize>().unwrap();
            let v = line[3..].parse::<usize>().unwrap();
            graph[u].insert(v);
            continue;
        }
        
        let updates: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut correct_updates: Vec<usize> = updates.clone();
        correct_updates.sort_unstable_by(|&a, &b| {
            match (graph[a].contains(&b), graph[b].contains(&a)) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });

        let n = updates.len();
        if updates == correct_updates {
            ans1 += updates[n/2];
        } else {
            ans2 += correct_updates[n/2];
        }
    }
    
    println!("ans1 = {}", ans1);
    println!("ans2 = {}", ans2);
}