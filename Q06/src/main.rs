use std::{collections::HashSet, fs};

fn main() {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // 0: up, 1: right, 2: down, 3: left

    let contents = fs::read_to_string("input.txt").expect("Read error"); // size: 140x140

    let lines = contents.lines();
    let mut matrix: [[char; 130]; 130] = [[' '; 130]; 130];

    let mut state: (i32, i32, u32);
    let mut starting_state: (i32, i32, u32) = (0, 0, 0);

    let mut past_states: HashSet<(i32, i32, u32)> = HashSet::new();
    let mut visited_nodes: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            matrix[i][j] = ch;
            if ch == '^' {
                starting_state.0 = i as i32;
                starting_state.1 = j as i32;
            }
        }
    }
    state = starting_state;

    let mut new_x;
    let mut new_y;
    let mut count = 0;

    loop {
        // println!("{:?}", state);
        past_states.insert(state);
        if !visited_nodes.contains(&(state.0, state.1)) {
            visited_nodes.insert((state.0, state.1));
            count += 1;
        }

        new_x = state.0 + directions[state.2 as usize].0;
        new_y = state.1 + directions[state.2 as usize].1;

        let mut new_state = (new_x, new_y, state.2);
        let mut flag = false;

        if new_state.0 < 0
            || new_state.0 > 130 - 1
            || new_state.1 < 0
            || new_state.1 > 130 - 1
            || past_states.contains(&new_state)
        {
            break;
        }

        while matrix[new_state.0 as usize][new_state.1 as usize] == '#' {
            new_state.2 = (new_state.2 + 1) % 4;

            new_x = state.0 + directions[new_state.2 as usize].0;
            new_y = state.1 + directions[new_state.2 as usize].1;
            new_state = (new_x, new_y, new_state.2);

            if new_state.0 < 0
                || new_state.0 > 130 - 1
                || new_state.1 < 0
                || new_state.1 > 130 - 1
                || past_states.contains(&new_state)
            {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }

        state = new_state;
    }
    println!("{}", count);

    let mut count2 = 0;
    let original_visited_nodes = visited_nodes.clone();

    for (obs_i, obs_j) in original_visited_nodes {
        if matrix[obs_i as usize][obs_j as usize] != '.' {
            continue;
        }
        //println!("Testing {:?}", (obs_i, obs_j));
        state = starting_state;
        let mut past_states: HashSet<(i32, i32, u32)> = HashSet::new();
        let mut visited_nodes: HashSet<(i32, i32)> = HashSet::new();
        loop {
            // println!("{:?}", state);
            past_states.insert(state);
            if !visited_nodes.contains(&(state.0, state.1)) {
                visited_nodes.insert((state.0, state.1));
                count += 1;
            }

            new_x = state.0 + directions[state.2 as usize].0;
            new_y = state.1 + directions[state.2 as usize].1;

            let mut new_state = (new_x, new_y, state.2);
            let mut flag = false;

            if new_state.0 < 0 || new_state.0 > 130 - 1 || new_state.1 < 0 || new_state.1 > 130 - 1
            {
                break;
            }
            if past_states.contains(&new_state) {
                count2 += 1;
                break;
            }

            while matrix[new_state.0 as usize][new_state.1 as usize] == '#'
                || (new_state.0 == obs_i && new_state.1 == obs_j)
            {
                new_state.2 = (new_state.2 + 1) % 4;

                new_x = state.0 + directions[new_state.2 as usize].0;
                new_y = state.1 + directions[new_state.2 as usize].1;
                new_state = (new_x, new_y, new_state.2);

                if new_state.0 < 0
                    || new_state.0 > 130 - 1
                    || new_state.1 < 0
                    || new_state.1 > 130 - 1
                {
                    flag = true;
                    break;
                }
                if past_states.contains(&new_state) {
                    flag = true;
                    count2 += 1;
                    break;
                }
            }
            if flag {
                break;
            }

            state = new_state;
        }
    }
    println!("{}", count2);
}
