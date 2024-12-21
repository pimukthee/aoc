use std::io;

fn is_valid(b: &[u8], target: &[u8], alt: &[u8]) -> bool {
    b == target || b == alt
}

fn check_horizontal(table: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    if row >= table.len() || col + 4 > table[row].len() {
        return false;
    }

    is_valid(&table[row][col..col+4], b"XMAS", b"SAMX")
}

fn check_vertical(table: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    if row + 4 > table.len() || col >= table[row].len() {
        return false;
    }
    
    let word = [table[row][col], table[row+1][col], table[row+2][col], table[row+3][col]];
    is_valid(&word, b"XMAS", b"SAMX")
}

fn check_right_diagonal(table: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    if row >= table.len() || row + 4 > table.len() || col + 4 > table[row].len() {
        return false;
    }
    
    let word = [table[row][col], table[row+1][col+1], table[row+2][col+2], table[row+3][col+3]];
    is_valid(&word, b"XMAS", b"SAMX")
}

fn check_left_diagonal(table: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    if row + 3 >= table.len() || col < 3 {
        return false;
    }
    
    let word = [table[row][col], table[row+1][col-1], table[row+2][col-2], table[row+3][col-3]];
    is_valid(&word, b"XMAS", b"SAMX")
}

fn check_x(table: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    if row + 2 >= table.len() || col + 2 >= table[row].len() {
        return false;
    }

    let left = [table[row][col], table[row+1][col+1], table[row+2][col+2]];
    let rigth = [table[row+2][col], table[row+1][col+1], table[row][col+2]];

    is_valid(&left, b"MAS", b"SAM") && is_valid(&rigth, b"MAS", b"SAM")
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut input = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        input.push(line.into_bytes());
    }
    let n = input.len();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n {
        let m = input[i].len();
        for j in 0..m {
            let pair = (i, j);
            ans1 += check_horizontal(&input, pair) as usize;
            ans1 += check_vertical(&input, pair) as usize;
            ans1 += check_right_diagonal(&input, pair) as usize;
            ans1 += check_left_diagonal(&input, pair) as usize;
            
            ans2 += check_x(&input, pair) as usize;
        }
    }
    println!("ans1 = {:?}", ans1);
    println!("ans2 = {:?}", ans2);
}