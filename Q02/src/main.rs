use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines = contents.lines();

    let mut count = 0; 
    let mut count2 = 0;

    for line in lines {
        let numbers = line.split_whitespace().map(|s| s.parse::<i32>());

        let mut nums: Vec<i32> = Vec::new();

        for number in numbers {
            let num = match number {
                Ok(val) => val,
                Err(_) => panic!(),
            };
            nums.push(num);
        }

        //println!("{:#?}", nums);

        let mut flag = true;
        let mut errored = false;
        let mut prev_val = nums[0];

        let increasing = nums[1] > nums[0];

        for i in 1..nums.len() {
            if (nums[i] > prev_val) == increasing {
                let val = (nums[i] - prev_val).abs();
                if val < 1 || val > 3 {
                    if errored {
                        flag = false;
                        break;
                    } else {
                        errored = true;
                    }
                } else {
                    prev_val = nums[i]
                }
            } else {
                if errored {
                    flag = false;
                    break;
                } else {
                    errored = true;
                }
            }
        }

        if flag && !errored {
            count += 1;
        }
        if flag {
            count2 += 1;
            continue;
        }

        // flag is false: recheck whether we can remove element[0]
        let increasing = nums[2] > nums[1];
        let mut flag = true;
        let mut prev_val = nums[1];
        for i in 2..nums.len() {
            if (nums[i] > prev_val) == increasing {
                let val = (nums[i] - prev_val).abs();
                if val < 1 || val > 3 {
                    flag = false;
                    break;
                } else {
                    prev_val = nums[i]
                }
            } else {
                flag = false;
                break;
            }
        }
        if flag {
            count2 += 1;
        }
        
    }
    println!("{} {}", count, count2);
}