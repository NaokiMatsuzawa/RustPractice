mod square;
mod rectangle;
pub trait Shape{
    fn new() -> Self where Self: Sized;
    fn area(&self) -> f64;
    fn get_name(&self) -> &str;
}

pub fn make_square() -> crate::shape::square::Square{
    square::Square::new()
}

pub fn make_rectangle() -> rectangle::Rectangle{
    rectangle::Rectangle::new()
}