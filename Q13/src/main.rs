use scan_fmt::scan_fmt_some;
use std::{cmp::min, fs};

fn calc_ans(a_x: i64, a_y: i64, b_x: i64, b_y: i64, prize_x: i64, prize_y: i64) -> i64 {
    // a_x * a + b_x * b = prize_x
    // a_y * a + b_y * b = prize_y

    // Given all known values are positive integers, (a,b):
    //
    // A. a_x / a_y != b_x / b_y
    //    (i.e., either only one between b_x and b_y is zero, or divisions don't match)
    //    In this case, we have unique solution (a,b). Just need to ensure it's int and positive
    //
    // B. a_x / a_y == b_x / b_y && a_x / a_y == prize_x / prize_y
    //    Infinite solutions, we need to compare
    //    a_x / 3 and b_x / 1
    //    to minimise token spending, and we need to make sure both are positive ints
    //
    // C. a_x / a_y == b_x / b_y && a_x / a_y != prize_x / prize_y
    //    No solutions

    // Solve a,b:
    // a = (prize_x b_y - prize_y b_x) / (a_x b_y - a_y b_x)
    // b = (prize_y a_x - prize_x a_y) / (a_x b_y - a_y b_x)

    let denom = a_x * b_y - a_y * b_x;
    let numer_a = prize_x * b_y - prize_y * b_x;
    let numer_b = prize_y * a_x - prize_x * a_y;

    if denom != 0 && numer_a % denom == 0 && numer_b % denom == 0 {
        let a = numer_a / denom;
        let b = numer_b / denom;

        if a >= 0 && b >= 0 {
            return a * 3 + b;
        }
        return 0;
    }

    if denom == 0 && numer_a == 0 {
        let mut min_val = i64::MAX;
        for b in 0..=prize_x / b_x {
            if (prize_x - b * b_x) % a_x == 0 {
                let a = (prize_x - b * b_x) / a_x;
                min_val = 3 * a + b;
                break;
            }
        }
        for b in prize_x / b_x..=0 {
            if (prize_x - b * b_x) % a_x == 0 {
                let a = (prize_x - b * b_x) / a_x;
                min_val = min(min_val, 3 * a + b);
                break;
            }
        }
        return min_val;
    }
    0
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error"); // size: 50x50
    let lines = contents.lines();

    let mut a_x = 0;
    let mut a_y = 0;
    let mut b_x = 0;
    let mut b_y = 0;
    let mut prize_x;
    let mut prize_y;

    let mut result_1 = 0;
    let mut result_2 = 0;

    for line in lines {
        if line.starts_with("Button A") {
            let (a_x_opt, a_y_opt) = scan_fmt_some!(line, "Button A: X+{}, Y+{}", i64, i64);
            a_x = a_x_opt.unwrap();
            a_y = a_y_opt.unwrap();
        }

        if line.starts_with("Button B") {
            let (b_x_opt, b_y_opt) = scan_fmt_some!(line, "Button B: X+{}, Y+{}", i64, i64);
            b_x = b_x_opt.unwrap();
            b_y = b_y_opt.unwrap();
        }

        if line.starts_with("Prize") {
            let (prize_x_opt, prize_y_opt) = scan_fmt_some!(line, "Prize: X={}, Y={}", i64, i64);
            prize_x = prize_x_opt.unwrap();
            prize_y = prize_y_opt.unwrap();

            result_1 += calc_ans(a_x, a_y, b_x, b_y, prize_x, prize_y);

            prize_x += 10i64.pow(13);
            prize_y += 10i64.pow(13);
            result_2 += calc_ans(a_x, a_y, b_x, b_y, prize_x, prize_y);
        }
    }
    println!("{} {}", result_1, result_2);
}
