fn prefix_sum(arr: &mut [[[[[usize; 7]; 7]; 7]; 7]; 7]) {
    for col in 0..5 {
        for a in 0..6 {
            for b in 0..6 {
                for c in 0..6 {
                    for d in 0..6 {
                        for e in 0..6 {
                            match col {
                                0 => arr[a+1][b][c][d][e] += arr[a][b][c][d][e],
                                1 => arr[a][b+1][c][d][e] += arr[a][b][c][d][e],
                                2 => arr[a][b][c+1][d][e] += arr[a][b][c][d][e],
                                3 => arr[a][b][c][d+1][e] += arr[a][b][c][d][e],
                                4 => arr[a][b][c][d][e+1] += arr[a][b][c][d][e],
                                _ => unreachable!()
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut keys = [[[[[0; 7]; 7]; 7]; 7]; 7];
    let mut down_keys = vec![];

    while let Some(Ok(mut line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut pin = [0; 5];
        let mut is_down = false;
        if line.as_bytes()[0] == b'#' {
            is_down = true;
        }
        for j in 0..7 {
            for (i, p) in line.as_bytes().iter().enumerate() {
                if p == &b'#' {
                    pin[i] += 1;
                }
            }
            if j != 6 {
                line = lines.next().unwrap().unwrap();
            }
        }

        
        if is_down {
            down_keys.push([pin[0]-1, pin[1]-1, pin[2]-1, pin[3]-1, pin[4]-1]);
        } else {
            keys[pin[0]-1][pin[1]-1][pin[2]-1][pin[3]-1][pin[4]-1] += 1;
        }
    }

    prefix_sum(&mut keys);
    
    let ans = down_keys.iter().map(|down_key| keys[5 - down_key[0]][5-down_key[1]][5 - down_key[2]][5 - down_key[3]][5 - down_key[4]]).sum::<usize>();
    println!("{}", ans);
}