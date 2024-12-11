use std::{collections::HashMap, fs};

fn number_of_digits(&num: &u64) -> u32 {
    if num == 0 {
        return 1;
    }
    (num as f64).log10().floor() as u32 + 1
}

fn next_value(num: u64) -> (u64, i128) {
    if num == 0 {
        return (1, -1);
    }

    let digit_count = number_of_digits(&num);

    if digit_count % 2 == 0 {
        let number_a = num / (10u64.pow(digit_count / 2));
        let number_b = num % (10u64.pow(digit_count / 2));

        return (number_a, number_b as i128);
    } else {
        return (num * 2024, -1);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let line = contents.lines().next().unwrap();

    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut numbers_count = HashMap::<u64, u64>::new();

    for num in numbers.into_iter() {
        let val = numbers_count.entry(num).or_insert(0);
        *val += 1;
    }

    let mut sum: u64 = 0;

    let mut new_values: (u64, i128);

    for round in 0..75 {
        let mut new_numbers_count = HashMap::<u64, u64>::new();

        for (key, count) in numbers_count.iter() {
            new_values = next_value(*key);

            if new_values.1 == -1 {
                let val = new_numbers_count.entry(new_values.0).or_insert(0);
                *val += count;
            } else {
                let val_a = new_numbers_count.entry(new_values.0).or_insert(0);
                *val_a += count;

                let val_b = new_numbers_count.entry(new_values.1 as u64).or_insert(0);
                *val_b += count;
            }
        }

        numbers_count = new_numbers_count;

        sum = numbers_count.values().sum();

        if round == 24 {
            println!("{}", sum);
        }
        println!("{} {}", round, sum);
    }

    println!("{}", sum);
}
