pub mod game_logic;

pub fn game_borad_factory(width : usize, height: usize, bomb_num: u32) -> game_logic::FieldBoard{
    game_logic::FieldBoard::new(width, height, bomb_num)
}