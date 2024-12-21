use std::{cmp::Reverse, collections::BinaryHeap, usize};

const SUM: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
fn part1(disk: &[usize]) -> usize {
    let n = disk.len();
    let mut left = 0;
    let mut block = 0;
    let mut right = if n % 2 == 0 { n - 2 } else { n - 1 };
    let mut tail = disk[right];
    let mut ans: usize = 0;
    while left < right {
        let mut free = disk[left + 1];
        let cur = disk[left];
        ans += (block*cur + SUM[cur])*(left/2);
        block += cur;
        left += 2;
        while free > 0 {
            while right > left && tail == 0 {
                right -= 2;
                tail = disk[right];
            }

            let used = free.min(tail);
            free -= used;
            tail -= used;
            ans += (block*used + SUM[used])*(right/2);
            block += used;
        }
    }

    ans += (block*tail + SUM[tail])*(right/2);

    ans
}

fn part2(disk: &[usize]) -> usize {
    let mut occupied: Vec<(usize, usize, usize)> = vec![];
    let mut free: Vec<BinaryHeap<Reverse<usize>>> = vec![BinaryHeap::new(); 10];
    let mut offset = 0;
    for (i, &space) in disk.iter().enumerate() {
        if i % 2 == 0 {
            occupied.push((offset, i/2, space));
        } else {
            free[space].push(Reverse(offset));
        }
        offset += space;
    }

    let mut moved = vec![];
    for i in occupied.iter().rev() {
        let &(offset, id, space) = i;
        let idx = free[space..]
            .iter()
            .enumerate()
            .min_by(|a, b| {
                let k = a.1.peek().unwrap_or(&Reverse(usize::MAX)).0;
                let l = b.1.peek().unwrap_or(&Reverse(usize::MAX)).0;
                k.cmp(&l)
            })
            .map(|(index, _)| (index));
        match idx {
            None => break,
            Some(i) => {
                let new_offset = free[space+i].pop().unwrap_or(Reverse(usize::MAX)).0;
                if new_offset >= offset {
                    moved.push((offset, id, space));
                    continue;
                } 
                moved.push((new_offset, id, space));
                if i != 0 {
                    free[i].push(Reverse(new_offset + space));
                }
            }
        };
    }

    moved.iter().fold(0, |acc, (offset, id, space)| {
        acc + (offset*space + SUM[*space])*id
    })
}

fn main() {
   let line = std::io::stdin().lines().next().unwrap().unwrap().into_bytes();
    
    let disk = line.iter().map(|x| (x - b'0') as usize).collect::<Vec<_>>();
 
    println!("{}", part1(&disk));
    println!("{}", part2(&disk));
}
