use scan_fmt::scan_fmt_some;
use std::fs;

fn main() {
    const X_SIZE: i32 = 101;
    const Y_SIZE: i32 = 103;

    let contents = fs::read_to_string("input.txt").expect("Read error");
    let lines = contents.lines();

    let mut p_x;
    let mut p_y;
    let mut v_x;
    let mut v_y;
    let mut p_x_final;
    let mut p_y_final;

    let mut quad = [0, 0, 0, 0];

    let mut matrix: [[u32; X_SIZE as usize]; Y_SIZE as usize];
    let mut bots = Vec::<(i32, i32, i32, i32)>::new();

    for line in lines {
        let (p_x_opt, p_y_opt, v_x_opt, v_y_opt) = scan_fmt_some!(line, "p={},{} v={},{}", i32, i32, i32, i32);
        // x in this case is "from left", y is "from top". x size 101, y size 103
        p_x = p_x_opt.unwrap();
        p_y = p_y_opt.unwrap();
        v_x = v_x_opt.unwrap();
        v_y = v_y_opt.unwrap();
        bots.push((p_x, p_y, v_x, v_y));

        p_x_final = (( p_x + v_x * 100 ) % X_SIZE + X_SIZE ) % X_SIZE;
        p_y_final = (( p_y + v_y * 100 ) % Y_SIZE + Y_SIZE ) % Y_SIZE;

        //println!("{} {}", p_x_final, p_y_final);

        if p_x_final < X_SIZE / 2 && p_y_final < Y_SIZE / 2 {
            quad[0] += 1;
        } else if p_x_final < X_SIZE / 2 && p_y_final > Y_SIZE / 2 {
            quad[1] += 1;
        } else if p_x_final > X_SIZE / 2 && p_y_final < Y_SIZE / 2 {
            quad[2] += 1;
        } else if p_x_final > X_SIZE / 2 && p_y_final > Y_SIZE / 2 {
            quad[3] += 1;
        }
    }

    println!("{}", quad[0] * quad[1] * quad[2] * quad[3]);

    // Part B: Flag 20 consecutives and manually inspect. t=6752s
    for t in 0..10000 {
        matrix = [[0; X_SIZE as usize]; Y_SIZE as usize];
        for bot in &bots {
            let (p_x, p_y, v_x, v_y) = bot;
            let p_x_new = ((( p_x + v_x * t ) % X_SIZE + X_SIZE ) % X_SIZE) as u32;
            let p_y_new = ((( p_y + v_y * t ) % Y_SIZE + Y_SIZE ) % Y_SIZE) as u32;
            matrix[p_y_new as usize][p_x_new as usize] += 1;
        }

        let mut flag = false;
        let mut streak = 0;

        for i in 0..Y_SIZE as usize {
            for j in 0..X_SIZE as usize {
                if matrix[i][j] > 0 {
                    streak += 1;
                    if streak >= 20 {
                        flag = true;
                        break;
                    }
                } else {
                    streak = 0;
                }
            }
            if flag {
                break;
            }
        }

        if flag {
            println!("\n\n\nt = {t}");
            for j in 0..Y_SIZE as usize {
                for i in 0..X_SIZE as usize {
                    print!("{}", if matrix[j][i] == 0 {'.'} else { char::from_digit(matrix[j][i], 10).unwrap() });
                }
                println!();
            }
        }
    }
}
