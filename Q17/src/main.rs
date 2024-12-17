use scan_fmt::scan_fmt_some;
use std::fs;

fn get_combo(opcode: u64, val_a: u64, val_b: u64, val_c: u64) -> u64 {
    match opcode {
        0..=3 => opcode,
        4 => val_a,
        5 => val_b,
        6 => val_c,
        _ => panic!(),
    }
}

fn calc_output(a: u64) -> u64 {
    /*
        2,4: b = a % 8
        1,2: b ^= 2 (010)                          // revert second digit
        7,5: c = a / 2.pow(b)                      // (we only care about last three digits)
        1,7: b ^= 7 (111)                          // revert all digits
        4,4: b ^= c                                // revert digits where c's digit is 1
        0,3: a /= 8
        5,5: print b % 8                           // print out b's last three digits
        3,0: jump to first command if a is 0
    */
    let b = (a & 7) ^ 2;
    let c = (a >> b) & 7;
    (b ^ 7) ^ c
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");
    let lines: Vec<&str> = contents.lines().collect();

    let val_a_opt = scan_fmt_some!(lines[0], "Register A: {}", u64);
    let mut val_a = val_a_opt.unwrap();

    let val_b_opt = scan_fmt_some!(lines[1], "Register B: {}", u64);
    let mut val_b = val_b_opt.unwrap();

    let val_c_opt = scan_fmt_some!(lines[2], "Register C: {}", u64);
    let mut val_c = val_c_opt.unwrap();

    let commands = scan_fmt_some!(lines[4], "Program: {}", String);
    let commands: Vec<u64> = commands
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<u64>().unwrap_or(8))
        .filter(|t| *t <= 7)
        .collect();

    let mut output = Vec::<u64>::new();

    let mut i: usize = 0;
    while i < commands.len() {
        let instruction = commands[i];
        let opcode = commands[i + 1];

        match instruction {
            0 => val_a /= 2u64.pow(get_combo(opcode, val_a, val_b, val_c).try_into().unwrap()),
            1 => val_b ^= opcode,
            2 => val_b = get_combo(opcode, val_a, val_b, val_c) % 8,
            3 => {
                if val_a != 0 {
                    i = opcode as usize;
                    continue;
                }
            }
            4 => val_b ^= val_c,
            5 => output.push(get_combo(opcode, val_a, val_b, val_c) % 8),
            6 => {
                val_b = val_a / 2u64.pow(get_combo(opcode, val_a, val_b, val_c).try_into().unwrap())
            }
            7 => {
                val_c = val_a / 2u64.pow(get_combo(opcode, val_a, val_b, val_c).try_into().unwrap())
            }
            _ => panic!(),
        }
        i += 2;
    }
    let output_str = output
        .into_iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", output_str);

    // Part B
    // Analysis of Input: The input program is a loop, and the output of the digit is entirely dependent on A.
    // Each loop outputs one number. In total we want 16 numbers, thus 16 loops.
    // Each loop also modifies A by right-shifting three bits (>>=3), so we want an A between 8^16 and 8^17.
    // Given each output is entirely dependent on the remaining bits of A, we can iterate backwards from the last three bits
    //   which must produce the last digit of the desired output.
    // For a rigorous solution, we can record all possible As at the current stage and try adding 0-7 to each of them.
    // We then find out the minimum inside all possible As after stage 16.

    let mut target_a = Vec::<u64>::from([0]);

    for i in 0..16 {
        // Iterate from backwards
        let target_result = commands[commands.len() - 1 - i];
        let mut new_target_a = Vec::<u64>::new();

        for a in &target_a {
            for try_bits in 0..8 {
                // Try last three bits
                if calc_output((a << 3) + try_bits) == target_result {
                    new_target_a.push((a << 3) + try_bits);
                }
            }
        }
        target_a = new_target_a;
    }

    let result = target_a
        .iter()
        .min()
        .unwrap_or_else(|| panic!("No elements"));

    println!("{}", result);
}
