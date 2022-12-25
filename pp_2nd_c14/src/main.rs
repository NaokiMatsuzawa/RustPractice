mod turtle_graphics;

use std::io::{self, BufRead};

fn main()  -> io::Result<()>{
    let mut turtle = turtle_graphics::Turtle::new();
    let stdin =io::stdin();
    for line_result in stdin.lock().lines(){
        let line = line_result?;
        let words = line.split_whitespace().collect::<Vec<&str>>();
        turtle.input_command(words);
        turtle.dispInfo();
    }
    Ok(())
}
