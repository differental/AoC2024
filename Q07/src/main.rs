use std::fs;

fn dfs(arr: &Vec<i128>, idx: i32, target: i128, current: i128, allow_concat: bool) -> bool{

    //println!("{} {} {}", idx, target, current);
    if current == target && idx as usize == arr.len() {
        return true;
    }
    if idx as usize == arr.len() {
        return false;
    }

    let val = arr[idx as usize];

    if dfs(arr, idx + 1, target, current + val, allow_concat)
    //|| dfs(arr, idx + 1, target, current - val)
    || dfs(arr, idx + 1, target, current * val, allow_concat) {
    //|| (current % val == 0 && dfs(arr, idx + 1, target, current / val)) {
        return true;
    }

    if allow_concat && dfs(arr, idx+1, target, current * 10i128.pow(val.to_string().len() as u32) + val, allow_concat) {
        return true;
    }

    false
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines = contents.lines();
    let mut count = 0;
    let mut count2 = 0;

    for line in lines {
        let chunks: Vec<&str> = line.split(':').collect();
        if chunks.len() != 2 {
            panic!("Error");
        }

        if let Ok(result) = chunks[0].parse() as Result<i128, _> {
            let numbers: Vec<i128> = chunks[1].trim().split(' ').map(|x| match x.trim().parse::<i128>() {
                Ok(val) => val,
                Err(_) => {
                    println!("{:?}", chunks[1]);
                    panic!("ParseIntError")
                }
            }).collect();
            if dfs(&numbers, 1, result, numbers[0], false) {
                count += result;
                //println!("{}", result);
            }
            if dfs(&numbers, 1, result, numbers[0], true) {
                count2 += result;
                //println!("{}", result);
            }
        }
    }
    println!("{} {}", count, count2);
}
