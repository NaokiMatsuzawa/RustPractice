pub mod square;
pub mod rectangle;
pub trait Shape{
    fn new() -> Self where Self: Sized;
    fn area(&self) -> f64;
    fn get_name(&self) -> &str;
}