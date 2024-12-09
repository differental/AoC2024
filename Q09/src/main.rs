use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let str = contents.lines().collect::<Vec<&str>>()[0];

    let mut filesystem = Vec::<i32>::new();

    let mut filesystem2 = Vec::<(i32, usize)>::new();

    let mut size = 0;

    for i in (0..str.len()).step_by(2) {
        // println!("{}", i);
        if let Some(ch) = str.chars().nth(i) {
            if let Some(digit) = ch.to_digit(10) {
                filesystem.append(&mut vec![(i / 2) as i32; digit as usize]);
                filesystem2.push(((i / 2) as i32, digit as usize));
                size += digit;
            }
        }
        if let Some(ch) = str.chars().nth(i + 1) {
            if let Some(digit) = ch.to_digit(10) {
                filesystem.append(&mut vec![-1; digit as usize]);
                filesystem2.push((-1, digit as usize));
            }
        }
    }

    let mut starting_index = 0;

    while filesystem.len() > (size as usize) {
        // Part A
        if let Some(val) = filesystem.pop() {
            if val == -1 {
                continue;
            }
            while starting_index < filesystem.len() {
                if filesystem[starting_index] == -1 {
                    filesystem[starting_index] = val;
                    break;
                }
                starting_index += 1;
            }
        }
    }

    let mut _flag = false;

    //println!("Starting: {:?}", filesystem2);

    for idx in (0..filesystem2.len()).rev() {
        let (val, size) = filesystem2[idx];
        //println!("Moving: {}, ({} {})", idx, val, size);

        if val == -1 {
            continue;
        }

        _flag = false;

        for i in 0..idx {
            let (item_val, item_size) = filesystem2[i];
            //println!("Attempting: {}, ({} {})", i, item_val, item_size);
            if item_val != -1 {
                continue;
            }
            if item_size > size {
                filesystem2[i] = (item_val, item_size - size);
                filesystem2[idx] = (-1, size);
                filesystem2.insert(i, (val, size));
                //println!("Size bigger: {}, ({} {})", i, item_val, item_size);
                _flag = true;
                break;
            } else if item_size == size {
                filesystem2[i] = (val, size);
                filesystem2[idx] = (-1, size);
                //println!("Size exact: {}, ({} {})", i, item_val, item_size);
                _flag = true;
                break;
            }
        }
        //if flag {
        //println!("Updated: {:?}", filesystem2);
        //}
    }

    let mut result1 = 0;

    for (index, item) in filesystem.into_iter().enumerate() {
        result1 += (index as u64) * (item as u64);
    }

    let mut result2 = 0;

    let mut total_index = 0;

    //println!("{:?}", filesystem2);

    // (item, size)
    // (1, 2)       (0-1) -> 1 * (0 + 1) = 1 * 2 * ((0 + 1) / 2) = item * size * (start + (start + size-1)) / 2
    // (2, 3)       (2-4) -> 2 * (2 + 3 + 4) = 2 * 3 * ((2 + 4) / 2) = item * size * (start + (start+size-1)) / 2
    for (item, size) in filesystem2.iter() {
        if *item > 0 {
            result2 += (*item as u64) * (*size as u64) * (2 * total_index + *size as u64 - 1) / 2;
        }
        total_index += *size as u64;
    }

    println!("{} {}", result1, result2);
}
