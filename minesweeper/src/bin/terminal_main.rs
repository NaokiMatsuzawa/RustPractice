use std::io;

use minesweeper::*;

fn main(){
    let width = 10;
    let height = 10;
    let bomb_num = 10;

    let mut game = game_borad_factory(width, height, bomb_num);

    let stdin = io::stdin();

    loop{
        print_game_board(&game);

        if game.is_clear() {
            println!("Congratulation!!!");
            break;
        }

        if game.is_gameover(){
            println!("GameOver");
            break;
        }

        let mut line = String::from("");
        stdin.read_line(&mut line).expect("ERROR");

        let splited_line : Vec<&str> = line.split_whitespace().collect();
        let input_x = usize::from_str_radix(splited_line[0],10).unwrap();
        let input_y = usize::from_str_radix(splited_line[1], 10).unwrap();
        
        game.request_open(input_x, input_y);
    }
}

fn print_game_board(board : &minesweeper::game_logic::FieldBoard){
    for y in 0..board.get_height(){
        for x in 0..board.get_width(){
            print!("{}", grid_id_to_str(board.get_grid_id(x, y)));
        }
        println!("");
    }
}

fn grid_id_to_str(id: game_logic::GridID) -> String{
    match id{
        game_logic::GridID::BOMB => "B".to_string(),
        game_logic::GridID::UNOPEN => "*".to_string(),
        game_logic::GridID::NUMBER(num) => u32::to_string(&num)
    }
}

