#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;
const TIME: i32 = 100;

fn part1(robots: &[Robot]) -> usize {
    let robots: Vec<Robot> = robots
        .iter()
        .map(|robot| Robot {
            x: (WIDTH + (robot.x + TIME * robot.vx) % WIDTH) % WIDTH,
            y: (HEIGHT + (robot.y + TIME * robot.vy) % HEIGHT) % HEIGHT,
            vx: robot.vx,
            vy: robot.vy,
        })
        .collect();

    [
        ((0, WIDTH / 2 - 1), (0, HEIGHT / 2 - 1)),
        ((WIDTH / 2 + 1, WIDTH - 1), (0, HEIGHT / 2 - 1)),
        ((0, WIDTH / 2 - 1), (HEIGHT / 2 + 1, HEIGHT - 1)),
        ((WIDTH / 2 + 1, WIDTH - 1), (HEIGHT / 2 + 1, HEIGHT - 1)),
    ]
    .into_iter()
    .map(|(x, y)| {
        robots
            .iter()
            .filter(|robot| robot.x >= x.0 && robot.x <= x.1 && robot.y >= y.0 && robot.y <= y.1)
            .count()
    })
    .product()
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut robots: Vec<Robot> = vec![];
    while let Some(Ok(line)) = lines.next() {
        let (pos, vel) = line.split_once(' ').unwrap();
        let (x, y) = aoc2024::scan!(pos.split_once('=').unwrap().1, ",", i32, i32);
        let (vx, vy) = aoc2024::scan!(vel.split_once('=').unwrap().1, ",", i32, i32);
        robots.push(Robot { x, y, vx, vy });
    }

    println!("{:?}", part1(&robots));
}
