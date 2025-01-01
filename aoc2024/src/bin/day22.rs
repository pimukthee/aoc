use std::collections::{HashMap, HashSet, VecDeque};

const STEP: usize = 2000;
const MOD: isize = 16777216;

fn calculate_secret(nums: &[isize]) -> (isize, isize) {
    let mut ans1 = 0;
    let mut mem: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for &num in nums {
        let mut num = num;
        let mut changes = VecDeque::with_capacity(5);
        let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for _ in 0..STEP {
            let cur = num;
            num = ((num * 64) % MOD) ^ num;
            num = ((num / 32) % MOD) ^ num;
            num = ((num * 2048) % MOD) ^ num;
            changes.push_back(num%10 - cur%10);

            if changes.len() < 4 {
                continue;
            }
            if changes.len() == 5 {
                changes.pop_front();
            }
            
            let key = (changes[0], changes[1], changes[2], changes[3]);
            if !visited.contains(&key) {
                *mem.entry(key).or_insert(0) += num%10;
                visited.insert(key);
            }
        }
        ans1 += num;
    }

    let m = mem.into_iter().max_by_key(|(_, v)| *v).unwrap();
    println!("{:?}", m.0);
    let ans2 = m.1;

    (ans1, ans2)
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut nums = vec![];
    while let Some(Ok(line)) = lines.next() {
        let num = line.parse::<isize>().unwrap();
        nums.push(num);
    }

    let (ans1, ans2) = calculate_secret(&nums);

    println!("ans1 {}\nans2 {}", ans1, ans2);
}