use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error"); // size: 140x140

    let lines = contents.lines();
    let mut matrix: [[char; 140]; 140] = [[' '; 140]; 140];

    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            matrix[i][j] = ch
        }
    }

    //println!("{:?}", matrix);

    let mut count = 0;
    let mut count2 = 0;

    let pattern = ['X', 'M', 'A', 'S'];
    let pattern_res = ['S', 'A', 'M', 'X'];

    let pattern2 = ['M', 'A', 'S'];

    for i in 0..140 {
        for j in 0..140 {
            if matrix[i][j] == 'X' {
                // Task 1
                // HORIZONTAL
                if j >= 3 && &matrix[i][j - 3..j + 1] == pattern_res {
                    count += 1;
                }
                if j <= 140 - 4 && &matrix[i][j..j + 4] == pattern {
                    count += 1;
                }

                // VERTICAL
                if i >= 3 && (0..4).all(|k| matrix[i - k][j] == pattern[k]) {
                    count += 1;
                }
                if i <= 140 - 4 && (0..4).all(|k| matrix[i + k][j] == pattern[k]) {
                    count += 1;
                }

                // DIAGONAL
                if i <= 140 - 4
                    && j <= 140 - 4
                    && (0..4).all(|k| matrix[i + k][j + k] == pattern[k])
                {
                    count += 1;
                }
                if i >= 3 && j <= 140 - 4 && (0..4).all(|k| matrix[i - k][j + k] == pattern[k]) {
                    count += 1;
                }
                if i <= 140 - 4 && j >= 3 && (0..4).all(|k| matrix[i + k][j - k] == pattern[k]) {
                    count += 1;
                }
                if i >= 3 && j >= 3 && (0..4).all(|k| matrix[i - k][j - k] == pattern[k]) {
                    count += 1;
                }
            }
            if i >= 1 && i <= 140 - 2 && j >= 1 && j <= 140 - 2 && matrix[i][j] == 'A' {
                if ((-1..2).all(|k: isize| {
                    let x = i as isize + k;
                    let y = j as isize + k;
                    matrix[x as usize][y as usize] == pattern2[(k + 1) as usize]
                }) || (-1..2).all(|k: isize| {
                    let x = i as isize - k;
                    let y = j as isize - k;
                    matrix[x as usize][y as usize] == pattern2[(k + 1) as usize]
                })) && ((-1..2).all(|k: isize| {
                    let x = i as isize - k;
                    let y = j as isize + k;
                    matrix[x as usize][y as usize] == pattern2[(k + 1) as usize]
                }) || (-1..2).all(|k: isize| {
                    let x = i as isize + k;
                    let y = j as isize - k;
                    matrix[x as usize][y as usize] == pattern2[(k + 1) as usize]
                })) {
                    count2 += 1;
                }
            }
        }
    }
    println!("{} {}", count, count2);
}
