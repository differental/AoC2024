use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error"); // size: 50x50

    const N: u32 = 50;

    let mut towers = HashMap::<char, Vec<(i32, i32)>>::new();

    let lines = contents.lines();
    let mut matrix: [[char; N as usize]; N as usize] = [[' '; N as usize]; N as usize];

    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            matrix[i][j] = ch;
            if ch != '.' {
                let item = towers.entry(ch).or_insert(Vec::<(i32, i32)>::new());
                item.push((i as i32, j as i32));
            }
        }
    }

    let mut anodes = HashSet::<(i32, i32)>::new();

    for (_, pos) in &towers {
        for i in 0..pos.len() - 1 {
            for j in i + 1..pos.len() {
                let (a_x, a_y) = pos[i];
                let (b_x, b_y) = pos[j];

                let pot_x1: i32 = 2 * a_x - b_x;
                let pot_y1: i32 = 2 * a_y - b_y;

                let pot_x2: i32 = 2 * b_x - a_x;
                let pot_y2: i32 = 2 * b_y - a_y;

                if pot_x1 >= 0
                    && pot_x1 <= (N - 1) as i32
                    && pot_y1 >= 0
                    && pot_y1 <= (N - 1) as i32
                {
                    anodes.insert((pot_x1 as i32, pot_y1 as i32));
                }
                if pot_x2 >= 0
                    && pot_x2 <= (N - 1) as i32
                    && pot_y2 >= 0
                    && pot_y2 <= (N - 1) as i32
                {
                    anodes.insert((pot_x2 as i32, pot_y2 as i32));
                }
            }
        }
    }
    println!("{}", anodes.len());

    for (_, pos) in &towers {
        for i in 0..pos.len() - 1 {
            for j in i + 1..pos.len() {
                let (a_x, a_y) = pos[i];
                let (b_x, b_y) = pos[j];

                for loc_i in 0..N {
                    // not fantastic but it works for small cases...
                    for loc_j in 0..N {
                        let _dx = ((loc_i as i32 - a_x) as f64) / ((b_x - a_x) as f64);
                        let _dy = ((loc_j as i32 - a_y) as f64) / ((b_y - a_y) as f64);

                        let val_a = (loc_i as i32 - a_x) * (b_y - a_y);
                        let val_b = (loc_j as i32 - a_y) * (b_x - a_x);

                        if val_a == val_b {
                            anodes.insert((loc_i as i32, loc_j as i32));
                        }
                    }
                }
            }
        }
    }
    println!("{}", anodes.len());
}
