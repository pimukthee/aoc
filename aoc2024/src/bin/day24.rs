use std::{
    collections::{HashMap, HashSet},
    ops::{BitAnd, BitOr, BitXor},
};

const MAX_BIT: &[u8] = b"45";

fn part1(wires: &[String], commands: &[String]) -> usize {
    let mut wires = wires
        .iter()
        .map(|line| {
            let (wire, val) = line.split_once(':').unwrap();
            let val = if val == " 0" { false } else { true };
            (wire, val)
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..commands.len() {
        for command in commands {
            let c = command.split(' ').collect::<Vec<_>>();
            let op: fn(bool, bool) -> bool = match c[1] {
                "XOR" => bool::bitxor,
                "AND" => bool::bitand,
                "OR" => bool::bitor,
                _ => unreachable!(),
            };
            match (wires.get(&c[0]), wires.get(&c[2]), wires.get(&c[4])) {
                (Some(&x), Some(&y), None) => wires.insert(c[4], op(x, y)),
                _ => None,
            };
        }
    }

    (0..100)
        .filter_map(|i| {
            let key = format!("z{:02}", i);
            wires.get(key.as_str())
        })
        .enumerate()
        .fold(0, |acc: usize, (i, &bit)| acc + ((bit as usize) << i))
}

fn part2(commands: &[String]) -> String {
    let commands: Vec<(&[u8], &[u8], &[u8], &[u8])> = commands
        .iter()
        .map(|command| {
            let c = command
                .split(' ')
                .map(|op| op.as_bytes())
                .collect::<Vec<_>>();
            (c[0], c[1], c[2], c[4])
        })
        .collect();

    // add bit => res = A ^ B ^ Cin
    let mut ans = commands
        .iter()
        .filter_map(|command| {
            let (input1, op, input2, output) = command;
            if op != b"XOR" && output[0] == b'z' && &output[1..] != MAX_BIT {
                return Some(String::from_utf8_lossy(output));
            }
            if op == b"XOR"
                && !(input1[0] == b'x'
                    || input2[0] == b'x'
                    || input1[0] == b'y'
                    || input2[0] == b'y'
                    || output[0] == b'z')
            {
                return Some(String::from_utf8_lossy(output));
            }

            None
        })
        .collect::<Vec<_>>();

    // carry => Cout = (A&B) | (Cin & (A^B))
    let mut input_or = HashSet::new();
    for command in &commands {
        if command.1 != b"OR" {
            continue;
        }
        input_or.insert(command.0);
        input_or.insert(command.2);
    }

    for command in &commands {
        if command.1 != b"AND" || command.0 == b"x00" {
            continue;
        }
        if !input_or.remove(command.3) {
            input_or.insert(command.3);
        }
    }

    for wrong in input_or {
        ans.push(String::from_utf8_lossy(wrong));
    }
    ans.sort_unstable();
    ans.dedup();

    ans.join(",")
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut wires = vec![];
    let mut commands = vec![];
    let mut command = false;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            command = true;
            continue;
        }

        if command {
            commands.push(line);
            continue;
        }

        wires.push(line);
    }

    println!("{}", part1(&wires, &commands));
    println!("{}", part2(&commands))
}
