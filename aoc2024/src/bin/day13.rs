fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

fn parse_step(line: &str) -> (i64, i64) {
    let (left, right) = line.split_once(',').unwrap();
    let x = left.split_once('+').unwrap().1.parse::<i64>().unwrap();
    let y = right.split_once('+').unwrap().1.parse::<i64>().unwrap();

    (x, y)
}

fn parse_result(line: &str) -> (i64, i64) {
    let (left, right) = line.split_once(',').unwrap();
    let x = left.split_once('=').unwrap().1.parse::<i64>().unwrap();
    let y = right.split_once('=').unwrap().1.parse::<i64>().unwrap();

    (x, y)
}

fn solve(mut x: [i64; 3], mut y: [i64; 3]) -> (i64, i64) {
    let lcm = x[0] * y[0] / gcd(x[0], y[0]);
    x[1] = x[1].checked_mul(lcm / x[0]).unwrap();
    x[2] = x[2].checked_mul(lcm / x[0]).unwrap();
    x[0] = lcm;

    y[1] = y[1].checked_mul(lcm / y[0]).unwrap();
    y[2] = y[2].checked_mul(lcm / y[0]).unwrap();
    y[0] = lcm;

    let b = (x[2] - y[2]) / (x[1] - y[1]);
    let a = (x[2] - x[1] * b) / x[0];
    if (a * x[0] + x[1] * b, a * y[0] + y[1] * b) != (x[2], y[2]) {
        return (0, 0);
    }
    (a, b)
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let mut cnt = 0;
    let mut x: [i64; 3] = [0, 0, 0];
    let mut y: [i64; 3] = [0, 0, 0];
    let mut ans1 = 0;
    let mut ans2 = 0;
    while let Some(Ok(line)) = lines.next() {
        if cnt == 0 || cnt == 1 {
            let (l, r) = parse_step(&line);
            x[cnt] = l;
            y[cnt] = r;
        }
        if cnt == 2 {
            let (l, r) = parse_result(&line);
            x[2] = l;
            y[2] = r;
            let (a, b) = solve(x, y);
            ans1 += a * 3 + b;
            x[2] += 10000000000000;
            y[2] += 10000000000000;
            let (a, b) = solve(x, y);
            // println!("a = {}, b = {}", a, b);
            ans2 += a * 3 + b;
        }
        cnt += 1;
        cnt %= 4;
    }

    println!("ans1 {}", ans1);
    println!("ans2 {}", ans2);
}
