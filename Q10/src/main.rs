use std::{collections::HashSet, fs};

const N: usize = 55;

fn dfs(
    coords: (usize, usize),
    val: u32,
    &matrix: &[[u32; N]; N],
    mut visited9: &mut HashSet<(usize, usize)>,
    remove_dupe: bool,
) -> u32 {
    let (i, j) = coords as (usize, usize);

    if remove_dupe && val == 9 && !visited9.contains(&coords) {
        visited9.insert(coords);
        return 1;
    }
    if !remove_dupe && val == 9 {
        return 1;
    }

    fn check_coords(x: i32, y: i32, &matrix: &[[u32; N]; N], val: u32) -> bool {
        if x < 0 || x > (N - 1) as i32 {
            return false;
        }
        if y < 0 || y > (N - 1) as i32 {
            return false;
        }
        if matrix[x as usize][y as usize] != val {
            return false;
        }

        true
    }

    let res_left = if check_coords(i as i32, j as i32 - 1, &matrix, val + 1) {
        dfs((i, j - 1), val + 1, &matrix, &mut visited9, remove_dupe)
    } else {
        0
    };
    let res_right = if check_coords(i as i32, j as i32 + 1, &matrix, val + 1) {
        dfs((i, j + 1), val + 1, &matrix, &mut visited9, remove_dupe)
    } else {
        0
    };
    let res_up = if check_coords(i as i32 - 1, j as i32, &matrix, val + 1) {
        dfs((i - 1, j), val + 1, &matrix, &mut visited9, remove_dupe)
    } else {
        0
    };
    let res_down = if check_coords(i as i32 + 1, j as i32, &matrix, val + 1) {
        dfs((i + 1, j), val + 1, &matrix, &mut visited9, remove_dupe)
    } else {
        0
    };

    return res_left + res_right + res_up + res_down;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error"); // size: 55x55

    let lines = contents.lines();
    let mut matrix: [[u32; N]; N] = [[0; N]; N];

    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if let Some(num) = ch.to_digit(10) {
                matrix[i][j] = num;
            }
        }
    }

    let mut ans1 = 0;
    let mut ans2 = 0;

    for (i, row) in (&matrix).into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            if *val == 0 {
                let mut visited_9s = HashSet::<(usize, usize)>::new();

                ans1 += dfs((i, j), 0, &matrix, &mut visited_9s, true); // part A
                ans2 += dfs((i, j), 0, &matrix, &mut visited_9s, false); // part B
            }
        }
    }
    println!("{} {}", ans1, ans2);
}
