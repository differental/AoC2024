use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines = contents.lines();

    let mut arr: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace().map(|s| s.parse::<i32>());

        let a = numbers.next();
        let num1 = match a {
            None => panic!(),
            Some(Ok(val)) => val,
            Some(Err(_)) => panic!(),
        };
        arr.push(num1);

        let a = numbers.next();
        let num1 = match a {
            None => panic!(),
            Some(Ok(val)) => val,
            Some(Err(_)) => panic!(),
        };
        arr2.push(num1);
    }

    arr.sort();
    arr2.sort();

    let mut sum = 0;

    for i in 0..arr.len() {
        sum += (arr[i] - arr2[i]).abs();
    }

    println!("{}", sum);

    let mut counter = HashMap::new();

    for i in 0..arr.len() {
        let count = counter.entry(arr2[i]).or_insert(0);
        *count += 1;
    }

    let mut sum = 0;

    for i in 0..arr.len() {
        let count = match counter.get(&arr[i]) {
            None => &0,
            Some(val) => val,
        };
        sum += arr[i] * count;
    }

    println!("{}", sum);
}
