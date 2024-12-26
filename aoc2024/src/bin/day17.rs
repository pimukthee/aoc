use std::collections::VecDeque;

fn get_combo(n: usize, a: usize, b: usize, c: usize) -> usize {
    match n {
        0..4 => n,
        4 => a,
        5 => b,
        6 => c,
        _ => 0,
    }
}

fn solve(a: usize) -> usize {
   let mut b = a & 0b111;
    let c = a >> (b^7);
    b ^= c;
    
    b & 0b111
}

fn part2(instructions: &[usize]) -> usize {
    let mut q = VecDeque::new();
    for i in 0..8 {
        q.push_back((i, instructions.len() - 1));
    }

    while let Some((a, idx)) = q.pop_front() {
        if solve(a) != instructions[idx] {
            continue;
        }
        if idx == 0 {
            return a
        }
    
        for i in 0..8 {
            q.push_back(((a<<3) + i, idx - 1));
        }
    }

    unreachable!()
}

fn part1(instructions: &[usize], mut a: usize, mut b: usize, mut c: usize) -> String {
    let mut ip = 0;
    let mut output: Vec<String> = Vec::new();
    loop {
        if ip + 1 >= instructions.len() {
            break;
        }
        let instruction = instructions[ip];
        let operand = instructions[ip + 1];
        let combo = get_combo(operand, a, b, c);
        match instruction {
            0 => a >>= combo,
            1 => b ^= operand,
            2 => b = combo & 0b111,
            3 => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            },
            4 => b ^= c,
            5 => output.push((combo & 0b111).to_string()),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => unreachable!(),
        }
        ip += 2;
    }

    output.join(",")
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let a: usize = lines.next().unwrap().unwrap().trim_start_matches("Register A: ").parse().unwrap();
    let b: usize = lines.next().unwrap().unwrap().trim_start_matches("Register B: ").parse().unwrap();
    let c: usize = lines.next().unwrap().unwrap().trim_start_matches("Register C: ").parse().unwrap();
    lines.next();
    let instructions: Vec<usize> = lines.next().unwrap().unwrap().trim_start_matches("Program: ").split(',').map(|opcode| opcode.parse().unwrap()).collect();

    println!("{}", part1(&instructions, a, b, c));
    let ans2 = part2(&instructions);
    println!("{}",ans2);
    println!("{}", part1(&instructions, ans2, b, c));
}