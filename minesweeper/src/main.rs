use rand::prelude::*;

fn main() {
    const HEIGHT : usize= 10;
    const WIDTH : usize = 20;
    const BOMB_NUM : usize= 10;

    let mut field = [[0; WIDTH];HEIGHT];
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

    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            print!("{}", field[y][x]);
        }
        println!();
    }

    
}
