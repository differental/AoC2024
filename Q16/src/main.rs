use std::{
    collections::{HashMap, HashSet},
    fs,
};

const N: usize = 141;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs(
    matrix: &[[char; N]; N],
    direction: usize,
    coords: (i32, i32),
    target: (i32, i32),
    val: u32,
    scores: &mut HashMap<(i32, i32, usize), u32>,
    min_paths: &mut HashSet<(i32, i32)>,
    min_val: &mut u32,
    current_path: &mut Vec<(i32, i32)>,
) {
    let (x, y) = coords;

    //println!("{} {} {}", x, y, val);

    if val > *min_val {
        return;
    }

    if x == target.0 && y == target.1 {
        if val == *min_val {
            let current_path_copy = current_path.clone();
            min_paths.extend(current_path_copy);
        } else if val < *min_val {
            *min_val = val;

            let current_path_copy = current_path.clone();
            min_paths.clear();
            min_paths.extend(current_path_copy);
        }
        return;
    }

    match scores.get_mut(&(x, y, direction)) {
        Some(ori_val) => {
            if val > *ori_val {
                return;
            } else {
                *ori_val = val;
            }
        }
        None => {
            scores.insert((x, y, direction), val);
        }
    }

    let (dx, dy) = DIRECTIONS[direction];

    if x + dx >= 0
        && x + dx <= N as i32 - 1
        && y + dy >= 0
        && y + dy <= N as i32 - 1
        && matrix[(x + dx) as usize][(y + dy) as usize] != '#'
    {
        current_path.push((x + dx, y + dy));
        dfs(
            matrix,
            direction,
            (x + dx, y + dy),
            target,
            val + 1,
            scores,
            min_paths,
            min_val,
            current_path,
        );
        current_path.pop();
    }

    for i in 0..4 {
        if i == direction || i == (direction + 2) % 4 {
            // no forwards or opposite
            continue;
        }

        let (dx, dy) = DIRECTIONS[i];

        if x + dx >= 0
            && x + dx <= N as i32 - 1
            && y + dy >= 0
            && y + dy <= N as i32 - 1
            && matrix[(x + dx) as usize][(y + dy) as usize] != '#'
        {
            current_path.push((x + dx, y + dy));
            dfs(
                matrix,
                i,
                (x + dx, y + dy),
                target,
                val + 1001,
                scores,
                min_paths,
                min_val,
                current_path,
            );
            current_path.pop();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines: Vec<_> = contents.lines().collect();
    let mut matrix: [[char; N]; N] = [[' '; N]; N];

    for i in 0..N {
        for (j, ch) in lines[i].chars().enumerate() {
            matrix[i][j] = ch;
        }
    }

    let mut score_map = HashMap::<(i32, i32, usize), u32>::new();

    let mut min_set = HashSet::<(i32, i32)>::new();
    let mut min_val = u32::MAX;

    let mut current_path = Vec::<(i32, i32)>::new();

    current_path.push((N as i32 - 2, 1));

    dfs(
        &matrix,
        1,
        (N as i32 - 2, 1),
        (1, N as i32 - 2),
        0,
        &mut score_map,
        &mut min_set,
        &mut min_val,
        &mut current_path,
    );

    println!("{} {}", min_val, min_set.len());
}
