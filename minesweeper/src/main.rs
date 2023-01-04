use std::io;

use rand::prelude::*;

fn main() {
    const HEIGHT : usize= 10;
    const WIDTH : usize = 20;
    const BOMB_NUM : usize= 10;

    let mut field = [[0; WIDTH];HEIGHT];
    let mut field_is_open = [[false; WIDTH];HEIGHT];

    let mut field_sub = [[0; WIDTH + 2]; HEIGHT + 2];

    let mut rng = rand::thread_rng();

    let mut nums : Vec<usize> = (0..HEIGHT*WIDTH).collect();
    nums.shuffle(&mut rng);
    for i in 0..BOMB_NUM{
        let index = nums[i];
        let y = index / WIDTH;
        let x = index % WIDTH;
        field[y][x] = 9;
        field_sub[y+1][x+1] = 9;
    }

    let field_vec = [(-1, -1),(-1, 0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            if field[y][x] == 9 {
                continue;
            }
            let y_sub : i32 = y as i32 +1;
            let x_sub : i32 = x as i32+1;
            let mut surround_bomb_num = 0;
            for vector in &field_vec{
                if field_sub[(y_sub+vector.0) as usize][(x_sub+vector.1) as usize] == 9 {
                    surround_bomb_num += 1;
                }
            }
            field[y][x] = surround_bomb_num;
        }
    }

    let stdin = io::stdin();

    loop{
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                if field_is_open[y][x]{
                    print!("{}", field[y][x]);
                }
                else{
                    print!(" ");
                }
            }
            println!();
        }
        println!("Input 2 Integers");
        //入力を受け付ける

        let input_x: usize;
        let input_y;

        let mut line = String::from("");
        stdin.read_line(&mut line).expect("ERROR");
        
        let splited_line : Vec<&str> = line.split_whitespace().collect();
        input_x = usize::from_str_radix(splited_line[0],10).unwrap();
        input_y = usize::from_str_radix(splited_line[1], 10).unwrap();
        

        if field_is_open[input_y][input_x]{
            continue;
        }

        field_is_open[input_y][input_x] = true;
        if field[input_y][input_x] == 9 {
            println!("BOMB!");
            println!("GAMEOVER");
            break;
        }
    }

    
}
