use std::{
    collections::{HashSet, VecDeque},
    fs,
};

const N: u32 = 5; // size: 140X140

fn helper(x: i32, y: i32, to_visit: &HashSet::<(i32, i32)>, matrix: &[[char; N as usize]; N as usize], ch: char) -> (bool, bool) { 
    // return (whether_to_add_to_circum, whether_to_explore);
    
    // if out of bounds, add to circum but do not explore
    if !(x >= 0 && x <= N as i32 - 1 && y >= 0 && y <= N as i32 - 1) {
        return (true, false);
    }

    // if in bounds and char is different, add to circum but do not explore
    if matrix[x as usize][y as usize] != ch {
        return (true, false);
    }

    // if in bounds and char is same, but already visited, don't add to circum and don't explore
    if !to_visit.contains(&(x, y)) {
        return (false, false);
    }

    // if in bounds and char is same, and haven't visited, don't add to circum but do explore
    (false, true)
}

fn explore(start_i: i32, start_j: i32, circum: &mut u32, area: &mut u32, to_visit: &mut HashSet::<(i32, i32)>, matrix: &[[char; N as usize]; N as usize], barriers: &mut HashSet::<(i32, i32, u32)>) {

    let mut queue = VecDeque::<(i32, i32)>::new();

    let mut i = start_i;
    let mut j = start_j;

    queue.push_back((i, j));

    while queue.len() != 0 {

        (i, j) = queue.pop_front().expect("Should not happen since empty queue doesn't satisfy while condition");

        *area += 1;
        to_visit.remove(&(i, j));
        print!("({i}, {j}) -> ");

        let ch = matrix[i as usize][j as usize];
        
        let up = helper(i-1, j, &to_visit, matrix, ch);
        if up.0 {
            // Going up needs a barrier. Can we get it for free? We can if left or right already has one
            // barrier: no need to bound check since it's (i32,i32)
            if !barriers.contains(&(i, j-1, 0)) && !barriers.contains(&(i, j+1, 0)) {
                //println!("New Barrier at ({i}, {j}), up");
                *circum += 1;
            }

            //println!("Barrier at ({i}, {j}), up");
            barriers.insert((i, j, 0)); // 0: up
        }
        if up.1 && !queue.contains(&(i-1, j)) {
            queue.push_back((i-1, j));
        }

        let right = helper(i, j+1, &to_visit, matrix, ch);
        if right.0 {
            if !barriers.contains(&(i-1, j, 3)) && !barriers.contains(&(i+1, j, 3)) {
                //println!("New Barrier at ({i}, {j}), right");
                *circum += 1;
            }

            //println!("Barrier at ({i}, {j}), right");
            barriers.insert((i, j, 3)); // 3: right
        }
        if right.1 && !queue.contains(&(i, j+1)) {
            queue.push_back((i, j+1));
        }

        let down = helper(i+1, j, &to_visit, matrix, ch);
        if down.0 {
            if !barriers.contains(&(i, j-1, 1)) && !barriers.contains(&(i, j+1, 1)) {
                //println!("New Barrier at ({i}, {j}), down");
                *circum += 1;
            }

            //println!("Barrier at ({i}, {j}), down");
            barriers.insert((i, j, 1)); // 1: down
        }
        if down.1 && !queue.contains(&(i+1, j)) {
            queue.push_back((i+1, j));
        }

        let left = helper(i, j-1, &to_visit, matrix, ch);
        if left.0 {
            if !barriers.contains(&(i-1, j, 2)) && !barriers.contains(&(i+1, j, 2)) {
                //println!("New Barrier at ({i}, {j}), left");
                *circum += 1;
            }

            //println!("Barrier at ({i}, {j}), left");
            barriers.insert((i, j, 2)); // 2: left
        }
        if left.1 && !queue.contains(&(i, j-1)) {
            queue.push_back((i, j-1));
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error"); 


    let lines = contents.lines();
    let mut matrix: [[char; N as usize]; N as usize] = [[' '; N as usize]; N as usize];

    let mut to_visit = HashSet::<(i32, i32)>::new();

    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            matrix[i][j] = ch;
            to_visit.insert((i as i32, j as i32));
        }
    }

    let mut result1 = 0;
    let mut result2 = 0;

    while !to_visit.is_empty() {
        let next_node: Vec<&(i32, i32)> = to_visit.iter().collect();
        let (i, j) = *next_node[0];
        let mut circum: u32 = 0;
        let mut area: u32 = 0;

        let mut barriers = HashSet::<(i32, i32, u32)>::new(); // i, j, location_relative_to_node (0-3)

        explore(i, j, &mut circum, &mut area, &mut to_visit, &matrix, &mut barriers);
        let real_circum = barriers.len() as u32;

        result1 += real_circum * area;
        result2 += circum * area;

        println!("{:?} explored, circum {circum}, real_circum {real_circum}, area {area}", (i, j));
    }
    println!("{} {}", result1, result2);
}
