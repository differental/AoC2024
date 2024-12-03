use std::fs;

fn parse_num(str: &str) -> i32 {
    let numbers: Result<Vec<i32>, _> = str
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect();

    return match numbers {
        Ok(val) => {
            if val.len() == 2 {
                //println!("{} {}", val[0], val[1]);
                return val[0] * val[1];
            }
            0
        }
        Err(_) => 0
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines = contents.lines();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut enabled = true;

    for line in lines {
        for i in 0..line.len()-4 {
            if &line[i..i+4] == "do()" {
                enabled = true;
            }
            if i+7 < line.len() && &line[i..i+7] == "don't()" {
                enabled = false;
            }
            if &line[i..i+4] == "mul(" {
                for j in i+4..line.len() {
                    if &line[j..j+1] == ")" {
                        let val = parse_num(&line[i+4..j]);
                        count1 += val;
                        if enabled {
                            count2 += val;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", count1, count2);
}