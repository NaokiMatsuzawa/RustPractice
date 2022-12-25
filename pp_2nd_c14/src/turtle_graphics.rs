pub struct Turtle{
    pos_x_ : i32,
    pos_y_ : i32,
    is_pen_down_ : bool,
    selected_pen_id_ : u32,
}

impl Turtle{
    pub fn new() -> Self{
        Turtle{
            pos_x_ : 0,
            pos_y_ : 0,
            is_pen_down_ : false,
            selected_pen_id_ : 0,
        }
    }

    pub fn input_command(&mut self, args : Vec<&str>){
        let arg1: i32 = if args.len() == 1{
            0
        }
        else{
            match i32::from_str_radix(args[1], 10){
                Ok(value) => value,
                Err(e) => 0,
            }
        };

        match args[0]{
            "P" => self.selectPen(arg1 as u32),
            "D" => self.penDown(),
            "U" => self.penUp(),
            "E" => self.moveX(arg1),
            "W" => self.moveX(-arg1),
            "S" => self.moveY(arg1),
            "N" => self.moveY(-arg1),
            _ => panic!(""),
        }
    }

    fn penUp(&mut self){
        self.is_pen_down_ = false;
    }

    fn penDown(&mut self){
        self.is_pen_down_ = true;
    }

    fn selectPen(&mut self, pen_id : u32){
        self.selected_pen_id_ = pen_id;
    }

    fn moveX(&mut self, dx : i32){
        self.pos_x_ += dx;
    }

    fn moveY(&mut self, dy : i32){
        self.pos_y_ += dy;
    }

    pub fn dispInfo(&self){
        println!("X : {}", self.pos_x_);
        println!("Y : {}", self.pos_y_);
        println!("PenID : {}", self.selected_pen_id_);
        println!("PenDown : {}", self.is_pen_down_);
    }

}