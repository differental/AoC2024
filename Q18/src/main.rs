use scan_fmt::scan_fmt_some;
use std::{collections::HashMap, fs};

const N: usize = 71;
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn helper(coord: (i32, i32)) -> bool {
    coord.0 >= 0 && coord.0 <= (N as i32 - 1) && coord.1 >= 0 && coord.1 <= (N as i32 - 1)
}

fn dfs(
    coord: (i32, i32),
    values: &mut HashMap<(i32, i32), u32>,
    steps: u32,
    matrix: &[[char; N]; N],
) {
    //println!("{:?}", coord);

    match values.get_mut(&coord) {
        Some(val) => {
            if steps < *val {
                *val = steps;
            } else {
                return;
            }
        }
        None => {
            values.insert(coord, steps);
        }
    }

    if coord == (N as i32 - 1, N as i32 - 1) {
        return;
    }

    for direction in DIRECTIONS {
        let new_coord = (coord.0 + direction.0, coord.1 + direction.1);
        if helper(new_coord) && matrix[new_coord.0 as usize][new_coord.1 as usize] != '#' {
            dfs(new_coord, values, steps + 1, matrix);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");
    let lines: Vec<&str> = contents.lines().collect();
    let mut matrix: [[char; N]; N] = [['.'; N]; N];

    for i in 0..1024 {
        let (x_opt, y_opt) = scan_fmt_some!(lines[i], "{},{}", usize, usize);
        matrix[x_opt.unwrap()][y_opt.unwrap()] = '#';
    }

    let mut values = HashMap::<(i32, i32), u32>::new();

    dfs((0, 0), &mut values, 0, &matrix);

    println!("{}", values.get(&(N as i32 - 1, N as i32 - 1)).unwrap());

    /*
       Not really proud of this solution to Part B, but it works...

       The other obvious solution would've probably been to navigate the entire maze in the first go
           and record all possible paths (including repetitive ones), then eliminate blocked paths
           with each input.

       The optimal solution is probably to have a hashmap mapping each node to all possible paths
           from there that lead to the finish. Then we no longer have to repetitively go through
           paths that we've already navigated, so the first go can be faster as well.
    */

    for i in 1024..lines.len() {
        let (x_opt, y_opt) = scan_fmt_some!(lines[i], "{},{}", usize, usize);
        matrix[x_opt.unwrap()][y_opt.unwrap()] = '#';

        let mut values = HashMap::<(i32, i32), u32>::new();
        dfs((0, 0), &mut values, 0, &matrix);

        match values.get(&(N as i32 - 1, N as i32 - 1)) {
            None => {
                println!("{} {}", x_opt.unwrap(), y_opt.unwrap());
                break;
            }
            Some(_) => (),
        }
    }
}
