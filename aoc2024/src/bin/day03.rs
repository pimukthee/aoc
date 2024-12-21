use std::io;

fn sum_mul(s: &str) -> i32 {
    s.split("mul(")
        .map(|x| x.split(")").next())
        .filter(|x| x.is_some())
        .map(|x| {
            match x.unwrap().split_once(",") {
                Some((a, b)) => a.parse::<i32>().unwrap_or(0) * b.parse::<i32>().unwrap_or(0),
                _ => 0
            }
        })
        .sum()
}

fn part2(s: &str) -> i32 {
    s.split("do()").map(|ss| {
        let candidate = ss.split("don't()").next();
        sum_mul(candidate.unwrap())
    }).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut s = String::new();
    let mut ans1 = 0;
    let mut ans2 = 0;

    while let Some(Ok(line)) = lines.next() {
        s += &line;
    }
    ans1 += sum_mul(&s);
    ans2 += part2(&s);
        
    println!("ans1 = {}", ans1);
    println!("ans2 = {}", ans2);
}