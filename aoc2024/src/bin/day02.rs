use std::io;

fn is_report_safe(levels: &[i32]) -> bool {
    let first_diff = levels[1] - levels[0];
    levels.windows(2).all(|levels| is_safe(first_diff, levels[1] - levels[0]))
}

fn is_safe(prev: i32, cur: i32) -> bool {
    prev.signum() == cur.signum() && matches!(cur.abs(), 1..=3)
}

fn part2(levels: &[i32]) -> bool {
    let n = levels.len();
    (2..n).all(|i| {
        if is_safe(levels[i] - levels[i-1], levels[i-1] - levels[i-2]) {
            return true
        }

        [i-2, i-1, i].iter().any(|&idx| {
            let report = levels.iter().enumerate().filter(|(i, _)| *i != idx).map(|(_, level)| *level).collect::<Vec<i32>>();
            is_report_safe(&report)
        })
    })
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut ans1 = 0;
    let mut ans2 = 0;
    while let Some(Ok(line)) = lines.next() {
        let levels: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        ans1 += if is_report_safe(&levels) { 1 } else { 0 };
        ans2 += if part2(&levels) { 1 } else { 0 };
        //let ilevels: Vec<i32> = levels.iter().map(|&l| l as i32).collect();
       // solve2(
    }

    println!("part1 {}", ans1);
    println!("part2 {}", ans2);
    //println!("{:?}", solve(&reports));
}
