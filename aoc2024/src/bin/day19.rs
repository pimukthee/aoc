use std::{collections::{HashMap, HashSet}, time::Instant};

fn count_way<'a, 'b>(stripes: &HashSet<Vec<u8>>, target: &'a [u8], mem: &'b mut HashMap<&'a [u8], usize>) -> usize {
    if let Some(&cnt) = mem.get(target) {
        return cnt
    }
    
    if target.len() == 0 {
        return 1;
    }
    
    let mut cnt = 0;
    for i in 1..target.len()+1 {
        if !stripes.contains(&target[0..i]) {
            continue;
        }
        
        cnt += count_way(stripes, &target[i..], mem);
    }

    mem.insert(&target[..], cnt);
    cnt
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let stripes = lines.next().unwrap().unwrap();
    let stripes = stripes
        .split(',')
        .map(|stripe| stripe.trim().to_owned().into_bytes())
        .collect::<HashSet<Vec<u8>>>();

    let _ = lines.next();

    let mut targets: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        targets.push(line.into_bytes());
    }

    let mut mem = HashMap::new();
    let (ans2, ans1) = targets.iter().map(|target|  count_way(&stripes, target, &mut mem)).fold((0, 0), |acc, cnt| {
        if cnt == 0 {
            return acc;
        }

        (acc.0 + cnt, acc.1 + 1)
    });

    println!("ans1 {}\nans2 {}", ans1, ans2);

}
