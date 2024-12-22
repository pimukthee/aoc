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

fn print(pos: &Vec<Vec<usize>>) {
    for i in 0..HEIGHT as usize {
        for j in 0..WIDTH as usize {
            if pos[i][j] != 0 {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn check(pos: &Vec<Vec<usize>>) -> bool {
    for i in 0..(HEIGHT-3) as usize {
        for j in 0..(WIDTH-3) as usize {
            let mut cnt = 0;
            for k in 0..3 {
                for l in 0..3 {
                    if pos[i+k][j+l] != 0 {
                        cnt += 1
                    }
                }
            }
            if cnt >= 9 {
                return true;
            }
        }
    }
    false
}

/// there should be 3*3 block size fill with robot
fn part2(robots: &[Robot]) -> usize {
    let mut robots: Vec<Robot> = robots.to_vec();
    let mut pos: Vec<Vec<usize>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for robot in robots.iter() {
        pos[robot.y as usize][robot.x as usize] += 1;
    }
    for i in 0..WIDTH*HEIGHT {
        for robot in robots.iter_mut() {
            pos[robot.y as usize][robot.x as usize] -= 1;
            robot.x = (robot.x + robot.vx + WIDTH) % WIDTH;
            robot.y = (robot.y + robot.vy + HEIGHT) % HEIGHT;
            pos[robot.y as usize][robot.x as usize] += 1;
        }
        // can be improved. no need to loop all cells in the grid
        if check(&pos) {
            return i as usize + 1;
        }
    }
    unreachable!()
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
    println!("{}", part2(&robots));
}
