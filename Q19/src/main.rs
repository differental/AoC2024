use std::fs;

const MAX_LEN: usize = 65;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Input error");
    let lines: Vec<&str> = contents.lines().collect();

    let towels: Vec<&str> = lines[0].split(',').map(|t| t.trim()).collect();
    let targets: Vec<&str> = lines[1..]
        .iter()
        .map(|t| t.trim())
        .filter(|t| *t != "")
        .collect();

    let mut count1: u64 = 0;
    let mut count2: u64 = 0;

    for target in targets {
        let mut dp: [u64; 65] = [0; MAX_LEN];
        dp[0] = 1;

        for i in 0..target.len() {
            if dp[i] == 0 {
                continue;
            }

            for towel in &towels {
                if i + towel.len() - 1 <= target.len() - 1 && **towel == target[i..i + towel.len()]
                {
                    dp[i + towel.len()] += dp[i];
                }
            }
        }

        if dp[target.len()] != 0 {
            count1 += 1;
        }
        count2 += dp[target.len()];
    }

    println!("{} {}", count1, count2);
}
