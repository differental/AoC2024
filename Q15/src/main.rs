use std::{collections::HashSet, fs};

const N: usize = 50;

fn process_a(matrix: &mut [[char; N]; N], direction: u32, location: &mut (u32, u32)) -> () {
    let (x, y) = location;
    let x = *x as usize;
    let y = *y as usize;

    match direction {
        0 => {
            // going up
            let mut has_box = false;

            for i in (0..x).rev() {
                match matrix[i][y] {
                    '#' => break,
                    'O' => has_box = true,
                    '.' => {
                        if has_box {
                            matrix[i][y] = 'O';
                            matrix[x - 1][y] = '@'; // no bound check needed - border is always '#'
                            *location = ((x - 1) as u32, y as u32);
                            matrix[x][y] = '.';
                            break;
                        } else {
                            matrix[x - 1][y] = '@';
                            *location = ((x - 1) as u32, y as u32);
                            matrix[x][y] = '.';
                            break;
                        }
                    }
                    _ => panic!(), // This would never happen
                }
            }
        }
        1 => {
            // going right
            let mut has_box = false;

            for j in y + 1..N {
                match matrix[x][j] {
                    '#' => break,
                    'O' => has_box = true,
                    '.' => {
                        if has_box {
                            matrix[x][j] = 'O';
                            matrix[x][y + 1] = '@';
                            *location = (x as u32, (y + 1) as u32);
                            matrix[x][y] = '.';
                            break;
                        } else {
                            matrix[x][y + 1] = '@';
                            *location = (x as u32, (y + 1) as u32);
                            matrix[x][y] = '.';
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        2 => {
            // going down
            let mut has_box = false;

            for i in x + 1..N {
                match matrix[i][y] {
                    '#' => break,
                    'O' => has_box = true,
                    '.' => {
                        if has_box {
                            matrix[i][y] = 'O';
                            matrix[x + 1][y] = '@';
                            *location = ((x + 1) as u32, y as u32);
                            matrix[x][y] = '.';
                            break;
                        } else {
                            matrix[x + 1][y] = '@';
                            *location = ((x + 1) as u32, y as u32);
                            matrix[x][y] = '.';
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        3 => {
            // going left
            let mut has_box = false;

            for j in (0..y).rev() {
                match matrix[x][j] {
                    '#' => break,
                    'O' => has_box = true,
                    '.' => {
                        if has_box {
                            matrix[x][j] = 'O';
                            matrix[x][y - 1] = '@';
                            *location = (x as u32, (y - 1) as u32);
                            matrix[x][y] = '.';
                            break;
                        } else {
                            matrix[x][y - 1] = '@';
                            *location = (x as u32, (y - 1) as u32);
                            matrix[x][y] = '.';
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        _ => (),
    }
}

fn process_b(matrix: &mut [[char; 2 * N]; N], direction: u32, location: &mut (u32, u32)) -> () {
    let (x, y) = location;
    let x = *x as usize;
    let y = *y as usize;

    match direction {
        0 => {
            // going up
            let mut checking = Vec::<usize>::from([y]);
            let mut is_blocked;
            let mut is_clear = true;

            // coords of the '[' of all blocks that needed movement.
            // i from small to large, they need to all be moved upwards
            let mut all_blocks = Vec::<(usize, usize)>::new();

            let mut _target_i = 0;

            for i in (0..x).rev() {
                let mut to_check = HashSet::<usize>::new();
                is_blocked = false;
                is_clear = true;
                for j in &checking {
                    match matrix[i][*j] {
                        '#' => {
                            is_blocked = true;
                            is_clear = false;
                            break;
                        }
                        '[' => {
                            to_check.insert(*j);
                            to_check.insert(*j + 1);
                            all_blocks.push((i, *j));
                            is_clear = false;
                        }
                        ']' => {
                            to_check.insert(*j - 1);
                            to_check.insert(*j);
                            all_blocks.push((i, *j - 1));
                            is_clear = false;
                        }
                        '.' => (),
                        _ => panic!(),
                    }
                }
                if is_blocked {
                    break;
                }
                if is_clear {
                    _target_i = i;
                    break;
                }

                let mut next_check: Vec<usize> = to_check.into_iter().collect();
                checking.clear();
                checking.append(&mut next_check);
            }

            if is_clear {
                // target i located and we can move
                matrix[x][y] = '.';
                *location = ((x - 1) as u32, y as u32);

                loop {
                    match all_blocks.pop() {
                        Some((i, j)) => {
                            matrix[i][j] = '.';
                            matrix[i][j + 1] = '.';
                            matrix[i - 1][j] = '[';
                            matrix[i - 1][j + 1] = ']';
                        }
                        None => break,
                    }
                }

                matrix[x - 1][y] = '@';
            }
        }
        1 => {
            // going right
            let mut has_box = false;
            let mut all_blocks = Vec::<(usize, usize)>::new();

            for j in y + 1..2 * N {
                match matrix[x][j] {
                    '#' => break,
                    '[' => {
                        all_blocks.push((x, j));
                        has_box = true;
                    }
                    ']' => (), // already handled '['
                    '.' => {
                        //print!("Result...");
                        if has_box {
                            //print!("Has box");
                            matrix[x][y] = '.';
                            *location = (x as u32, (y + 1) as u32);

                            all_blocks.iter().rev().for_each(|(a, b)| {
                                matrix[*a][*b] = '.';
                                matrix[*a][*b + 1] = '[';
                                matrix[*a][*b + 2] = ']';
                            });

                            matrix[x][y + 1] = '@';
                            break;
                        } else {
                            //print!("No box");
                            matrix[x][y] = '.';
                            matrix[x][y + 1] = '@';
                            *location = (x as u32, (y + 1) as u32);
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        2 => {
            // going down
            let mut checking = Vec::<usize>::from([y]);
            let mut is_blocked;
            let mut is_clear = true;

            let mut all_blocks = Vec::<(usize, usize)>::new();

            let mut _target_i = 0;

            for i in x + 1..N {
                let mut to_check = HashSet::<usize>::new();
                is_blocked = false;
                is_clear = true;
                for j in &checking {
                    match matrix[i][*j] {
                        '#' => {
                            is_blocked = true;
                            is_clear = false;
                            break;
                        }
                        '[' => {
                            to_check.insert(*j);
                            to_check.insert(*j + 1);
                            all_blocks.push((i, *j));
                            is_clear = false;
                        }
                        ']' => {
                            to_check.insert(*j - 1);
                            to_check.insert(*j);
                            all_blocks.push((i, *j - 1));
                            is_clear = false;
                        }
                        '.' => (),
                        _ => panic!(),
                    }
                }
                if is_blocked {
                    break;
                }
                if is_clear {
                    _target_i = i;
                    break;
                }

                let mut next_check: Vec<usize> = to_check.into_iter().collect();
                checking.clear();
                checking.append(&mut next_check);
            }

            if is_clear {
                matrix[x][y] = '.';
                *location = ((x + 1) as u32, y as u32);

                loop {
                    match all_blocks.pop() {
                        Some((i, j)) => {
                            matrix[i][j] = '.';
                            matrix[i][j + 1] = '.';
                            matrix[i + 1][j] = '[';
                            matrix[i + 1][j + 1] = ']';
                        }
                        None => break,
                    }
                }

                matrix[x + 1][y] = '@';
            }
        }
        3 => {
            // going left
            let mut has_box = false;
            let mut all_blocks = Vec::<(usize, usize)>::new();

            for j in (0..y).rev() {
                match matrix[x][j] {
                    '#' => break,
                    ']' => {
                        all_blocks.push((x, j - 1));
                        has_box = true;
                    }
                    '[' => (), // already handled ']'
                    '.' => {
                        if has_box {
                            matrix[x][y] = '.';
                            *location = (x as u32, (y - 1) as u32);

                            all_blocks.iter().rev().for_each(|(a, b)| {
                                matrix[*a][*b + 1] = '.';
                                matrix[*a][*b] = ']';
                                matrix[*a][*b - 1] = '[';
                            });

                            matrix[x][y - 1] = '@';
                            break;
                        } else {
                            matrix[x][y] = '.';
                            matrix[x][y - 1] = '@';
                            *location = (x as u32, (y - 1) as u32);
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        _ => (),
    }

    /*println!("Move {direction}");
    let mut count = 0;
    for i in 0..N {
        for j in 0..2*N {
            print!("{}", matrix[i][j]);
            if matrix[i][j] == '[' {
                count += 1;
            }
        }
        println!();
    }
    println!("Count: {count}");*/
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines: Vec<_> = contents.lines().collect();
    let mut matrix: [[char; N]; N] = [[' '; N]; N];
    let mut commands = Vec::<u32>::new();
    // 0: ^, 1: >, 2: v, 3: <

    let mut location: (u32, u32) = (0, 0);

    for i in 0..N {
        for (j, ch) in lines[i].chars().enumerate() {
            matrix[i][j] = ch;
            if ch == '@' {
                location = (i as u32, j as u32);
            }
        }
    }
    for line in lines[N..].iter() {
        for ch in line.chars() {
            commands.push(match ch {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => continue,
            });
        }
    }

    commands
        .iter()
        .for_each(|t| process_a(&mut matrix, *t, &mut location));

    let mut result_1 = 0;

    for i in 0..N {
        for j in 0..N {
            if matrix[i][j] == 'O' {
                result_1 += 100 * i + j;
            }
        }
    }

    let mut matrix: [[char; 2 * N]; N] = [[' '; 2 * N]; N];

    for i in 0..N {
        for (j, ch) in lines[i].chars().enumerate() {
            match ch {
                '#' => {
                    matrix[i][2 * j] = '#';
                    matrix[i][2 * j + 1] = '#';
                }
                '@' => {
                    location = (i as u32, 2 * j as u32);
                    matrix[i][2 * j] = '@';
                    matrix[i][2 * j + 1] = '.';
                }
                '.' => {
                    matrix[i][2 * j] = '.';
                    matrix[i][2 * j + 1] = '.';
                }
                'O' => {
                    matrix[i][2 * j] = '[';
                    matrix[i][2 * j + 1] = ']';
                }
                _ => panic!(),
            }
        }
    }

    commands
        .iter()
        .for_each(|t| process_b(&mut matrix, *t, &mut location));

    let mut result_2 = 0;

    for i in 0..N {
        for j in 0..2 * N {
            if matrix[i][j] == '[' {
                result_2 += 100 * i + j;
            }
        }
    }

    println!("{} {}", result_1, result_2);
}
