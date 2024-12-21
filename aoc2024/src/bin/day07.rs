fn part1(ans: u64, cur: u64, nums: &[u64]) -> bool {
    if cur > ans {
        return false;
    }

    let add = cur + nums[0];
    let mul = cur * nums[0];

    if nums.len() == 1 {
        return add == ans || mul == ans;
    }

    part1(ans, add, &nums[1..]) || part1(ans, mul, &nums[1..])
}

fn part2(ans: u64, cur: u64, nums: &[u64]) -> bool {
    if cur > ans {
        return false;
    }

    let l = nums[0].checked_ilog10().unwrap_or(0) + 1;
    let concat = cur * 10u64.pow(l) + nums[0];
    let add = cur + nums[0];
    let mul = cur * nums[0];

    if nums.len() == 1 {
        return add == ans || mul == ans || concat == ans;
    }

    part2(ans, add, &nums[1..]) || part2(ans, mul, &nums[1..]) || part2(ans, concat, &nums[1..])
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut ans1 = 0;
    let mut ans2 = 0;
    
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (ans, nums) = line.split_once(":").unwrap();
        let ans = ans.parse::<u64>().unwrap();
        let nums: Vec<u64> = nums.split(" ").filter_map(|n| n.parse::<u64>().ok()).collect();

        ans1 += if part1(ans, nums[0], &nums[1..]) { ans } else { 0 };
        ans2 += if part2(ans, nums[0], &nums[1..]) { ans } else { 0 };
    }

    println!("ans1 = {}", ans1);
    println!("ans2 = {}", ans2);
}