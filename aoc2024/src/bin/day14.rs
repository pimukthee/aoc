use std::cmp::Ordering::*;
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
    robots
        .iter()
        .fold([0, 0, 0, 0], |mut acc, robot| {
            let x = (WIDTH + (robot.x + TIME * robot.vx) % WIDTH) % WIDTH;
            let y = (HEIGHT + (robot.y + TIME * robot.vy) % HEIGHT) % HEIGHT;

            match (x.cmp(&(WIDTH / 2)), y.cmp(&(HEIGHT / 2))) {
                (Less, Less) => acc[0] += 1,
                (Less, Greater) => acc[2] += 1,
                (Greater, Less) => acc[1] += 1,
                (Greater, Greater) => acc[3] += 1,
                _ => (),
            };
            acc
        })
        .iter()
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
